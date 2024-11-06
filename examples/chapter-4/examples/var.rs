use std::env;
use std::path::Path;

fn main() {
    let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| String::from("/etc/myapp/config"));

    if Path::new(&config_path).exists() {
        println!("Using configuration file at: {}", config_path);
        // Load and parse the configuration file
    } else {
        eprintln!("Configuration file not found at: {}", config_path);
        // Handle the error accordingly
    }
}