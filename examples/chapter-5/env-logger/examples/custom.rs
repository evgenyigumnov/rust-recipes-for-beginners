use env_logger::{Builder, Target};
use log::{LevelFilter,debug};
use std::io::Write;

fn main() {
    // Customize the logger
    Builder::new()
        .target(Target::Stdout)
        .filter_level(LevelFilter::Debug)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();

    debug!("This is a debug message with custom formatting.");
}