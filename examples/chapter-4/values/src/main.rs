use clap::Parser;

/// Simple program to greet users
#[derive(Parser)]
struct Cli {
    /// Names of the users
    names: Vec<String>,

    /// Display the greeting in uppercase
    #[arg(short, long)]
    uppercase: bool,
}

fn main() {
    let args = Cli::parse();

    for name in args.names {
        let greeting = format!("Hello, {}!", name);
        if args.uppercase {
            println!("{}", greeting.to_uppercase());
        } else {
            println!("{}", greeting);
        }
    }
}