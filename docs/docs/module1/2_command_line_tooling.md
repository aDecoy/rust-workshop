---
sidebar_position: 2
---

# Command Line Tooling

```sh
dotnet new console -n DotnetConsoleApp
```

```sh
cargo init --name rust_app
```

:::info

The Rust compiler is actually called `rustc`. You can compile a single file by calling `rustc <file_name>`. Clearly, most projects aren't made up of a single file though. And that's where `cargo` comes in.

:::

The respective commands will scaffold you a console application.

## Build Your Application

If you want to build your application, you'd run

```sh
dotnet build
```

In .NET, as you might expect you'd run

```sh
cargo build
```

in Rust. You can also publish a release build by running:

```sh
dotnet publish -c Release
```

and in Rust

```sh
cargo build --release
```

:::info

One of the trade-offs with Rust is extended compilation times. To get the performance, the compiler does a lot of work upfront. Running a `release` build further extends this. If you're running/developing locally stick to `cargo build`.

:::

Rust also has a command to check your application, rather than compiling you can validate that your code is correct.

```sh
cargo check
```