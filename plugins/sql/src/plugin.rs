// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use futures_core::future::BoxFuture;
use serde::{ser::Serializer, Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[cfg(feature = "sqlite")]
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteLockingMode, SqliteSynchronous};
#[cfg(feature = "sqlite")]
use std::str::FromStr;

use sqlx::{
    error::BoxDynError,
    migrate::{Migration as SqlxMigration, MigrationSource, MigrationType, Migrator},
    Column, Pool, Row,
};

#[cfg(not(feature = "sqlite"))]
use sqlx::migrate::MigrateDatabase;

use tauri::{
    command,
    plugin::{Builder as PluginBuilder, TauriPlugin},
    AppHandle, Manager, RunEvent, Runtime, State,
};
use tokio::sync::Mutex;

use indexmap::IndexMap;
use std::collections::HashMap;

#[cfg(feature = "sqlite")]
use std::{fs::create_dir_all, path::PathBuf};

#[cfg(feature = "sqlite")]
type Db = sqlx::sqlite::Sqlite;
#[cfg(feature = "mysql")]
type Db = sqlx::mysql::MySql;
#[cfg(feature = "postgres")]
type Db = sqlx::postgres::Postgres;

#[cfg(feature = "sqlite")]
type LastInsertId = i64;
#[cfg(not(feature = "sqlite"))]
type LastInsertId = u64;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Sql(#[from] sqlx::Error),
    #[error(transparent)]
    Migration(#[from] sqlx::migrate::MigrateError),
    #[error("database {0} not loaded")]
    DatabaseNotLoaded(String),
    #[error("unsupported datatype: {0}")]
    UnsupportedDatatype(String),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

type Result<T> = std::result::Result<T, Error>;

#[cfg(feature = "sqlite")]
/// Resolves the App's **file path** from the `AppHandle` context
/// object
fn app_path<R: Runtime>(app: &AppHandle<R>) -> PathBuf {
    app.path().app_config_dir().expect("No App path was found!")
}

#[cfg(feature = "sqlite")]
/// Maps the user supplied DB connection string to a connection string
/// with a fully qualified file path to the App's designed "app_path"
fn path_mapper(mut app_path: PathBuf, connection_string: &str) -> String {
    app_path.push(
        connection_string
            .split_once(':')
            .expect("Couldn't parse the connection string for DB!")
            .1,
    );

    format!(
        "sqlite:{}",
        app_path
            .to_str()
            .expect("Problem creating fully qualified path to Database file!")
    )
}

#[derive(Default)]
struct DbInstances(Mutex<HashMap<String, Pool<Db>>>);

struct Migrations(Mutex<HashMap<String, MigrationList>>);

#[cfg(feature = "sqlite")]
#[derive(Clone, Deserialize)]
pub struct SqliteConfig {
    pub key: &'static str,     // Database key
    pub cipher_page_size: i32, // Page size of encrypted database. Default for SQLCipher v4 is 4096.
    pub cipher_plaintext_header_size: i32,
    pub kdf_iter: i32, // Number of iterations used in PBKDF2 key derivation. Default for SQLCipher v4 is 256000
    pub cipher_kdf_algorithm: &'static str, // Define KDF algorithm to be used. Default for SQLCipher v4 is PBKDF2_HMAC_SHA512.
    pub cipher_hmac_algorithm: &'static str, // Choose algorithm used for HMAC. Default for SQLCipher v4 is HMAC_SHA512.
    pub cipher_salt: Option<&'static str>, // Allows to provide salt manually. By default SQLCipher sets salt automatically, use only in conjunction with 'cipher_plaintext_header_size' pragma
    pub cipher_compatibility: Option<i32>, // 1, 2, 3, 4
    pub journal_mode: &'static str,        // DELETE | TRUNCATE | PERSIST | MEMORY | WAL | OFF
    pub foreign_keys: bool,
    pub synchronous: &'static str,  // EXTRA | FULL | NORMAL |  OFF
    pub locking_mode: &'static str, // NORMAL | EXCLUSIVE
    pub read_only: bool,            // Open database in read-only mode
}

#[cfg(feature = "sqlite")]
impl Default for SqliteConfig {
    fn default() -> Self {
        SqliteConfig {
            key: "",
            cipher_page_size: 4096,
            cipher_plaintext_header_size: 0,
            kdf_iter: 256000,
            cipher_salt: None,
            cipher_compatibility: None,
            cipher_kdf_algorithm: "PBKDF2_HMAC_SHA512",
            cipher_hmac_algorithm: "HMAC_SHA512",
            journal_mode: "DELETE",
            foreign_keys: true,
            synchronous: "FULL",
            locking_mode: "NORMAL",
            read_only: false,
        }
    }
}

#[cfg(feature = "sqlite")]
pub fn sqlite_config_to_options(db: &str, config: SqliteConfig) -> SqliteConnectOptions {
    let is_in_memory = db.contains(":memory") || db.contains("mode=memory");
    let mut options = if is_in_memory {
        SqliteConnectOptions::from_str("sqlite::memory:").unwrap()
    } else {
        SqliteConnectOptions::from_str(db).unwrap()
    };
    if config.key != "" {
        options = options
            .pragma("key", config.key)
            .pragma("cipher_kdf_algorithm", config.cipher_kdf_algorithm)
            .pragma(
                "cipher_plaintext_header_size",
                config.cipher_plaintext_header_size.to_string(),
            )
            .pragma("cipher_page_size", config.cipher_page_size.to_string())
            .pragma("kdf_iter", config.kdf_iter.to_string())
            .pragma("cipher_hmac_algorithm", config.cipher_hmac_algorithm);
        if let Some(cipher_salt) = config.cipher_salt {
            options = options.pragma("cipher_hmac_algorithm", cipher_salt.to_string())
        };
        if let Some(cipher_compatibility) = config.cipher_compatibility {
            options = options.pragma("cipher_compatibility", cipher_compatibility.to_string())
        };
    }
    options
        .foreign_keys(config.foreign_keys)
        .journal_mode(
            SqliteJournalMode::from_str(config.journal_mode).unwrap_or(SqliteJournalMode::Delete),
        )
        .synchronous(
            SqliteSynchronous::from_str(config.synchronous).unwrap_or(SqliteSynchronous::Full),
        )
        .locking_mode(
            SqliteLockingMode::from_str(config.locking_mode).unwrap_or(SqliteLockingMode::Normal),
        )
        .create_if_missing(true)
}

#[cfg(feature = "sqlite")]
struct SqlLiteOptionStore(Mutex<HashMap<String, SqliteConfig>>);

#[derive(Default, Clone, Deserialize)]
pub struct PluginConfig {
    #[serde(default)]
    preload: Vec<String>,
}

#[derive(Debug)]
pub enum MigrationKind {
    Up,
    Down,
}

impl From<MigrationKind> for MigrationType {
    fn from(kind: MigrationKind) -> Self {
        match kind {
            MigrationKind::Up => Self::ReversibleUp,
            MigrationKind::Down => Self::ReversibleDown,
        }
    }
}

/// A migration definition.
#[derive(Debug)]
pub struct Migration {
    pub version: i64,
    pub description: &'static str,
    pub sql: &'static str,
    pub kind: MigrationKind,
}

#[derive(Debug)]
struct MigrationList(Vec<Migration>);

impl MigrationSource<'static> for MigrationList {
    fn resolve(self) -> BoxFuture<'static, std::result::Result<Vec<SqlxMigration>, BoxDynError>> {
        Box::pin(async move {
            let mut migrations = Vec::new();
            for migration in self.0 {
                if matches!(migration.kind, MigrationKind::Up) {
                    migrations.push(SqlxMigration::new(
                        migration.version,
                        migration.description.into(),
                        migration.kind.into(),
                        migration.sql.into(),
                    ));
                }
            }
            Ok(migrations)
        })
    }
}

