use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbErr {}

#[derive(Debug)]
pub struct Database {
    path: String,
}

impl Database {
    pub fn from_config(c: &Config) -> Result<Database, DbErr> {}

    pub fn migrate() {}
}
