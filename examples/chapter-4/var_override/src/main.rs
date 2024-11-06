use clap::{Command, Arg};
use std::env;
use std::path::Path;

fn main() {
    let matches = Command::new("var-override")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .required(false)
        )
        .get_matches();

    let config_path = if let Some(config) = matches.get_one::<String>("config")  {
        config.to_string()
    } else {
        env::var("CONFIG_PATH").unwrap_or_else(|_| String::from("/etc/myapp/config"))
    };

    if Path::new(&config_path).exists() {
        println!("Using configuration file at: {}", config_path);
        // Load and parse the configuration file
    } else {
        eprintln!("Configuration file not found at: {}", config_path);
        // Handle the error accordingly
    }
}