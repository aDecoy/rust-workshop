---
sidebar_position: 2
---

# Challenge

Now it's time to put what you've learned into practice! In this module's challenge, you'll:

1. Add the serde and serde_json dependencies to your project's Cargo.toml file
2. Apply the `Serialize` and `Deserialize` derive macros to your User and UserDetails types
3. Modify your main function to:
   - Create a user and serialize it to JSON
   - Print the JSON to the console
   - Parse a JSON string back into a User struct
4. Experiment with customizing the JSON output by adding at least one serde attribute (like rename_all)
5. Ensure you can successfully compile and run your application to confirm the JSON serialization works

The starter code is available in `src/examples/module4/rust_app`, and you can check your solution against `src/solutions/module4/rust_app`.

Good luck, and remember that working with JSON in Rust gives you the benefits of strong type checking while maintaining high performance!