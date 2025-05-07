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

### Cross-Compilation

Rust does support cross compilation, in the same way you might run `dotnet publish -c Release -r linux-x64` in .NET. There are two parts to cross compilation, the *host* and the *target*. The *host* platform is the one doing the compiling, and the *target* is what you are building for. 

The target platform is specified as a `triplet`, which follows the structure *machine-vendor-os*. The *machine* part could be `x86_64`, `aarch64` or even `wasm32`. The *vendor* part is going to be one of `pc` (Windows), `apple` (MacOS) and `unknown` for everything else. And then the *os* part tells the compiler what format to use for the final binary output, using a value of `linux` will give you *.so* files, `windows` will give you *.dll*.

To tell Rust to cross-compile, you pass the required target to the `--target` flag.

```sh
cargo build --target aarch64-unknown-linux
```

Before you can actually compile your application though, you'll need to add the toolchain for the required target.

```sh
rustup target add aarch64-unknown-linux
```

:::info

`Rustup` is a command-line tool that serves as the official installer and version management system for the Rust programming language.

:::

## Check your application

Rust also has a command to check your application, rather than compiling you can validate that your code is correct.

```sh
cargo check
```