use clap::Parser;

/// A simple CLI tool to greet users
#[derive(Parser)]
#[command(author = "Your Name", version = "1.0", about = "Greets a user", long_about = None)]
struct Args {
    /// Name of the user to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello, {}!", args.name);
    }
}