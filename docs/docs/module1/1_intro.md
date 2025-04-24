---
sidebar_position: 1
---

# Rust & .NET, More Similar Than You Might Think

If you've spent any time browsing the internet, you'll probably have seen the enormous learning curve that you take on as you start learning Rust. Impossible to learn, meant only for systems programmers and C++ developers.

An interesting thought, but actually there are more similarities between .NET & Rust than you might think. As a .NET developer, you'll find many familiar concepts that will help you get started with Rust quickly.

Both have modern command line interfaces:

```sh
dotnet new 
```

```sh
cargo init
```

For some of you older .NET developers, you might remember the days when a .NET application had a `Program.cs` file that looked a little bit something like this:

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

What's that, you want to run your application now? Cool, cool. In .NET land, that would look a little bit something like this wouldn't it:

```sh
dotnet run
```

I betcha can't guess the Rust CLI command:

```sh
cargo run
```

So, if you put all this together you can now go off and run your first Rust application:

```sh
cd src/module_1/rust_app
cargo run
```

And look at that, `Hello, world!`

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

## Your First Challenge

Now it's time for you to take your first steps with Rust! Here's a simple challenge to get you started:

1. Navigate to the first module's Rust application:
   ```sh
   cd src/module_1/rust_app
   ```

2. Open the `src/main.rs` file in your editor and modify the "Hello, world!" message to include your name.

3. Run the application to see your changes:
   ```sh
   cargo run
   ```

4. Try adding a second line to the program that prints the current date and time.
   Hint: You'll need to add the `chrono` crate to your dependencies:
   ```sh
   cargo add chrono
   ```
   
   Then use it in your code:
   ```rust showLineNumbers
   use chrono::Local;
   
   fn main() {
       println!("Hello, world! My name is [your name]");
       println!("The current time is: {}", Local::now());
   }
   ```

5. Run the application again to see both lines printed.

Congratulations! You've just written, modified, and run your first Rust program, and you've learned how to add and use external dependencies. In the next module, we'll dive deeper into Rust's memory safety features and how they compare to .NET.

Remember: the Rust compiler is your friend. If you get error messages, read them carefully - they often tell you exactly what's wrong and how to fix it!
