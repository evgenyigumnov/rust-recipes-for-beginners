use anyhow::{Context, Result};
use std::fs::File;
use std::io::Read;

fn main() -> Result<()> {
    let config = load_config("config.toml")?;
    let data = fetch_data(&config.api_url)?;
    process_data(data)?;
    Ok(())
}

fn load_config(path: &str) -> Result<Config> {
    let mut file = File::open(path)
        .with_context(|| format!("Could not open config file '{}'", path))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .context("Could not read config file")?;
    let config: Config = toml::from_str(&contents)
        .context("Could not parse config file")?;
    Ok(config)
}

fn fetch_data(url: &str) -> Result<String> {
    let response = reqwest::blocking::get(url)
        .with_context(|| format!("Failed to fetch data from '{}'", url))?;
    let data = response.text().context("Failed to read response body")?;
    Ok(data)
}

fn process_data(_data: String) -> Result<()> {
    // Process the data...
    Ok(())
}

// Assume Config is a struct defined elsewhere
#[derive(serde::Deserialize)]
struct Config {
    api_url: String,
}