use dotenvy;
use serde::Deserialize;
use std::fmt;
use thiserror::Error;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub env: Env,
    pub database: Database,

    #[serde(skip_deserializing)]
    pub secrets: Secrets,
}

#[derive(Deserialize, Debug)]
pub struct Env {
    pub port: u16,
    pub ip_addr: String,
}

#[derive(Deserialize, Debug)]
pub struct Database {
    pub wal: bool,
}

#[derive(Default, Debug)]
pub struct Secrets {
    nasa_api_key: String,
    database_url: String,
}

#[derive(Debug, Error)]
pub enum ConfigErr {
    #[error("Failed to parse toml.")]
    Toml(#[from] toml::de::Error),

    #[error("Could not load .env file.")]
    DotEnvy(#[from] dotenvy::Error),

    #[error("Missing required env var: {name}")]
    Env {
        name: String,
        #[source]
        source: std::env::VarError,
    },
}

impl Config {
    // TODO: Change this api to be more explicit.
    /// parses given string with config.toml schema. Loads environment variables.
    pub fn from_string(string: String) -> Result<Config, ConfigErr> {
        let mut config = Self::parse_toml_string(string)?;
        config.secrets = Self::load_secrets()?;

        Ok(config)
    }

    fn parse_toml_string(string: String) -> Result<Config, ConfigErr> {
        let config = toml::from_str(&string)?;

        Ok(config)
    }

    fn load_secrets() -> Result<Secrets, ConfigErr> {
        dotenvy::dotenv()?;

        let nasa_api_key =
            std::env::var("NASA_API_KEY").map_err(|err| ConfigErr::env_err("NASA_API_KEY", err))?;

        let database_url =
            std::env::var("DATABASE_URL").map_err(|err| ConfigErr::env_err("DATABASE_URL", err))?;

        let secrets = Secrets {
            nasa_api_key,
            database_url,
        };

        Ok(secrets)
    }
}

impl ConfigErr {
    pub fn env_err(name: impl Into<String>, source: std::env::VarError) -> Self {
        Self::Env {
            name: name.into(),
            source,
        }
    }
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.ip_addr, self.port)
    }
}

impl fmt::Display for Secrets {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "NASA_API_KEY: {}\nDATABASE_URL: {}",
            self.nasa_api_key, self.database_url
        )
    }
}

impl fmt::Display for Database {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "wal?: {}", self.wal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_to_string() {
        let e = Env {
            port: 8080,
            ip_addr: "192.168.1.1".to_string(),
        };

        assert_eq!(e.to_string(), "192.168.1.1:8080".to_string());
    }
}