#[cfg(not(feature = "sqlite"))]
#[command]
async fn load<R: Runtime>(
    #[allow(unused_variables)] app: AppHandle<R>,
    db_instances: State<'_, DbInstances>,
    migrations: State<'_, Migrations>,
    db: String,
) -> Result<String> {
    let fqdb = db.clone();
    let pool = Pool::connect(&fqdb).await?;
    if let Some(migrations) = migrations.0.lock().await.remove(&db) {
        let migrator = Migrator::new(migrations).await?;
        migrator.run(&pool).await?;
    }
    db_instances.0.lock().await.insert(db.clone(), pool);
    Ok(db)
}

#[cfg(feature = "sqlite")]
#[command]
async fn load<R: Runtime>(
    #[allow(unused_variables)] app: AppHandle<R>,
    db_instances: State<'_, DbInstances>,
    migrations: State<'_, Migrations>,
    sqlite_options_store: State<'_, SqlLiteOptionStore>,
    db: String,
) -> Result<String> {
    let fqdb = {
        create_dir_all(app_path(&app)).expect("Problem creating App directory!");
        path_mapper(app_path(&app), &db)
    };
    let sqlite_options = if let Some(options) = sqlite_options_store.0.lock().await.remove(&db) {
        options
    } else {
        SqliteConfig::default()
    };
    let pool = Pool::connect_with(sqlite_config_to_options(&fqdb, sqlite_options)).await?;
    if let Some(migrations) = migrations.0.lock().await.remove(&db) {
        let migrator = Migrator::new(migrations).await?;
        migrator.run(&pool).await?;
    }

    db_instances.0.lock().await.insert(db.clone(), pool);
    Ok(db)
}

