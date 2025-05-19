---
sidebar_position: 4
---

# Challenge

Now it's time to apply what you've learned about modular architecture! In this module's challenge, you'll:

1. Take the monolithic web API from the previous module and split it into a modular architecture
2. Create separate modules for:
   - `core`: Containing your domain models and business logic, this should use the folder (`core`) method for defining a module
   - `data_access.rs`: Containing your data storage mechanism, this should use the single file method for defining a module
   - `lib.rs`: For your application library, all the setup code for your Axum API should live here
   - `main.rs`: The binary for your application entry point, simple a `main()` function that calls into your library
3. Apply appropriate visibility modifiers to enforce architectural boundaries
4. Ensure all modules are properly connected and the API still works as expected


*syntax for having both a library and a binary in the same project*
```toml
[lib]
path = "src/lib.rs"
name = "rust_users_app"

[[bin]]
path = "src/main.rs"
name = "rust_users"
```

Specifically, you need to:
- Move all domain types (User, UserDetails) and their implementations to the core module
- Move data storage (AppState) to the data access module
- Use proper visibility modifiers (pub, pub(crate)) to restrict access where appropriate
- Create clear module boundaries between business logic and infrastructure concerns
- Update imports in the main file to reference these new modules

The starter code for this challenge is [available on GitHub](https://github.com/jeastham1993/rust-for-dotnet-devs-workshop/tree/main/src/examples/module6/rust_app).

If you're struggling, you can find [a solution on GitHub](https://github.com/jeastham1993/rust-for-dotnet-devs-workshop/tree/main/src/solutions/module6/rust_app). Try it on your own first, if you're finding it difficult that's good. It means you're learning.

Good luck, and remember that a well-structured application will be much easier to maintain and extend as it grows!