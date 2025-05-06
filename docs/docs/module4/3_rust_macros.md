---
sidebar_position: 3
---

# Understanding Rust Macros

Macros are, in essence, a tool for making the Rust compiler write code for you. There are two different types of macros, declaritive and procedural, that give you the flexibility to implement many different forms of code generation.

For the purposes of this workshop, you won't actually be writing any macros yourself. But it's important to understand what they are, and what they are doing under the hood.

## Declarative Macros

Declarative macros (also called "macros by example" or "macro_rules! macros") allow you to write code that resembles a match expression but operates on Rust code at compile time. A simple example is the `println!` macro. When you write `println("Hello {}", "James")` at compile time that unwraps to:

```rust showLineNumbers
std::io::_print(format_args!("Hello {}", "James"));
```

This then calls the `format_args!()` macro, which then handles any formatting/string interpolation.

In the fantastic book **Rust for Rustaceans, by Jon Gjengset**, Jon referes to declartive macros as *"compiler-assisted search and replace: it does the job of for many, well structued transformation tasks, and for eliminating repetitive boilerplate"*.

Declartive macros are useful when you find yourself writing the same code over and over again and need a quick shorthand for writing that specific piece of code.

## Procedural Macros

Procedural macros are more akin to a combination of parser and code generation, where you write the glue code in between. You define **how** to generate code given a set of input tokens, rather than writing the exact code that gets generated.

Procedural macros act more like functions (they take syntax as input, manipulate it, and produce code as output). You can even introduce completely new syntax if you wish, the sky's the limit. There's even a crate for adding [inline Python in Rust](https://docs.rs/inline-python/latest/inline_python/). There are three different types of procedural macros:

- Function like macros
- Attribute macros, like `#[test]` 
- `derive` macros, which you'll see shortly

It's worth pointing out that procedural macros can significantly increase compile time, as they typically bring in some pretty large dependencies and you could end up generating a lot of code without actually realising it. The boilerplate adds  up, and therefore starts to add to your compile time.

For the remainder of this module, you'll focused on `derive` macros as these are what `serde` uses to implement serialization and deserialization.

### Derive Macros

Derive macros allow you to automate the implementation of a trait, where auto-implementation is possible. Derive macros should only be used if the trait is implemented often, and it's implementation is obvious.

Take the `Debug` macros, if you added the `Debug` macro to a struct what would you expect it to do? Print every property of the struct and it's value? Yeah that'd probablly be pretty useful. What about deriving `Serialize`, well yeah you would expect that to serialize your struct into a JSON string.

You don't need to understand in detail how macros work but I'd highly recommend picking up **Rust for Rustaceans, by Jon Gjengset** if you want to dive deeper. And a special shoutout to Jon as much of the content in this page is taken from the easy to follow explanations in the book. This [Youtube video](https://www.youtube.com/watch?v=Zmoy65pcHlk) is also helpful to see exactly what is happening under the hood.

Serde's derive macros automatically implement the `Serialize` and `Deserialize` traits for your types, saving you from writing boilerplate code.

Now that you understand macros, how exactly do you use them?