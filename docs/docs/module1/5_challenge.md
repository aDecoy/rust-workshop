---
sidebar_position: 5
---

# Challenge

Now it's time for you to take your first steps with Rust! Here's a challenge to get you started:

1. Initialize a Rust application in a brand new directory
2. Open the `src/main.rs` file in your editor and modify the "Hello, world!" message to include your name.
3. Run the application to see your changes
4. Try adding a second line to the program that prints the current date and time.

   Hint: You'll need to add the `chrono` crate to your dependencies:
   ```sh
   cargo add chrono
   ```

   And chrono has a method called `Local::now()`

   ```rust showLineNumbers
    let time_now = Local::now();
   ```

5. Run the application again to see both lines printed.

Congratulations! You've written, modified, and run your first Rust program, and you've learned how to add and use external dependencies. In the next module, we'll dive deeper into Rust's memory safety features and how they compare to .NET.

Remember: the Rust compiler is your friend. If you get error messages, read them carefully - they often tell you exactly what's wrong and how to fix it!

The starter code for this challenge is [available on GitHub](https://github.com/jeastham1993/rust-for-dotnet-devs-workshop/tree/main/src/examples/module1/rust_app).

If you're struggling, you can find [a solution on GitHub](https://github.com/jeastham1993/rust-for-dotnet-devs-workshop/tree/main/src/solutions/module1/rust_app). Try it on your own first, if you're finding it difficult that's good. It means you're learning.
