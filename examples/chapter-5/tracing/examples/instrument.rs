use tracing::{info, instrument, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {

    // Set up a subscriber to collect and display trace data
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE) // Set the maximum log level
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set subscriber");

    info!("Starting async application");

    fetch_data().await;
}

#[instrument]
async fn fetch_data() {
    info!("Fetching data...");
    // Simulate async operation
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    info!("Data fetched");
}