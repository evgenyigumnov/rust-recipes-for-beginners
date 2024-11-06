use flexi_logger::{Age, Cleanup, Criterion, Duplicate, FileSpec, Logger, Naming};
use log::{info, warn};

fn main() {
    // Configure flexi_logger
    Logger::try_with_str("info")
        .unwrap()
        .log_to_file(
            FileSpec::default()
                .directory("logs") // Create log files in the 'logs' directory
                .basename("myapp") // Base name for the log files
                .suffix("log"),    // Set the file suffix to '.log'
        )
        .rotate(
            // Rotate the log file when it reaches ~10 KB
            Criterion::Size(10_000),
            // Rename the old log file using a timestamp
            Naming::Timestamps,
            // Keep up to 7 rotated log files; remove older ones
            Cleanup::KeepLogFiles(5),
        )
        // Also duplicate logs to stdout
        .duplicate_to_stdout(Duplicate::Info)
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed: {}", e));

    // Application code
    for i in 0..10000 {
        info!("This is log entry number {}", i);
    }

    warn!("Application has finished logging.");
}
