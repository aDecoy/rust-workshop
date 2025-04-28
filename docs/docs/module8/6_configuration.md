---
sidebar_position: 6
---

# Configuration

For almost any database you'll work with, you need to provide some kind of connection string. Which brings us on to the topic of application configuration. What's the best way to pass custom configuration to your application?

Almost all applications I've ever worked on, and I'm sure it's the same for you, need configuration. Whether that be database connection strings, API secrets, feature flags, or any other kind of configuration that's going to change the behaviour of your app at runtime. The important part of that sentence __at runtime__. Separating configuration from code allows you to build and ship your application once, and then configure it differently for different environments. This is a key part of the [12 factor app methodology](https://12factor.net/).

Imagine if every time you moved your application between environments you had to re-compile it and substitute in a new set of configuration values. If you went as far as to hard-code the values into your application, you'd actually need to go and make changes to your codebase every time you wanted to ship it to a new environment. That's not a great place to be. The litmus test for this that comes from the [config section of the 12 factor app methodology](https://12factor.net/config) is that you should be able to open source your code at any moment, without compromising any credentials.

## The Figment Crate

The Rust ecosystem has several different crates for managing application configuration, one I'm particularly fond of is the [Figment crate](https://github.com/SergioBenitez/Figment). 

:::info

The [`config`](https://github.com/rust-cli/config-rs/tree/main) crate is also popular in the Rust eco-system. It works in a similar way to Figment. Personally, I like the `Jail` support in Figment for use in unit tests which you'll see shortly. But I did want to point out that other configuration crates are available.

:::

Figment is a configuration management library for Rust that allows you to define your configuration in a structured way. It supports loading configuration from a variety of sources, including environment variables, strings and files. It also has a really nice API for defining your configuration in a structured way and then extracting that configuration into a custom struct. Let's see what that looks like:

```rust showLineNumbers
#[derive(Deserialize)]
struct Config {
    connection_string: String,
}

#[tokio::main]
async fn main() -> Result<(), ApplicationError> {
    let config: Config = Figment::new()
        .merge(Env::raw())
        .merge(figment::providers::Json::file("config.json"))
        .extract()
        .map_err(|e| ApplicationError::ApplicationError(e.to_string()))?;


   let postgres_data_access = PostgresUsers::new(config.connection_string).await?;

   // Removed for brefity
}
```

Let's walk through this step by step:

1. We define a struct called `Config` that has one fields: `connection_string`. We also derive the `Deserialize` trait on this struct so that we can easily convert it to and from JSON.
2. The first thing we do in the `main` function is to create a new Figment.
3. We then call the `merge` function on the Figment instance. The `Env::raw()` function tells Figment to load all environment variables.
4. We then call the `extract` function on the Figment instance to extract the configuration into a `Result<Configuration, Error>`, handling errors and returning the `Config` struct.
5. The connection string property is then passed to the `PostgresUsers` struct to initialize the database connection

:::important

Figment also has a function called `join` as an alternative to `merge`. The difference is that `merge` will overwrite any existing values in the configuration with the new values, whereas `join` will only fill in missing values. You can read more details in the [official docs](https://docs.rs/figment/latest/figment/struct.Figment.html#conflict-resolution).

In practice, this allows you to create hierarchial configuration.

:::

To actual use this configuraiton, you can either set the `connection_string` environment variable or create a new file called `config.json` in the same folder as your `Cargo.toml` file.

