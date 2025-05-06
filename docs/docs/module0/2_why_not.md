---
sidebar_position: 2
---

# Why not Rust

1. **Learning Curve**: I'll be honest with you—Rust has a steeper learning curve than many languages, including C#. The ownership and borrowing concepts will challenge you, especially in your first few weeks. You'll likely experience what Rustaceans call "fighting with the borrow checker" as you learn to structure your code to satisfy Rust's strict rules. Don't be discouraged! The struggle is temporary, and most developers report a "click moment" when these concepts suddenly make sense. The compiler provides extremely helpful error messages that guide you toward correct code, and the community is exceptionally welcoming to beginners.

2. **Ecosystem Maturity**: While the Rust ecosystem has grown impressively, it's still younger than .NET's. Some specialized libraries might be less mature or feature-complete than their .NET counterparts. You may occasionally need to implement functionality yourself that would be available out-of-the-box in .NET. The good news is that the ecosystem is growing rapidly, and many libraries are already production-ready.

3. **Team Adoption**: If you're working on a team, transitioning to Rust requires bringing everyone along on the learning journey. Finding experienced Rust developers for hiring can be more challenging than finding .NET developers, though this is changing as Rust's popularity continues to grow. You'll need to factor in training time and potential short-term productivity impacts when considering adoption.

4. **Compile Times**: Rust is slower than compiling code than .NET. The compiler toolchain is complex, and as you'll see throughout the workshop there are a lot of safety checks that happen at compile time. This is beneficial, as it removes a whole class of runtime bugs. Equally, you will find yourself sat around waiting for compilation to happen. Particularly if you're working on a large code base with lots of dependencies.

5. **Strictness**: The Rust compiler is strict. At some point in your Rust journey you'll find yourself swearing at the compiler wondering why the code you've written won't compile. The strictness is intentional, it stops you being a lazy programmer. Eventually, you'll come to appreciate this feature. I promise.
