if("__TAURI__"in window){var __TAURI_PLUGIN_HTTP__=function(e){"use strict";async function t(e,t={},r){return window.__TAURI_INTERNALS__.invoke(e,t,r)}return"function"==typeof SuppressedError&&SuppressedError,e.fetch=async function(e,r){const n=r?.maxRedirections,a=r?.connectTimeout,s=r?.proxy;r&&(delete r.maxRedirections,delete r.connectTimeout,delete r.proxy);const i=r?.signal,o=r?.headers?r.headers instanceof Headers?r.headers:new Headers(r.headers):new Headers,d=new Request(e,r),c=await d.arrayBuffer(),u=0!==c.byteLength?Array.from(new Uint8Array(c)):null;for(let[e,t]of d.headers)o.get(e)||o.set(e,t);const _=(o instanceof Headers?Array.from(o.entries()):Array.isArray(o)?o:Object.entries(o)).map((([e,t])=>[e,"string"==typeof t?t:t.toString()])),f=await t("plugin:http|fetch",{clientConfig:{method:d.method,url:d.url,headers:_,data:u,maxRedirections:n,connectTimeout:a,proxy:s}});i?.addEventListener("abort",(()=>{t("plugin:http|fetch_cancel",{rid:f})}));const{status:h,statusText:l,url:p,headers:y,rid:w}=await t("plugin:http|fetch_send",{rid:f}),T=await t("plugin:http|fetch_read_body",{rid:w}),A=new Response(T instanceof ArrayBuffer&&0!==T.byteLength?T:T instanceof Array&&T.length>0?new Uint8Array(T):null,{headers:y,status:h,statusText:l});return Object.defineProperty(A,"url",{value:p}),A},e}({});Object.defineProperty(window.__TAURI__,"http",{value:__TAURI_PLUGIN_HTTP__})}
