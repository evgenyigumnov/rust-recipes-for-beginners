# Chapter 5 Logging and Monitoring

## Introduction

Logging and monitoring are critical components in software development, enabling developers to track the behavior of applications, diagnose issues, and ensure proper functioning in production environments. In Rust, several crates provide robust tools for logging and log management, each suited for different use cases ranging from basic logging to structured, event-based diagnostics. This chapter introduces the key logging tools and techniques available in Rust, covering basic logging, structured logging, and log rotation.

We'll start with a simple logging setup using the `log` crate, which offers a standard logging facade, allowing you to log messages at various levels such as `info`, `debug`, and `error`. Next, we'll delve into structured logging using the `tracing` crate, which enables richer and more detailed logging, particularly useful in asynchronous and concurrent applications. Finally, we'll explore log rotation with `flexi_logger`, ensuring logs are managed efficiently by handling file size and rotation policies.

By the end of this chapter, you'll be equipped with practical recipes to integrate logging and monitoring into your Rust applications, making it easier to track performance, troubleshoot issues, and maintain a clean and manageable log history.

## Structure 
This chapter includes the following topics:
- Introduction to logging in Rust
- Basic logging with the `log` crate
- Structured logging using `tracing`
- Log rotation and management with `flexi_logger`
- Tracing operations with OpenTelemetry and Jaeger


## Objectives
The primary objective of this chapter is to provide a comprehensive understanding of logging and monitoring in Rust applications, enabling developers to effectively track and diagnose issues in their software. By the end of this chapter, readers will have learned how to set up and manage logging in Rust using popular crates, ensuring that logs capture crucial information at various stages of execution. The chapter begins with a focus on simple logging setups using the `log` crate, teaching how to capture basic messages like `info`, `error`, and `debug` logs. This foundation helps developers follow application behavior and detect errors in a straightforward manner.

Beyond basic logging, the chapter aims to deepen the understanding of structured logging with the `tracing` crate. Readers will learn how to enrich their logs with additional contextual information, which is particularly useful in asynchronous and concurrent Rust applications. The use of spans, events, and structured fields in `tracing` allows developers to better observe the flow of operations, making it easier to diagnose complex, multi-threaded issues.

Additionally, the chapter covers the management and rotation of log files, a crucial aspect of maintaining application stability in production environments. By integrating `flexi_logger`, readers will understand how to prevent log files from growing uncontrollably and how to implement efficient log rotation policies. This ensures that logs remain useful while minimizing the risk of overwhelming system resources.

Finally, the chapter introduces readers to distributed tracing with OpenTelemetry and Jaeger, providing the tools necessary to track the lifecycle of operations across services. By leveraging these technologies, developers can monitor performance, trace errors, and gain visibility into how requests propagate through distributed systems, enhancing their ability to maintain and optimize complex applications.


## Recipes
The chapter will cover the following recipes:
1. **Implementing Basic Logging with `log`**. Set up simple logging to standard output using the `log` crate and configure different log levels (info, debug, error).
2. **Structured Logging with `tracing`**. Add structured logging capabilities using the `tracing` crate to provide rich diagnostic information.
3. **Log Rotation with `flexi_logger`**. Configure `flexi_logger` to handle log file rotation, ensuring log files do not grow indefinitely.
4. **Tracing Opentelemetry with Jaeger**. Set up tracing for your application using OpenTelemetry and Jaeger to monitor the lifecycle of operations.


# Implementing Basic Logging with `log`

Logging is an essential feature in any application for tracking its behavior, diagnosing issues, and understanding its flow during execution. In Rust, the `log` crate provides a lightweight logging facade, allowing libraries and applications to produce log messages without being tied to a specific logging implementation.

In this section, we'll set up simple logging to the standard output using the `log` crate and configure different log levels such as `info`, `debug`, and `error`.

## Adding Dependencies

First, add the `log` crate to your `Cargo.toml` file:

