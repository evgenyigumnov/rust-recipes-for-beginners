use tracing::{info, trace};
use tracing_subscriber::{filter::EnvFilter, FmtSubscriber};

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env()) // Use RUST_LOG env variable
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set subscriber");

    info!("Starting application");

    // Application code...

    trace!("Application finished");
}
