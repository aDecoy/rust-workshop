---
sidebar_position: 2
---

# Logging

Up until this point, whenever you needed to see some output from your application you used the `println!()` macro. This works, but it's like relying exclusively on `Console.WriteLine()` in .NET. It's not really up to scratch when you are building production ready applications. So what is the alternative in Rust?

## The `log` crate

The [`log`](https://crates.io/crates/log) crate a logging facade provides a single logging API that abstracts over the actual logging implementation. It provides API's for writing log messages, with seperate crates that implement the actual log output. If you're familiar with `Serilog` in the .NET ecosystem, the various different `Sinks` provided, and the interplay with the `ILogger<T>` interface this will feel familiar.

You configure your logger once when your application starts up and from there you write log messages using:

```rust showLineNumbers
log::trace!("Trace message");
log::info!("Info message");
log::warn!("Warning message");
log::error!("Error message");
```

### Configuring the log crate

You can see the available logging implementations in the [docs for the log crate](https://crates.io/crates/log). For the purposes of this workshop, you'll be focused on structured logging in a JSON format. For that, you'll need to add the `structured-logger` crate.

:::info

You'll need to configure your logger in your `main()` function. It is generally recommended to make your logger configuration the first thing that happens when your application runs. That way, you have structured logging available from the start.

:::

```rust showLineNumbers
use structured_logger::{async_json::new_writer, Builder};

#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    let log_level = std::env::var("LOG_LEVEL").unwrap_or("INFO".to_string());
    // Initialize the logger.
    Builder::with_level(&log_level)
        .with_target_writer("*", new_writer(tokio::io::stdout()))
        .init();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.map_err(|e| ApplicationError::ApplicationError(e.to_string()))?;
    
    log::info!("listening on {}", listener.local_addr().unwrap());

    // Prints:
    // {"level":"INFO","message":"listening on 0.0.0.0:3000","target":"module_10_rust_app","timestamp":1746536374630}
}
```

You can configure the desired log level passing the desired level (DEBUG, TRACE, INFO, WARN, ERROR) to the `with_level()` function on the structured logger `Builder`.