/// Allows the database connection(s) to be closed; if no database
/// name is passed in then _all_ database connection pools will be
/// shut down.
#[command]
async fn close(db_instances: State<'_, DbInstances>, db: Option<String>) -> Result<bool> {
    let mut instances = db_instances.0.lock().await;

    let pools = if let Some(db) = db {
        vec![db]
    } else {
        instances.keys().cloned().collect()
    };

    for pool in pools {
        let db = instances
            .get_mut(&pool) //
            .ok_or(Error::DatabaseNotLoaded(pool))?;
        db.close().await;
    }

    Ok(true)
}

/// Execute a command against the database
#[command]
async fn execute(
    db_instances: State<'_, DbInstances>,
    db: String,
    query: String,
    values: Vec<JsonValue>,
) -> Result<(u64, LastInsertId)> {
    let mut instances = db_instances.0.lock().await;

    let db = instances.get_mut(&db).ok_or(Error::DatabaseNotLoaded(db))?;
    let mut query = sqlx::query(&query);
    for value in values {
        if value.is_null() {
            query = query.bind(None::<JsonValue>);
        } else if value.is_string() {
            query = query.bind(value.as_str().unwrap().to_owned())
        } else if value.is_number() {
            query = query.bind(value.as_f64().unwrap_or_default())
        } else {
            query = query.bind(value);
        }
    }
    let result = query.execute(&*db).await?;
    #[cfg(feature = "sqlite")]
    let r = Ok((result.rows_affected(), result.last_insert_rowid()));
    #[cfg(feature = "mysql")]
    let r = Ok((result.rows_affected(), result.last_insert_id()));
    #[cfg(feature = "postgres")]
    let r = Ok((result.rows_affected(), 0));
    r
}

