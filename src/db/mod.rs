use std::str::FromStr;

use super::Config;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbErr {
    #[error("sqlx error")]
    Sqlx(#[from] sqlx::Error),
}

#[derive(Debug)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn connect(c: &Config) -> Result<Self, DbErr> {
        let opts =
            SqliteConnectOptions::from_str(c.secrets.database_url())?.create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(opts)
            .await?;

        Ok(Database { pool })
    }

    pub fn migrate() {}
}
