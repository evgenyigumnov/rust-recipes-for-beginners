use clap::Parser;

/// Simple program to greet a user
#[derive(Parser)]
struct Cli {
    /// Name of the user (must be at least 3 characters)
    #[arg(value_parser = validate_name)]
    name: String,

    /// Display the greeting in uppercase
    #[arg(short, long)]
    uppercase: bool,
}

fn validate_name(name: &str) -> Result<String, String> {
    if name.len() >= 3 {
        Ok(name.to_string())
    } else {
        Err(String::from("Name must be at least 3 characters long"))
    }
}

fn main() {
    let args = Cli::parse();

    let greeting = format!("Hello, {}!", args.name);
    if args.uppercase {
        println!("{}", greeting.to_uppercase());
    } else {
        println!("{}", greeting);
    }
}