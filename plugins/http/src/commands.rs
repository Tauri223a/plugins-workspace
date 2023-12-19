// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{collections::HashMap, time::Duration};

use http::{header, HeaderName, HeaderValue, Method, StatusCode};
use reqwest::{redirect::Policy, NoProxy, Proxy};
use serde::{Deserialize, Serialize};
use tauri::{command, AppHandle, Runtime};

use crate::{Error, FetchRequest, HttpExt, RequestId};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchResponse {
    status: u16,
    status_text: String,
    headers: Vec<(String, String)>,
    url: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientConfig {
    method: String,
    url: url::Url,
    headers: Vec<(String, String)>,
    data: Option<Vec<u8>>,
    connect_timeout: Option<u64>,
    max_redirections: Option<usize>,
    proxy: Option<ProxyConfig>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum ProxyConfig {
    All {
        all: UrlOrConfig,
    },
    HttpAndHttps {
        http: UrlOrConfig,
        https: UrlOrConfig,
    },
    Http {
        http: UrlOrConfig,
    },
    Https {
        https: UrlOrConfig,
    },
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum UrlOrConfig {
    Url(String),
    Config(DetailedProxyConfig),
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetailedProxyConfig {
    url: String,
    basic_auth: Option<BasicAuth>,
    no_proxy: Option<String>,
}

#[derive(Deserialize)]
pub struct BasicAuth {
    username: String,
    password: String,
}

fn attach_config(
    basic_auth: Option<BasicAuth>,
    no_proxy: Option<String>,
    mut proxy: Proxy,
) -> Proxy {
    if let Some(basic_auth) = basic_auth {
        proxy = proxy.basic_auth(&basic_auth.username, &basic_auth.password)
    }
    if let Some(no_proxy) = no_proxy {
        proxy = proxy.no_proxy(NoProxy::from_string(&no_proxy))
    }
    proxy
}

macro_rules! process_proxy {
    ($config:expr, $builder:expr, $proxy_fn:path) => {
        match $config {
            UrlOrConfig::Url(url) => {
                $builder = $builder.proxy($proxy_fn(&url)?);
            }
            UrlOrConfig::Config(config) => {
                $builder = $builder.proxy(attach_config(
                    config.basic_auth,
                    config.no_proxy,
                    $proxy_fn(config.url)?,
                ));
            }
        }
    };
}

fn attach_proxy(
    config: ProxyConfig,
    mut builder: reqwest::ClientBuilder,
) -> crate::Result<reqwest::ClientBuilder> {
    match config {
        ProxyConfig::All { all } => process_proxy!(all, builder, Proxy::all),
        ProxyConfig::Http { http } => process_proxy!(http, builder, Proxy::http),
        ProxyConfig::Https { https } => process_proxy!(https, builder, Proxy::https),
        ProxyConfig::HttpAndHttps { http, https } => {
            process_proxy!(http, builder, Proxy::http);
            process_proxy!(https, builder, Proxy::https);
        }
    }

    Ok(builder)
}

#[command]
pub async fn fetch<R: Runtime>(
    app: AppHandle<R>,
    client_config: ClientConfig,
) -> crate::Result<RequestId> {
    let ClientConfig {
        method,
        url,
        headers,
        data,
        connect_timeout,
        max_redirections,
        proxy,
    } = client_config;

    let scheme = url.scheme();
    let method = Method::from_bytes(method.as_bytes())?;
    let headers: HashMap<String, String> = HashMap::from_iter(headers);

    match scheme {
        "http" | "https" => {
            if app.http().scope.is_allowed(&url) {
                let mut builder = reqwest::ClientBuilder::new();

                if let Some(timeout) = connect_timeout {
                    builder = builder.connect_timeout(Duration::from_millis(timeout));
                }

                if let Some(max_redirections) = max_redirections {
                    builder = builder.redirect(if max_redirections == 0 {
                        Policy::none()
                    } else {
                        Policy::limited(max_redirections)
                    });
                }

                if let Some(proxy_config) = proxy {
                    builder = attach_proxy(proxy_config, builder)?;
                }

                let mut request = builder.build()?.request(method.clone(), url);

                for (key, value) in &headers {
                    let name = HeaderName::from_bytes(key.as_bytes())?;
                    let v = HeaderValue::from_bytes(value.as_bytes())?;
                    if !matches!(name, header::HOST | header::CONTENT_LENGTH) {
                        request = request.header(name, v);
                    }
                }

                // POST and PUT requests should always have a 0 length content-length,
                // if there is no body. https://fetch.spec.whatwg.org/#http-network-or-cache-fetch
                if data.is_none() && matches!(method, Method::POST | Method::PUT) {
                    request = request.header(header::CONTENT_LENGTH, HeaderValue::from(0));
                }

                if headers.contains_key(header::RANGE.as_str()) {
                    // https://fetch.spec.whatwg.org/#http-network-or-cache-fetch step 18
                    // If httpRequest’s header list contains `Range`, then append (`Accept-Encoding`, `identity`)
                    request = request.header(
                        header::ACCEPT_ENCODING,
                        HeaderValue::from_static("identity"),
                    );
                }

                if !headers.contains_key(header::USER_AGENT.as_str()) {
                    request = request.header(header::USER_AGENT, HeaderValue::from_static("tauri"));
                }

                if let Some(data) = data {
                    request = request.body(data);
                }

                let http_state = app.http();
                let rid = http_state.next_id();
                let fut = async move { Ok(request.send().await.map_err(Into::into)) };
                let mut request_table = http_state.requests.lock().await;
                request_table.insert(rid, FetchRequest::new(Box::pin(fut)));

                Ok(rid)
            } else {
                Err(Error::UrlNotAllowed(url))
            }
        }
        "data" => {
            let data_url =
                data_url::DataUrl::process(url.as_str()).map_err(|_| Error::DataUrlError)?;
            let (body, _) = data_url
                .decode_to_vec()
                .map_err(|_| Error::DataUrlDecodeError)?;

            let response = http::Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, data_url.mime_type().to_string())
                .body(reqwest::Body::from(body))?;

            let http_state = app.http();
            let rid = http_state.next_id();
            let fut = async move { Ok(Ok(reqwest::Response::from(response))) };
            let mut request_table = http_state.requests.lock().await;
            request_table.insert(rid, FetchRequest::new(Box::pin(fut)));
            Ok(rid)
        }
        _ => Err(Error::SchemeNotSupport(scheme.to_string())),
    }
}

#[command]
pub async fn fetch_cancel<R: Runtime>(app: AppHandle<R>, rid: RequestId) -> crate::Result<()> {
    let mut request_table = app.http().requests.lock().await;
    let req = request_table
        .get_mut(&rid)
        .ok_or(Error::InvalidRequestId(rid))?;
    *req = FetchRequest::new(Box::pin(async { Err(Error::RequestCanceled) }));
    Ok(())
}

#[command]
pub async fn fetch_send<R: Runtime>(
    app: AppHandle<R>,
    rid: RequestId,
) -> crate::Result<FetchResponse> {
    let mut request_table = app.http().requests.lock().await;
    let req = request_table
        .remove(&rid)
        .ok_or(Error::InvalidRequestId(rid))?;

    let res = match req.0.lock().await.as_mut().await {
        Ok(Ok(res)) => res,
        Ok(Err(e)) | Err(e) => return Err(e),
    };

    let status = res.status();
    let url = res.url().to_string();
    let mut headers = Vec::new();
    for (key, val) in res.headers().iter() {
        headers.push((
            key.as_str().into(),
            String::from_utf8(val.as_bytes().to_vec())?,
        ));
    }

    app.http().responses.lock().await.insert(rid, res);

    Ok(FetchResponse {
        status: status.as_u16(),
        status_text: status.canonical_reason().unwrap_or_default().to_string(),
        headers,
        url,
    })
}

#[command]
pub(crate) async fn fetch_read_body<R: Runtime>(
    app: AppHandle<R>,
    rid: RequestId,
) -> crate::Result<tauri::ipc::Response> {
    let mut response_table = app.http().responses.lock().await;
    let res = response_table
        .remove(&rid)
        .ok_or(Error::InvalidRequestId(rid))?;

    Ok(tauri::ipc::Response::new(res.bytes().await?.to_vec()))
}
