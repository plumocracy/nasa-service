use std::{str::FromStr, time::Duration};

use crate::config::Config;

use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbErr {
    #[error("sqlx error")]
    Sqlx(#[from] sqlx::Error),

    #[error("wal error: {0}")]
    WalNotEnabled(String),

    #[error("Migration error!")]
    Migrate(#[from] sqlx::migrate::MigrateError),
}

#[derive(Debug, Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn connect(c: &Config) -> Result<Self, DbErr> {
        let mut conn_opts = SqliteConnectOptions::from_str(c.secrets.database_url())?;

        if let Some(ms) = c.db.busy_timeout_ms {
            conn_opts = conn_opts.busy_timeout(Duration::from_millis(ms))
        }

        let mut pool_opts = SqlitePoolOptions::new().max_connections(c.db.max_connections);

        if c.db.foreign_keys {
            pool_opts = pool_opts.after_connect(|conn, _meta| {
                Box::pin(async move {
                    sqlx::query("PRAGMA foreign_keys = ON;")
                        .execute(&mut *conn)
                        .await?;

                    Ok(())
                })
            });
        }

        let pool = pool_opts.connect_with(conn_opts).await?;

        if c.db.wal {
            let row: (String,) = sqlx::query_as("PRAGMA journal_mode = WAL;")
                .fetch_one(&pool)
                .await?;

            if row.0.to_lowercase() != "wal" {
                return Err(DbErr::WalNotEnabled(row.0));
            }
        }

        Ok(Self { pool })
    }

    pub async fn migrate(&self) -> Result<(), DbErr> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        Ok(())
    }
}
