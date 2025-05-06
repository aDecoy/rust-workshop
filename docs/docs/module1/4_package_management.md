---
sidebar_position: 4
---

# Package Management

Just like you use NuGet packages in .NET, Rust has its own package ecosystem called `crates`. Crates are managed by Cargo, Rust's package manager and build system (similar to how you use `dotnet` for .NET projects).

:::info

You can find a list of all available crates at [crates.io](https://crates.io)

:::

Here's how package management works in Rust:

1. **Cargo.toml** - This is similar to your `.csproj` file. It defines your project's metadata and dependencies:

    ```toml
    [package]
    name = "my_project"
    version = "0.1.0"
    edition = "2021"

    [dependencies]
    serde = { version = "1.0", features = ["derive"] }
    tokio = { version = "1", features = ["full"] }
    ```

2. **Adding dependencies** - Instead of using the NuGet package manager UI, you can add dependencies directly to your `Cargo.toml` file or use the command line:

    ```sh
    cargo add serde --features derive
    ```

3. **Installing dependencies** - When you run `cargo build` or `cargo run` for the first time, Cargo automatically downloads and compiles your dependencies.

Cargo is more than a package manager - it's a complete build system. It handles compiling your code, running tests, generating documentation, and more. As a .NET developer, you can think of it as combining the functionality of `dotnet`, NuGet, and MSBuild into a single tool.

## Crate Features

Cargo “features” provide a mechanism to express conditional compilation and optional dependencies. A package defines a set of named features in the [features] table of Cargo.toml, and each feature can either be enabled or disabled.

If you're using a crate published by a 3rd party author you can specify the features you want to be enabled for a specific crate in the `Cargo.toml` file. The below enables the `derive` feature. Most crates enable a default set of features, which you can disable by setting `default-features = false`.

```toml
serde = { version = "1.0", features = ["derive"] }
```

This helps to ensure you only compile exactly the code you need when running your application is running in production. And don't worry, the compiler will tell you a dependency isn't found. Although the compiler can be rather unhelpful here by telling you a given struct/function doesn't exist. It can take some searching through the crate docs to find the required feature.

:::info

If you're authoring a crate, you can define the available features in your `Cargo.toml` file.

```toml
[features]
# Defines a feature named `webp` that does not enable any other features.
my_custom_feature = []
```

And then in your code you can define conditional compilation by adding the `cfg(feature)` attribute

```rust
#[cfg(feature = "my_custom_feature")]
pub mod webp;
```

:::