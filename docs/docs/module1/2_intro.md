---
sidebar_position: 2
---

# Rust & .NET, More Similar Than You Might Think

If you've spent any time browsing the internet, you'll probably have heard about the enormous learning curve that you take on as you start learning Rust. Impossible to learn, meant only for systems programmers and C++ developers.

An interesting thought... But actually there are more similarities between .NET & Rust than you might think. As a .NET developer, you'll find many familiar concepts that will help you get started with Rust quickly. And whilst Rust might have started life as a systems programming language, it's a really powerful way to build many kinds of business applications.

Both Rust & .NET have modern command line interfaces (CLI's):

## Command Line Tooling

```sh
dotnet new console -n DotnetConsoleApp
```

```sh
cargo init --name rust_app
```

:::info

The Rust compiler is actually called `rustc`. You can compile a single file by calling `rustc <file_name>`. Clearly, most projects aren't made up of a single file though. And that's where `cargo` comes in.

:::

The respective commands will scaffold you a simple console application.