---
sidebar_position: 3
---

# Distributed tracing

Now that you've got structured logging in place, let's look at how you implement distributed tracing. The [`tracing`](https://crates.io/crates/tracing) crate, as part of the `tokio` ecosystem has long been the de-facto standard in Rust applications. Many 3rd party libraries have built in support for tracing, including the `Axum` web framework that you have used earlier in this workshop.

In order to record trace events, executables have to use a Subscriber implementation compatible with tracing. A Subscriber implements a way of collecting trace data, such as by logging it to standard output. One of the available subscribers is the [`tracing-opentelemetry`](https://crates.io/crates/tracing-opentelemetry) subscriber.

This allows interop between the `tracing` crate OpenTelemetry, allowing you to instrument your code with `tracing` but then export the data to an OpenTelemetry compatible backend.

```rust showLineNumbers
use opentelemetry::{trace::TracerProvider as _, KeyValue};
use opentelemetry_sdk::{
    trace::{RandomIdGenerator, Sampler, SdkTracerProvider},
    Resource,
};
use opentelemetry_semantic_conventions::{
    attribute::{DEPLOYMENT_ENVIRONMENT_NAME, SERVICE_NAME, SERVICE_VERSION},
    SCHEMA_URL,
};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing_opentelemetry::{OpenTelemetryLayer};

struct OtelGuard {
    tracer_provider: SdkTracerProvider,
}

// The `Drop` trait will execute when the memory is cleared for a given struct
// This ensures that traces are always flushed when the OtelGuard struct is dropped from memory.
impl Drop for OtelGuard {
    fn drop(&mut self) {
        if let Err(err) = self.tracer_provider.shutdown() {
            eprintln!("{err:?}");
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    let _otel_guard = init_tracing_subscriber();

    // Rest of startup code
}

// Create a Resource that captures information about the entity for which telemetry is recorded.
fn resource() -> Resource {
    Resource::builder()
        .with_schema_url(
            [
                KeyValue::new(SERVICE_NAME, "users-service"),
                KeyValue::new(SERVICE_VERSION, "1.0.0"),
                KeyValue::new(DEPLOYMENT_ENVIRONMENT_NAME, "develop"),
            ],
            SCHEMA_URL,
        )
        .build()
}


// Construct TracerProvider for OpenTelemetryLayer
fn init_tracer_provider() -> SdkTracerProvider {
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .build()
        .unwrap();

    SdkTracerProvider::builder()
        // Customize sampling strategy
        .with_sampler(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
            1.0,
        ))))
        // If export trace to AWS X-Ray, you can use XrayIdGenerator
        .with_id_generator(RandomIdGenerator::default())
        .with_resource(resource())
        .with_batch_exporter(exporter)
        .build()
}

// Initialize tracing-subscriber and return OtelGuard for opentelemetry-related termination processing
fn init_tracing_subscriber() -> OtelGuard {
    let tracer_provider = init_tracer_provider();

    let tracer = tracer_provider.tracer("users-service");

    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::LevelFilter::from_level(
            Level::INFO,
        ))
        .with(OpenTelemetryLayer::new(tracer))
        .init();

    OtelGuard {
        tracer_provider,
    }
}
```

Once you've got your subscriber configured, you can then add traces and spans to your application in two ways.

## Attribute tracing

You can add the `tracing::instrument` macro to to create and enter a tracing span every time the function is called. As a default, this automatically tags the span with any attributes passed into the function.

To skip recording one or more arguments to a function or method, pass the argument’s name inside the skip() argument on the #[instrument] macro. This can be used when an argument to an instrumented function does not implement `fmt::Debug`, or to exclude an argument with a verbose or costly Debug implementation. 

You can also use `skip_all` to skip all arguments.

Additional fields (key-value pairs with arbitrary data) can be passed to to the generated span through the fields argument on the `#[instrument]` macro. Strings, integers or boolean literals are accepted values for each field. You can also initialize a field with an empty value, and then populate it as part of your function code. In the below example `user.email_is_valid` and `user.password_is_valid` are added as fields.

```rust showLineNumbers
#[tracing::instrument(skip(state, payload), fields(user.email_is_valid, user.password_is_valid))]
async fn register_user<TDataAccess: DataAccess + Send + Sync>(){

}
```

And then the `email_is_valid` function inside the `User` struct sets the value of the field to be true or false respectively.

```rust showLineNumbers
fn email_is_valid(input: &str) -> Result<(), ApplicationError> {
        let re = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        if re.is_match(input) {
            tracing::Span::current().record("user.email_is_valid", "true");
            Ok(())
        } else {
            tracing::Span::current().record("user.email_is_valid", "false");
            Err(ApplicationError::ApplicationError("Invalid email address".to_string()))
        }
    }
```

## Manually starting spans

You can also manually start spans using the `span!()` macro. The level and span name are required parameters, and then you can pass in any number of additional arguments which will be added as span tags. The below example will start a new span called `user.new` and there will be a span tag called `user.type` set to standard.

The call to `enter()` is important, as this starts the span. A `Span` then implements the `Drop` trait, so whenever the variable is dropped from memory the end time is recorded. In this example, when the new function returns the `_entered` and `span` variables are dropped from memory, resulting in the end_time of the span being set.

```rust showLineNumbers
pub fn new(email_address: &str, name: &str, password: &str) -> Result<User, ApplicationError> {
    let span = span!(Level::INFO, "user.new", "user.type" = "standard");
    let _entered = span.enter();

    // Other code from new function
}
```

## Open Telemetry

It is also possible to configure distributed tracing **only** using the OpenTelemetry SDK without the need for interop with the `tracing` crate. At the time of writing, the `tracing` crate is still the de-facto standard inside many applications and libraries and the interop support is getting better. Personally, I feel like this is a good balance between a native way of doing things (`tracing`) with an open standard (`OpenTelemetry`). Much like how the OpenTelemetry implementation for .NET interops with `System.Diagnostics.Activity` to re-use a more native language specific feature.