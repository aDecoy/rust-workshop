---
sidebar_position: 3
---

# Progam Structure

For some of you older .NET developers, you might remember the days when a .NET application had a `Program.cs` file and a `static void Main()` method that looked a little bit something like this:

```csharp showLineNumbers
static void Main(object[] args){
    Console.WriteLine("Hello, World!");    
}
```

And then Microsoft went and made things all simpler, which means a .NET application still has a `Program.cs` file, now it looks a little bit something like this:

```csharp
// See https://aka.ms/new-console-template for more information
Console.WriteLine("Hello, World!");
```

Now if you squint and look at this piece of code, go on. Really squint. I'm sure you can see the similarities with Rust:

```rust showLineNumbers showLineNumbers
fn main() {
    // highlight-next-line
    println!("Hello, world!");
}
```

:::important

The entrypoint to all Rust programs is `main()`. It takes no arguments, and returns no value.

:::

Yep, you've got a function (in Rust world, you call them functions not methods) called main. And then you're printing a line (`println!`) to the console. Code blocks, or scopes, are wrapped in curly braces: `{}`.

Pretty similar to .NET right?

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