```toml
[dependencies]
log = "0.4.22"
```

Since `log` is only a facade, you'll need a logging implementation to actually display the logs. For basic logging to the console, the `env_logger` crate is a good choice:

```toml
[dependencies]
log = "0.4.22"
env_logger = "0.11.5"
```

## Initializing the Logger

Before you can start logging messages, you need to initialize the logger. The `env_logger` reads configuration from environment variables and formats log messages.

In your `main.rs` or `lib.rs` file:

```rust
use log::{info, warn, error, debug, trace};
use env_logger;

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
```

Result of running the application:

```bash
[2024-10-25T05:44:06Z ERROR levels] This is an error message.
```

## Configuring Log Levels

By default, `env_logger` filters log messages based on the `RUST_LOG` environment variable. You can set this variable to control which log levels are displayed.

- To display all logs at the `info` level and above:

```bash
RUST_LOG=info cargo run
```

Result of running the application:

```bash
[2024-10-25T05:47:13Z ERROR levels] This is an error message.
[2024-10-25T05:47:13Z WARN  levels] This is a warning message.
[2024-10-25T05:47:13Z INFO  levels] This is an info message.
```


- To include `debug` level logs:

```bash
RUST_LOG=debug cargo run
```

Result of running the application:

```bash
[2024-10-25T05:48:08Z ERROR levels] This is an error message.
[2024-10-25T05:48:08Z WARN  levels] This is a warning message.
[2024-10-25T05:48:08Z INFO  levels] This is an info message.
[2024-10-25T05:48:08Z DEBUG levels] This is a debug message.
```

- To include `trace` level logs:

```bash
RUST_LOG=trace cargo run
```

Result of running the application:

```bash
[2024-10-25T05:48:50Z ERROR levels] This is an error message.
[2024-10-25T05:48:50Z WARN  levels] This is a warning message.
[2024-10-25T05:48:50Z INFO  levels] This is an info message.
[2024-10-25T05:48:50Z DEBUG levels] This is a debug message.
[2024-10-25T05:48:50Z TRACE levels] This is a trace message.
```

## Filtering Logs by Module

You can fine-tune logging by specifying log levels for specific modules or crates:

```bash
RUST_LOG=my_crate=debug,other_module=info cargo run
```

This configuration sets the log level for `my_crate` to `debug` and `other_module` to `info`.

## Customizing Log Output

The `env_logger` crate allows you to customize the format of your log messages. You can use a builder pattern to set the log target, format, and filters.

```rust
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
```

In this example, we also use the `chrono` crate to include timestamps in the log output. Add `chrono = "0.4.38"` to your `Cargo.toml` dependencies if you wish to include timestamps.

Result of running the application:

```bash
2024-10-25 10:57:38 [DEBUG] - This is a debug message with custom formatting.
```


## Example Application

Here's a complete example demonstrating basic logging:

```rust
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
```

To see all log messages, run:

```bash
RUST_LOG=trace cargo run
```

Result of running the application:

```bash
[2024-10-25T06:00:23Z INFO  app] Application started.
[2024-10-25T06:00:23Z DEBUG app] Starting application logic.
[2024-10-25T06:00:23Z TRACE app] Processing iteration 1
[2024-10-25T06:00:23Z TRACE app] Processing iteration 2
[2024-10-25T06:00:23Z TRACE app] Processing iteration 3
[2024-10-25T06:00:23Z WARN  app] Iteration 3 encountered a minor issue.
[2024-10-25T06:00:23Z TRACE app] Processing iteration 4
[2024-10-25T06:00:23Z TRACE app] Processing iteration 5
[2024-10-25T06:00:23Z DEBUG app] Finished application logic.
[2024-10-25T06:00:23Z ERROR app] Application error: An unexpected error occurred.
[2024-10-25T06:00:23Z INFO  app] Application finished.
```

## Summary

