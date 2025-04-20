---
sidebar_position: 1
---

# Intro

- Explain traits and how they are like interfaces in .NET
- Make sure to explain `async_trait` and why that is neccessary
- Demonstrate the implementation of a trait
- Explain generics, from the `AppState` struct that has a generic type that implements `DataAccess`. The `<DataAccess + Send + Sync>` syntax is similiar to a `where` clause in .NET
- Briefly explain lifetimes in Rust, and what the lifetime of a variable means