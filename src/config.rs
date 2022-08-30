use anyhow::Result;
use std::fmt;

#[derive(serde::Deserialize)]
pub struct Config {
    pub address: String,
    pub port: u16,
}

impl Config {
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let text = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&text)?)
    }

    pub fn addr(&self) -> (&str, u16) {
        (&self.address as &str, self.port)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            address: "127.0.0.1".into(),
            port: 8080,
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "address: {}\nport: {}", self.address, self.port)
    }
}