By integrating the `log` crate with an implementation like `env_logger`, you can set up flexible and configurable logging in your Rust applications. Adjusting log levels and customizing output formats helps you monitor application behavior and troubleshoot issues effectively.

Always ensure that sensitive information is not logged, especially in production environments. Proper log management and rotation are important topics that we'll cover in subsequent sections.



# Structured Logging with `tracing`

Structured logging provides a way to include rich, contextual information in your logs, making it easier to analyze and understand the behavior of your application. In Rust, the `tracing` crate offers a powerful framework for instrumenting applications to produce structured, event-based diagnostic information.

In this section, we'll explore how to use `tracing` to add structured logging to your Rust applications, enabling you to capture detailed insights into your code's execution.

## Why Use `tracing`?

Traditional logging libraries focus on emitting unstructured text messages. While this can be sufficient for simple applications, it falls short in complex, asynchronous, or concurrent environments where understanding the flow of execution is crucial. `tracing` addresses this by:

- **Capturing structured data**: Log messages can include key-value pairs, providing more context.
- **Introducing spans**: Represent periods of time in your application, allowing you to group related events.
- **Supporting async code**: Works seamlessly with Rust's async ecosystem, preserving context across await points.

## Setting Up `tracing`

To get started with `tracing`, add the following dependencies to your `Cargo.toml`:

```toml
[dependencies]
tracing = "0.1.40"
tracing-subscriber = "0.3.18"
```

- `tracing`: Core library for instrumentation.
- `tracing-subscriber`: Provides subscribers to collect and record trace data.

## Basic Usage

Here's how you can integrate `tracing` into a simple application:

```rust
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
```

**Explanation:**

- **Initializing the Subscriber**: Before logging, you need to set up a subscriber that defines how logs are collected and displayed.
- **Logging Macros**: Use `trace!`, `debug!`, `info!`, `warn!`, and `error!` to log messages at different levels.
- **Structured Fields**: Include key-value pairs (e.g., `value`, `error = ?e`) to add structured data to your logs.

Output:

```bash
2024-10-26T05:52:54.508633Z  INFO basic: Application started
2024-10-26T05:52:54.508777Z DEBUG basic: Current value value=42
2024-10-26T05:52:54.508842Z TRACE basic: Performing task
2024-10-26T05:52:54.508894Z ERROR basic: Task failed error="An error occurred"
```
## Using Spans for Context

Spans represent a period of time during which certain events occur. They can be nested and are useful for grouping related operations.

```rust
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
```

**Explanation:**

- **Creating Spans**: Use `span!` to define a span with a level and name.
- **Entering Spans**: Use `enter()` to set the current span context, affecting all logs within its scope.
- **Structured Data in Spans**: Include fields (e.g., `work_units`) to provide additional context.
- **Nested Spans**: Spans can be nested, allowing you to group related operations.
- **Dropping Spans**: When a span goes out of scope, its context is dropped.

Output:

```bash
2024-10-26T06:04:16.105484Z  INFO main: span: Starting application
2024-10-26T06:04:16.105654Z  INFO main:compute{work_units=2}: span: Performing computation
2024-10-26T06:04:16.105746Z  INFO main:compute{work_units=2}:nested_compute: span: Performing nested computation
```

## Filtering Logs

Control the verbosity of your logs by filtering based on level or other criteria. Modify `Cargo.toml` file:
```toml
tracing-subscriber = {version =  "0.3.18", features = ["env-filter"] }
```

Then, set up a subscriber with an environment filter:

```rust
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
```

**Usage:**

Set the `RUST_LOG` environment variable to control log levels:

```shell
RUST_LOG=info my_app
```

## Instrumenting Asynchronous Code

`tracing` integrates well with async functions, preserving context across await points.

Firstly add the following dependencies to your `Cargo.toml`:
```toml
[dependencies]
tokio = { version = "1.41.0", features = ["tokio-macros", "time", "rt", "rt-multi-thread", "macros"] }
```

Then, you can instrument async functions as follows:

