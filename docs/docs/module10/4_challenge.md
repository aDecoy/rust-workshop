---
sidebar_position: 4
---
# Challenge

Now it's time to apply what you've learned about observability in Rust! In this module's challenge, you need to:

1. Add the `log` and `structured-logger` crates and implement structured logging:
2. Add the `tracing`, `tracing-subscriber`, `opentelemetry`, `opentelemetry_sdk`, `opentelemetry-semantic-conventions` and `opentelemetry-otlp` crates and implement distributed tracing

:::info

The final dependencies in your `Cargo.toml` should be:

```toml
[package]
name = "module_10_rust_app"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1.0.96"
async-trait = "0.1.83"
axum = { version = "0.8.1", features = ["macros"] }
serde = { version = "1.0.218", features = ["derive"] }
thiserror = "2"
tokio = { version = "1", features = ["full"] }
sqlx = {version = "0.8.3", features = ["postgres", "runtime-tokio"]}
argon2 = "0.5.3"
regex = "1.11.1"
mockall = {version = "0.13.1"}
log = {version = "0.4.27"}
structured-logger = "1.0.4"
tracing = "0.1.41"
tracing-opentelemetry = "0.30.0"
tracing-subscriber = { version = "0.3.19", default-features = false, features = ["registry", "std", "fmt"] }
opentelemetry = { version = "0.29.1", default-features = false, features = ["trace"] }
opentelemetry_sdk = { version = "0.29.0", default-features = false, features = ["trace"] }
opentelemetry-semantic-conventions = { version = "0.29.0", features = ["semconv_experimental"] }
opentelemetry-otlp = { version = "0.29.0", features = ["metrics", "grpc-tonic"] }

[dev-dependencies]

```

:::

3. Run `docker compose up -d`, which includes a locally running version of Jaeger

4. Test the observability of your application and expand on any gaps

The starter code for this challenge is [available on GitHub](https://github.com/jeastham1993/rust-for-dotnet-devs-workshop/tree/main/src/examples/module10/rust_app).

If you're struggling, you can find [a solution on GitHub](https://github.com/jeastham1993/rust-for-dotnet-devs-workshop/tree/main/src/solutions/module10/rust_app). But try it on your own first, if you're finding it difficult that's good. It means you're learning.