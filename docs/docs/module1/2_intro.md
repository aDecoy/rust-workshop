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

The respective commands will scaffold you a simple console application.

## Progam Structure

For some of you older .NET developers, you might remember the days when a .NET application had a `Program.cs` file and a `static void Main()` method that looked a little bit something like this:

```csharp showLineNumbers
static void Main(object[] args){
    Console.WriteLine("Hello, World!");    
}
```

And then Microsoft went and made things all simpler, which means a .NET application still has a `Program.cs` file, but now it looks a little bit something like this:

```csharp
// See https://aka.ms/new-console-template for more information
Console.WriteLine("Hello, World!");
```

Now if you squint and look at this piece of code, go on... Really squint... I'm sure you can see the similarities with Rust:

```rust showLineNumbers showLineNumbers
fn main() {
    // highlight-next-line
    println!("Hello, world!");
}
```

Yep, you've got a function (in Rust world, we call them functions not methods) called main. And then you're printing a line (`println!`) to the console.

Pretty similar right?

## Run Your Application

What's that, you want to run your application now? Cool, cool. In .NET land, that would look a little bit something like this wouldn't it:

```sh
dotnet run
```

I bet you can't guess the Rust CLI command:

```sh
cargo run
```

And that's not all. Rust also has:

1. Data types & Variables

    ```rust showLineNumbers
    let first_name: String = "James";
    ```

2. Structures (similar to classes)

    ```rust showLineNumbers
    struct User {
        name: String,
        email: String,
    }
    ```

3. Functions / methods

    ```rust showLineNumbers
    impl User {
        // no 'self' at all defines a static method. Called using User::new()
        fn new(email_address: &str, name: &str) -> User {
            User {
                email_address: email_address.to_string(),
                name: name.to_string(),
            }
        }
    }
    ```
4. And string interpolation

    ```rust showLineNumbers
    let myName = "James";

    println!("My name is {}", myName);
    ```

## Package Management in Rust

Just like you use NuGet packages in .NET, Rust has its own package ecosystem called "crates". Crates are managed by Cargo, Rust's package manager and build system (similar to how you use `dotnet` for .NET projects).

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

4. **Publishing packages** - You can publish your own crates to [crates.io](https://crates.io), the Rust community's central package registry (similar to nuget.org).

Cargo is more than just a package manager - it's a complete build system. It handles compiling your code, running tests, generating documentation, and more. As a .NET developer, you can think of it as combining the functionality of `dotnet`, NuGet, and MSBuild into a single tool.