```rust
use tracing::{info, instrument};

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
```

**Explanation:**

Let's break down the key features of this example:

- **`#[instrument]` Attribute**: Automatically creates a span for the function, capturing arguments and attaching it to all logs within.
- **Async Compatibility**: Context is preserved across `.await` points.

Result of running the application:

```bash
2024-10-26T06:41:08.624672Z  INFO instrument: Starting async application
2024-10-26T06:41:08.624866Z  INFO fetch_data: instrument: Fetching data...
2024-10-26T06:41:09.635002Z  INFO fetch_data: instrument: Data fetched
```


## Timing spans

You can measure the duration of a span by add **with_span_events(FmtSpan::CLOSE)**. This will log the duration of the span when it is closed.

```rust
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
```

Result of running the application:

```bash
2024-10-26T07:49:29.135862Z  INFO timing: Starting async application
2024-10-26T07:49:29.136019Z  INFO timing: Fetching data...
2024-10-26T07:49:30.490528Z  INFO fetch_data: timing: close time.busy=1.35s time.idle=11.0µs
2024-10-26T07:49:30.490736Z  INFO timing: Data fetched
```

Expected output:
- **time.busy**: The time spent in the span.
- **time.idle**: The time spent outside the span.


# Log Rotation with `flexi_logger`