#[command]
async fn select(
    db_instances: State<'_, DbInstances>,
    db: String,
    query: String,
    values: Vec<JsonValue>,
) -> Result<Vec<IndexMap<String, JsonValue>>> {
    let mut instances = db_instances.0.lock().await;
    let db = instances.get_mut(&db).ok_or(Error::DatabaseNotLoaded(db))?;
    let mut query = sqlx::query(&query);
    for value in values {
        if value.is_null() {
            query = query.bind(None::<JsonValue>);
        } else if value.is_string() {
            query = query.bind(value.as_str().unwrap().to_owned())
        } else if value.is_number() {
            query = query.bind(value.as_f64().unwrap_or_default())
        } else {
            query = query.bind(value);
        }
    }
    let rows = query.fetch_all(&*db).await?;
    let mut values = Vec::new();
    for row in rows {
        let mut value = IndexMap::default();
        for (i, column) in row.columns().iter().enumerate() {
            let v = row.try_get_raw(i)?;

            let v = crate::decode::to_json(v)?;

            value.insert(column.name().to_string(), v);
        }

        values.push(value);
    }

    Ok(values)
}

/// Tauri SQL plugin builder.
#[derive(Default)]
pub struct Builder {
    migrations: Option<HashMap<String, MigrationList>>,
    #[cfg(feature = "sqlite")]
    sqlite_options: Option<HashMap<String, SqliteConfig>>,
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add migrations to a database.
    #[must_use]
    pub fn add_migrations(mut self, db_url: &str, migrations: Vec<Migration>) -> Self {
        self.migrations
            .get_or_insert(Default::default())
            .insert(db_url.to_string(), MigrationList(migrations));
        self
    }

    #[cfg(feature = "sqlite")]
    /// Add sqlite options to a database.
    #[must_use]
    pub fn add_sqlite_options(mut self, db_url: &str, options: SqliteConfig) -> Self {
        self.sqlite_options
            .get_or_insert(Default::default())
            .insert(db_url.to_string(), options);
        self
    }

    pub fn build<R: Runtime>(mut self) -> TauriPlugin<R, Option<PluginConfig>> {
        PluginBuilder::<R, Option<PluginConfig>>::new("sql")
            .invoke_handler(tauri::generate_handler![load, execute, select, close])
            .setup(|app, api| {
                let config = api.config().clone().unwrap_or_default();

                #[cfg(feature = "sqlite")]
                create_dir_all(app_path(app)).expect("problems creating App directory!");

                tauri::async_runtime::block_on(async move {
                    let instances = DbInstances::default();
                    let mut lock = instances.0.lock().await;
                    for db in config.preload {
                        #[cfg(feature = "sqlite")]
                        let sqlite_options = if let Some(options) =
                            self.sqlite_options.as_mut().unwrap().remove(&db)
                        {
                            options
                        } else {
                            SqliteConfig::default()
                        };

                        #[cfg(feature = "sqlite")]
                        let fqdb = path_mapper(app_path(app), &db);

                        #[cfg(not(feature = "sqlite"))]
                        let fqdb = {
                            let path_db = db.clone();
                            if !Db::database_exists(&path_db).await.unwrap_or(false) {
                                Db::create_database(&path_db).await?;
                            }
                            path_db
                        };

                        #[cfg(not(feature = "sqlite"))]
                        let pool = Pool::connect(&fqdb).await?;

                        #[cfg(feature = "sqlite")]
                        let pool =
                            Pool::connect_with(sqlite_config_to_options(&fqdb, sqlite_options))
                                .await?;

                        if let Some(migrations) = self.migrations.as_mut().unwrap().remove(&db) {
                            let migrator = Migrator::new(migrations).await?;
                            migrator.run(&pool).await?;
                        }
                        lock.insert(db, pool);
                    }
                    drop(lock);
                    app.manage(instances);
                    app.manage(Migrations(Mutex::new(
                        self.migrations.take().unwrap_or_default(),
                    )));
                    #[cfg(feature = "sqlite")]
                    app.manage(SqlLiteOptionStore(Mutex::new(
                        self.sqlite_options.take().unwrap_or_default(),
                    )));
                    Ok(())
                })
            })
            .on_event(|app, event| {
                if let RunEvent::Exit = event {
                    tauri::async_runtime::block_on(async move {
                        let instances = &*app.state::<DbInstances>();
                        let instances = instances.0.lock().await;
                        for value in instances.values() {
                            value.close().await;
                        }
                    });
                }
            })
            .build()
    }
}
