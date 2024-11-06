use tracing::{debug, error, info, trace, Level};
use tracing_subscriber::FmtSubscriber;

fn main() {
    // Set up a subscriber to collect and display trace data
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE) // Set the maximum log level
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set subscriber");

    info!("Application started");

    let value = 42;
    debug!(value, "Current value");

    if let Err(e) = perform_task() {
        error!(error = ?e, "Task failed");
    }
}

fn perform_task() -> Result<(), &'static str> {
    trace!("Performing task");
    // Simulate a task that may fail
    Err("An error occurred")
}