In production environments, log files can grow rapidly and consume significant disk space if left unchecked. Implementing log rotation ensures that log files do not grow indefinitely by archiving or deleting old logs based on specified criteria. The [`flexi_logger`](https://docs.rs/flexi_logger/) crate provides a flexible and easy-to-use solution for log rotation in Rust applications.

In this section, we'll configure `flexi_logger` to handle log file rotation based on size and time, ensuring efficient log management for your Rust applications.

## Prerequisites
- **Add Dependencies**: Add the following to your `Cargo.toml` file:

```toml
[dependencies]
log = "0.4.22"
flexi_logger = "0.29.4"
```

## Example: Configuring Log Rotation with `flexi_logger`

The following example demonstrates how to set up `flexi_logger` to rotate log files when they reach a certain size:

```rust
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
            // Keep up to 5 rotated log files; remove older ones
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

```

**Explanation:**

- **Initialize the Logger:**

```rust
Logger::try_with_str("info")
  .unwrap()
```

  Initializes the logger with the log level filter set to `info` and above.

- **Specify Log File Details:**

```rust
.log_to_file(
  FileSpec::default()
      .directory("logs")
      .basename("myapp")
      .suffix("log"),
)
```

    - `directory("logs")`: Stores log files in the `logs` directory.
    - `basename("myapp")`: Sets `myapp` as the base name for log files.
    - `suffix("log")`: Appends `.log` to log file names.

- **Configure Rotation Policy:**

```rust
.rotate(
  Criterion::Size(10_000),
  Naming::Timestamps,
  Cleanup::KeepLogFiles(5),
)
  ```

    - `Criterion::Size(10_000)`: Rotates the log file when it reaches ~10 KB.
    - `Naming::Timestamps`: Renames old log files with a timestamp.
    - `Cleanup::KeepLogFiles(5)`: Retains the 5 most recent log files.

- **Duplicate Logs to Stdout:**

```rust
.duplicate_to_stdout(Duplicate::Info)
```

  Duplicates log messages of level `info` and above to standard output.

- **Start the Logger:**

```rust
.start()
.unwrap_or_else(|e| panic!("Logger initialization failed: {}", e));
```

Finalizes and starts the logger, handling any initialization errors.

In logs folder, you will see the following files:
```bash
myapp_r2024-10-26_14-33-16.restart-0043.log
myapp_r2024-10-26_14-33-16.restart-0044.log
myapp_r2024-10-26_14-33-17.restart-0045.log
myapp_r2024-10-26_14-33-17.restart-0046.log
myapp_r2024-10-26_14-33-17.restart-0047.log
myapp_rCURRENT.log
```

## Best Practices

- **Monitor Disk Usage:** Regularly monitor disk space to prevent issues due to unexpected log growth.
- **Secure Logs:** Ensure logs are stored securely, especially if they contain sensitive information.
- **Adjust Policies:** Tailor rotation criteria based on application needs and logging volume.
- **Backup Logs:** If necessary, implement a backup strategy for log files before cleanup.


Implementing log rotation with `flexi_logger` helps maintain application performance and system stability by managing log file sizes effectively. By customizing rotation criteria and log management policies, you can ensure that logging remains an asset rather than a liability in your Rust applications.




# Tracing Opentelemetry with Jaeger

Tracing is an essential part of understanding and debugging distributed systems. In modern software applications, which often rely on multiple services interacting across networks, it can be challenging to follow the lifecycle of a single request as it traverses various services and components. To address this complexity, tracing tools like OpenTelemetry and Jaeger offer the ability to collect and visualize information about how requests flow through the system, making it easier to identify performance bottlenecks, trace errors, and optimize the overall architecture.

We will explore how to set up tracing for your application using OpenTelemetry, the open-source observability framework, combined with Jaeger, a powerful backend for collecting, storing, and visualizing tracing data. We will demonstrate the process of integrating these tools in a Rust project and use them to monitor the lifecycle of operations in a sample application.

The provided code demonstrates a simple implementation of tracing using the opentelemetry and opentelemetry-otlp crates in Rust. The goal is to generate tracing spans, track operations, and export them to a Jaeger instance for visualization. 
We will break down each part of the code, explain the configuration of the tracing pipeline, and guide you through using Jaeger to monitor your application's activity.

Let's dive into the details of this example and see how we can enhance observability in Rust applications using OpenTelemetry and Jaeger.

## Prerequisites


Before we start, make sure you have the following dependencies in your `Cargo.toml` file:

```toml
[dependencies]
opentelemetry = "0.26.0"
opentelemetry-otlp = { version = "0.26.0" ,  features = ["tonic"] }
opentelemetry_sdk = { version = "0.26.0", features = ["rt-tokio"] }
async-std = { version = "1.6.0-beta.2" }
tokio = { version = "1.41.0", features = ["tokio-macros", "time", "rt", "rt-multi-thread", "macros"] }
```

## Setting Up OpenTelemetry with Jaeger

To get started with OpenTelemetry and Jaeger, you need to configure the tracing pipeline to collect and export trace data to a Jaeger instance. The following code demonstrates how to set up a basic tracing pipeline using OpenTelemetry and export the trace data to a Jaeger backend.

This code  based on code example from **opentelemetry** crate.
```rust
use opentelemetry::global::shutdown_tracer_provider;
use opentelemetry::{
    global,
    trace::{TraceContextExt, TraceError, Tracer},
    KeyValue,
};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{runtime, trace as sdktrace, Resource};

use std::error::Error;

fn init_tracer_provider() -> Result<opentelemetry_sdk::trace::TracerProvider, TraceError> {
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://localhost:4317"),
        )
        .with_trace_config(
            sdktrace::Config::default().with_resource(Resource::new(vec![KeyValue::new(
                "service.name",
                "tracing-jaeger",
            )])),
        )
        .install_batch(runtime::Tokio)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let tracer_provider = init_tracer_provider().expect("Failed to initialize tracer provider.");
    global::set_tracer_provider(tracer_provider.clone());

    let tracer = global::tracer("tracing-jaeger");
    tracer.in_span("main-operation", |cx| {
        let span = cx.span();
        span.set_attribute(KeyValue::new("my-attribute", "my-value"));
        span.add_event(
            "Main span event".to_string(),
            vec![KeyValue::new("foo", "1")],
        );
        tracer.in_span("child-operation...", |cx| {
            let span = cx.span();
            span.add_event("Sub span event", vec![KeyValue::new("bar", "1")]);
        });
    });

    shutdown_tracer_provider();
    Ok(())
}
```
This Rust code sets up OpenTelemetry tracing using the `opentelemetry` and `opentelemetry-otlp` crates, and then logs some span and event data to an OpenTelemetry collector, such as Jaeger, through an OpenTelemetry Protocol (OTLP) exporter. Here's a step-by-step breakdown:

1. **Initialization of the Tracer Provider**
- The `init_tracer_provider()` function initializes the OpenTelemetry tracing pipeline.
- It uses the OTLP exporter, which is configured to send traces to `http://localhost:4317` (the default gRPC endpoint for an OTLP collector like Jaeger).
- The `Resource` object defines metadata for the traces, such as the `service.name` attribute being set to `"tracing-jaeger"`.
- It calls `.install_batch(runtime::Tokio)` to install the tracing pipeline and send trace data in batches asynchronously using the Tokio runtime.

If this setup is successful, a `TracerProvider` is returned, which manages the trace pipeline.

2. **Starting the Asynchronous Main Function**
- The `main` function is the asynchronous entry point for the program.
- It first initializes the tracer provider via `init_tracer_provider()` and sets it globally using `global::set_tracer_provider(tracer_provider.clone())`. This ensures that the global tracer can be used throughout the code.

3. **Creating Traces and Spans**
- A `Tracer` is obtained using `global::tracer("tracing-jaeger")`. This tracer will generate spans for this named component (`"tracing-jaeger"`).
- Using the `in_span` method, two spans are created:
    - The first span is called `"main-operation"`. This span contains:
        - An attribute `my-attribute` with the value `"my-value"`.
        - An event called `"Main span event"` with an associated key-value pair (`"foo": "1"`).
    - Inside the `"main-operation"` span, a child span called `"child-operation..."` is created.
        - This child span logs an event `"Sub span event"` with the attribute (`"bar": "1"`).

A span is a logical unit of work (e.g., an operation), and it can have attributes (descriptive key-value pairs) and events (markers in time with additional data).

4. **Shutting Down the Tracer Provider**
- After the tracing work is complete, `shutdown_tracer_provider()` is called to flush and stop the tracing pipeline. This ensures that all traces are sent to the backend before the application exits.


In essence, this code demonstrates how to create and export structured trace data (spans, attributes, and events) to an OpenTelemetry-compatible tracing system like Jaeger using Rust.


## Running Jaeger Locally

```bash
# Run jaeger in background with native OTLP Ingestion
$ docker run -d -p16686:16686 -p4317:4317 -e COLLECTOR_OTLP_ENABLED=true jaegertracing/all-in-one:latest

# Run the app
$ cargo run

# View spans
$ firefox http://localhost:16686/
```


# Key learnings

- **Logging and monitoring** are essential for tracking application behavior, diagnosing issues, and ensuring proper functioning in production environments.
- **Rust** provides several logging crates, including `log`, `tracing`, and `flexi_logger`, each suited for different use cases.
- **Structured logging** with `tracing` enables richer and more detailed logging, particularly useful in asynchronous and concurrent applications.
- **Log rotation** with `flexi_logger` ensures logs are managed efficiently by handling file size and rotation policies.
- **Tracing operations** with OpenTelemetry and Jaeger helps monitor the lifecycle of operations in distributed systems.

# Conclusion

In this chapter, we've explored the importance of logging and monitoring in software development and how to effectively implement them in Rust applications. By setting up basic logging with the `log` crate, adding structured logging with `tracing`, managing log files with `flexi_logger`, and utilizing tracing tools like OpenTelemetry and Jaeger, you can gain deeper insights into your application's behavior and performance.

Logging and monitoring not only help you diagnose issues but also ensure the smooth operation of your software in production. With the right tools and techniques, as demonstrated in this chapter, you can build more reliable and maintainable systems.

In the next chapter, we will dive into how Rust handles the conversion of data structures into formats like JSON, YAML, and more, as well as how to deserialize these formats back into Rust types.

