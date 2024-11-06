use log::{info, warn, error, debug, trace};
use env_logger;

fn main() {
    env_logger::init();

    info!("Application started.");

    if let Err(e) = run_app() {
        error!("Application error: {}", e);
    }

    info!("Application finished.");
}

fn run_app() -> Result<(), &'static str> {
    debug!("Starting application logic.");

    for i in 1..=5 {
        trace!("Processing iteration {}", i);
        if i == 3 {
            warn!("Iteration {} encountered a minor issue.", i);
        }
    }

    debug!("Finished application logic.");

    // Simulate an error
    Err("An unexpected error occurred.")
}