---
sidebar_position: 1
---

# Rust & .NET, More Similar Than You Might Think

If you've spent any time browsing the internet, you'll probably have seen the enormous learning curve that you take on as you start learning Rust. Impossible to learn, meant only for systems programmers and C++ developers.

An interesting thought, but actually there are more similarities between .NET & Rust than you might think.

Both have modern command line interfaces:

```sh
dotnet new 
```

```sh
cargo init
```

For some of you older .NET developers, you might remember the days when a .NET application had a `Program.cs` file that looked a little bit something like this:

```c#
static void Main(object[] args){
    Console.WriteLine("Hello, World!");    
}
```

And then Microsoft went and made things all simpler, wihch means a .NET application still has a `Program.cs` file, but now it looks a little bit something like this:

```c#
// See https://aka.ms/new-console-template for more information
Console.WriteLine("Hello, World!");
```

Now if you squint and look at this piece of code, go on... Really squint... It I'm sure you can see the similarities.

```rs
fn main() {
    println!("Hello, world!");
}
```

Yep, you've got a function (in Rust world, we call them functions not methods) called main. And then you're printing a line (`println!`) to the console.

Pretty similar right?

What's what, you want to run your application now. Cool, cool. In .NET land, that would look a little bit something like this wouldn't it:

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

    ```rs
    var first_name: String = "James";
    ```

2. Classes

    ```rs
    struct User {
        
    }
    ```

3. Functions / methods

    ```rs
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

### Package Management

Explain how package management works in Rust.

But, you're getting ahead of yourself now aren't you. Let's take things step by step.
