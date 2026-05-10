use color_eyre::{eyre::Context, Result};
use std::fs;

use crate::config::Config;
mod config;

use crate::db::Database;
mod db;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    const CONFIG_PATH: &str = "config.toml";
    let conf_string = fs::read_to_string(CONFIG_PATH).context("Failed to read config.toml")?;

    let conf = Config::from_string(conf_string).context("Could not parse config!")?;
    println!("{}", conf.secrets.to_string());

    let db = Database::connect(&conf).await?;

    Ok(())
}
