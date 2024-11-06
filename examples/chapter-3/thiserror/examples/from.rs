use thiserror::Error;
use std::fs::File;
use std::io::Read;
use reqwest::Url;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Configuration error")]
    ConfigError(#[from] ConfigError),
    #[error("Network error")]
    NetworkError(#[from] reqwest::Error),
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file `{0}`: {1}")]
    ReadError(String, #[source] std::io::Error),
    #[error("Invalid URL `{0}` in config")]
    InvalidUrl(String),
}

fn load_config(filename: &str) -> Result<Url, ConfigError> {
    let mut file = File::open(filename)
        .map_err(|e| ConfigError::ReadError(filename.to_string(), e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| ConfigError::ReadError(filename.to_string(), e))?;

    let url = contents.trim();
    Url::parse(url).map_err(|_| ConfigError::InvalidUrl(url.to_string()))
}

fn main() -> Result<(), AppError> {
    let url = load_config("config.txt")?;
    let response = reqwest::blocking::get(url)?;
    println!("Response: {:?}", response);
    Ok(())
}
