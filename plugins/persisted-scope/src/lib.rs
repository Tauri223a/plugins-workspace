// Copyright 2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use aho_corasick::AhoCorasick;
use serde::{Deserialize, Serialize};
use tauri::{
    plugin::{Builder, TauriPlugin},
    AppHandle, FsScope, FsScopeEvent, Manager, Runtime,
};

use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
};

const SCOPE_STATE_FILENAME: &str = ".persisted-scope";

// Most of these patterns are just added to try to fix broken files in the wild.
// After a while we can hopefully reduce it to something like [r"[?]", r"[*]", r"\\?\\\?\"]
const PATTERNS: &[&str] = &[
    r"[[]",
    r"[]]",
    r"[?]",
    r"[*]",
    r"\?\?",
    r"\\?\\?\",
    r"\\?\\\?\",
];
const REPLACE_WITH: &[&str] = &[r"[", r"]", r"?", r"*", r"\?", r"\\?\", r"\\?\"];

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Tauri(#[from] tauri::Error),
    #[error(transparent)]
    TauriApi(#[from] tauri::api::Error),
    #[error(transparent)]
    Bincode(#[from] Box<bincode::ErrorKind>),
}

#[derive(Debug, Default, Deserialize, Serialize, Eq, PartialEq, Hash)]
enum TargetType {
    #[default]
    File,
    Directory,
    RecursiveDirectory,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct Scope {
    allowed_paths: Vec<String>,
    forbidden_patterns: Vec<String>,
}

fn fix_pattern(ac: &AhoCorasick, s: &str) -> String {
    let s = ac.replace_all(s, REPLACE_WITH);

    if ac.find(&s).is_some() {
        return fix_pattern(ac, &s);
    }

    s
}

const RESURSIVE_DIRECTORY_SUFFIX: &str = "**";
const DIRECTORY_SUFFIX: &str = "*";

fn detect_scope_type(scope_state_path: &str) -> TargetType {
    if scope_state_path.ends_with(RESURSIVE_DIRECTORY_SUFFIX) {
        TargetType::RecursiveDirectory
    } else if scope_state_path.ends_with(DIRECTORY_SUFFIX) {
        TargetType::Directory
    } else {
        TargetType::File
    }
}

fn fix_directory(path_str: &str) -> &Path {
    let mut path = Path::new(path_str);

    if path.ends_with(DIRECTORY_SUFFIX) || path.ends_with(RESURSIVE_DIRECTORY_SUFFIX) {
        path = match path.parent() {
            Some(value) => value,
            None => return path,
        };
    }

    path
}

fn allow_path(scope: &FsScope, path: &str) {
    let target_type = detect_scope_type(path);

    match target_type {
        TargetType::File => {
            let _ = scope.allow_file(path);
        }
        TargetType::Directory => {
            // We remove the '*' at the end of it, else it will be escaped by the pattern.
            let _ = scope.allow_directory(fix_directory(path), false);
        }
        TargetType::RecursiveDirectory => {
            // We remove the '**' at the end of it, else it will be escaped by the pattern.
            let _ = scope.allow_directory(fix_directory(path), true);
        }
    }
}

fn forbid_path(scope: &FsScope, path: &str) {
    let target_type = detect_scope_type(path);

    match target_type {
        TargetType::File => {
            let _ = scope.forbid_file(path);
        }
        TargetType::Directory => {
            let _ = scope.forbid_directory(fix_directory(path), false);
        }
        TargetType::RecursiveDirectory => {
            let _ = scope.forbid_directory(fix_directory(path), true);
        }
    }
}

fn save_scopes<R: Runtime>(app: &AppHandle<R>, app_dir: &Path, scope_state_path: &Path) {
    let fs_scope = app.fs_scope();

    let scope = Scope {
        allowed_paths: fs_scope
            .allowed_patterns()
            .into_iter()
            .map(|p| p.to_string())
            .collect(),
        forbidden_patterns: fs_scope
            .forbidden_patterns()
            .into_iter()
            .map(|p| p.to_string())
            .collect(),
    };

    let _ = create_dir_all(app_dir)
        .and_then(|_| File::create(scope_state_path))
        .map_err(Error::Io)
        .and_then(|mut f| {
            f.write_all(&bincode::serialize(&scope).map_err(Error::from)?)
                .map_err(Into::into)
        });
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("persisted-scope")
        .setup(|app| {
            let fs_scope = app.fs_scope();
            #[cfg(feature = "protocol-asset")]
            let asset_protocol_scope = app.asset_protocol_scope();
            let app = app.clone();
            let app_dir = app.path_resolver().app_data_dir();

            if let Some(app_dir) = app_dir {
                let scope_state_path = app_dir.join(SCOPE_STATE_FILENAME);

                let _ = fs_scope.forbid_file(&scope_state_path);
                #[cfg(feature = "protocol-asset")]
                let _ = asset_protocol_scope.forbid_file(&scope_state_path);

                // We're trying to fix broken .persisted-scope files seamlessly, so we'll be running this on the values read on the saved file.
                // We will still save some semi-broken values because the scope events are quite spammy and we don't want to reduce runtime performance any further.
                let ac = AhoCorasick::new(PATTERNS).unwrap(/* This should be impossible to fail since we're using a small static input */);

                if scope_state_path.exists() {
                    let scope: Scope = tauri::api::file::read_binary(&scope_state_path)
                        .map_err(Error::from)
                        .and_then(|scope| bincode::deserialize(&scope).map_err(Into::into))
                        .unwrap_or_default();

                    for allowed in &scope.allowed_paths {
                        let allowed = fix_pattern(&ac, allowed);
                        allow_path(&fs_scope, &allowed);
                        #[cfg(feature = "protocol-asset")]
                        allow_path(&asset_protocol_scope, &allowed);
                    }
                    for forbidden in &scope.forbidden_patterns {
                        let forbidden = fix_pattern(&ac, forbidden);
                        forbid_path(&fs_scope, &forbidden);
                        #[cfg(feature = "protocol-asset")]
                        forbid_path(&asset_protocol_scope, &forbidden);
                    }

                    // Manually save the fixed scopes to disk once.
                    // This is needed to fix broken .peristed-scope files in case the app doesn't update the scope itself.
                    save_scopes(&app, &app_dir, &scope_state_path);
                }

                fs_scope.listen(move |event| {
                    if let FsScopeEvent::PathAllowed(_) = event {
                        save_scopes(&app, &app_dir, &scope_state_path);
                    }
                });
            }
            Ok(())
        })
        .build()
}
