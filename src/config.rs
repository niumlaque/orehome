use anyhow::{bail, Result};
use std::fmt;

#[derive(serde::Deserialize)]
pub struct Config {
    pub address: String,
    pub port: u16,
    pub log: String,
}

impl Config {
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let text = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&text)?)
    }

    pub fn addr(&self) -> (&str, u16) {
        (&self.address as &str, self.port)
    }

    pub fn level(&self) -> Result<tracing::Level> {
        let level = self.log.to_uppercase();
        match &level as &str {
            "TRACE" => Ok(tracing::Level::TRACE),
            "DEBUG" => Ok(tracing::Level::DEBUG),
            "INFO" => Ok(tracing::Level::INFO),
            "WARN" => Ok(tracing::Level::WARN),
            "WARNING" => Ok(tracing::Level::WARN),
            "ERROR" => Ok(tracing::Level::ERROR),
            _ => bail!("Could not convert \"{}\" to log level", level),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            address: "127.0.0.1".into(),
            port: 8080,
            log: "INFO".into(),
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "address: {}, port: {}, log: {}",
            self.address, self.port, self.log
        )
    }
}
