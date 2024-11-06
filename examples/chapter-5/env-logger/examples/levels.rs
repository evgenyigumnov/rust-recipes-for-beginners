use env_logger;
use log::*;

fn main() {
    // Initialize the logger
    env_logger::init();

    // Log messages at different levels
    error!("This is an error message.");
    warn!("This is a warning message.");
    info!("This is an info message.");
    debug!("This is a debug message.");
    trace!("This is a trace message.");
}