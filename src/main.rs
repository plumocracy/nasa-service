use color_eyre::{eyre::Context, Result};
use nasa_service::{config::Config, create_router, db::Database, state::AppState};
use std::fs;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    const CONFIG_PATH: &str = "config.toml";
    let conf_string = fs::read_to_string(CONFIG_PATH).context("Failed to read config.toml")?;

    let config = Config::from_string(conf_string).context("Could not parse config!")?;

    let db = Database::connect(&config).await?;
    db.migrate().await?;

    let bind_addr = config.env.to_string();

    let app_state = AppState { config, db };

    let app = create_router(app_state);

    println!("Starting server on {}", bind_addr);

    let listener = tokio::net::TcpListener::bind(bind_addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
