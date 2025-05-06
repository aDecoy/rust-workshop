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