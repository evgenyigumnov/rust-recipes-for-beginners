fn get_env_variable(key: &str) -> Result<String, String> {
    let value = std::env::var_os(key)
        .ok_or(format!("Environment variable {} not found", key))?; // Converts Option to Result
    Ok(value.into_string().map_err(|_| "Invalid UTF-8 sequence")?)
}

fn main() {
    match get_env_variable("HOME") {
        Ok(value) => println!("Home directory: {}", value),
        Err(e) => eprintln!("Error: {}", e),
    }
}