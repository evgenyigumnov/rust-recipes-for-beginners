use tracing::{info, instrument, Level};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {

    // Set up a subscriber to collect and display trace data
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE) // Set the maximum log level
        .with_span_events(FmtSpan::CLOSE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set subscriber");

    info!("Starting async application");

    info!("Fetching data...");
    fetch_data().await;
    info!("Data fetched");

}

#[instrument]
async fn fetch_data() {
    // Simulate async operation
    for _i in 0..200000000 {

    }
}