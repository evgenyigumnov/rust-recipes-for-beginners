use tracing::{info, span, Level};
use tracing_subscriber::FmtSubscriber;

fn main() {
    // Set up a subscriber to collect and display trace data
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE) // Set the maximum log level
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set subscriber");

    let main_span = span!(Level::INFO, "main");
    let _enter = main_span.enter(); // Enter the span's context

    info!("Starting application");

    compute();
}

fn compute() {
    let compute_span = span!(Level::DEBUG, "compute", work_units = 2);
    let _enter = compute_span.enter();

    info!("Performing computation");
    // Computation logic...
    nested_compute();

}

fn nested_compute() {
    let nested_span = span!(Level::DEBUG, "nested_compute");
    let enter = nested_span.enter();

    info!("Performing nested computation");
    // Nested computation logic...

    // Drop the span's context if it goes out of scope
    drop(enter);

}