use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "sub-command", version = "1.0", author = "Your Name", about = "An example CLI with subcommands")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Starts the server
    Start {
        /// Optional port number
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },
    /// Stops the server
    Stop,
    /// Restarts the server
    Restart {
        /// Force restart without prompt
        #[arg(short, long)]
        force: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Start { port } => {
            println!("Starting the server on port {}", port);
            // Implement start logic here
        }
        Commands::Stop => {
            println!("Stopping the server");
            // Implement stop logic here
        }
        Commands::Restart { force } => {
            if *force {
                println!("Force restarting the server");
            } else {
                println!("Restarting the server");
            }
            // Implement restart logic here
        }
    }
}