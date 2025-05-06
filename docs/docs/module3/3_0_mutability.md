---
sidebar_position: 2
---

# Mutability

:::important

Every variable in Rust is immutable by default.

:::

To see this in action, go and run the `module3` code:

```sh
cd src/examples/module3/rust_app
cargo run
```

And oops! There's a problem:

**error[E0384]: cannot assign twice to immutable variable `integer_example`**

See if you can go and fix it. Go on, try on your own.