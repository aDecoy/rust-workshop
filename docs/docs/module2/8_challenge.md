---
sidebar_position: 8
---

# Challenge

## Demos

There are a couple of demos as part of this module, however, I would recommend exploring the [memory demo](https://github.com/jeastham1993/rust-for-dotnet-devs-workshop/tree/main/src/demos/1--memory-safety) and [memory efficiency](https://github.com/jeastham1993/rust-for-dotnet-devs-workshop/tree/main/src/solutions/module2/2-sustainability) demos alongside their .NET counterparts.

Open up Activity Monitor on Mac, or Task Manager on Windows, and then run both the .NET and Rust examples and see how it affects your local resource consumption.

## Challenge

The code in [src/examples/module2/rust_app](/src/examples/module2/rust_app/) contains the beginnings of a user management application. But it **doesn't compile** 🥺, your challenge is to get this small code snippet working. There are issues with borrowing, ownership and mutability.

If you get past that, you can also try to better handle errors in the `update_name` function. Instead of always updating the name, let's return an error if the length of the new name is less than or equal to zero.



