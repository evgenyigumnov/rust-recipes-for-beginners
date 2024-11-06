use std::time::Instant;

/// Starts the timer and returns the current time.
pub fn start_timer() -> Instant {
    Instant::now()
}

/// Stops the timer and prints the elapsed time in nanoseconds.
pub fn end_timer(start: Instant) {
    let duration = start.elapsed();
    println!("Execution time: {} nanoseconds", duration.as_nanos());
}