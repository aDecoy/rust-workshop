---
sidebar_position: 1
---

# Why

Start With Why by Simon Sinek. A book that flipped how I view many things in technology. As a software engineer, it's easy to get caught up in what and how. What do I need to build? And how exactly do I need to build it? Normally spending more time than neccessary on that second one.

That misses a fundamental question though, **why?** To understand why I need to do something, makes the what and the how so much easier to understand.

So what's where I want to start today, why Rust?

## Why Rust

1. **Performance**: As a .NET developer, you're used to decent performance. Rust takes this to another level. Rust offers C/C++-level performance without sacrificing safety. You get zero-cost abstractions, minimal runtime overhead, and predictable performance characteristics. This means your code runs fast—really fast. When you have compute-intensive operations or need to optimize resource usage, Rust shines. You'll see comparable or better performance than C# in most scenarios, with benchmarks consistently showing Rust near the top of performance rankings alongside C and C++.

2. **Memory Safety Without Garbage Collection**: One of the biggest differences you'll notice coming from .NET is that Rust achieves memory safety without a garbage collector. Instead, Rust uses a compile-time ownership system that prevents common bugs like null references, dangling pointers, and data races—all without runtime overhead. As you write code, the compiler becomes your ally, catching potential issues before your program ever runs. This leads to more robust, secure applications that you can deploy with confidence.

3. **Resource Efficiency**: When you run .NET applications, you're accepting a certain level of resource overhead—memory for the runtime, CPU cycles for garbage collection, and more. Rust programs are remarkably resource-efficient by comparison. They have tiny memory footprints, predictable resource usage patterns, and lower overall infrastructure costs. This matters when you're scaling services, running on edge devices, or trying to fit more workloads on the same hardware. Your applications will use dramatically less memory and compute resources, which translates to real cost savings in production environments.

4. **Fearless Concurrency**: Writing correct concurrent code in .NET requires careful attention to locking, race conditions, and thread safety. Rust's ownership model makes it impossible to create data races at compile time. The compiler literally won't let you build code with these kinds of bugs. You can write highly concurrent programs with confidence, knowing that many common threading issues are eliminated by design. This is revolutionary for building modern, multi-threaded applications that fully utilize today's hardware.

5. **Modern Language Features**: As you learn Rust, you'll discover it offers many of the features you enjoy in C#: pattern matching, async/await, LINQ-like iterators, and more. Rust often takes these concepts further with features like match exhaustiveness checking, a more powerful trait system, and first-class support for functional programming patterns. You'll have powerful tools for expressing your ideas clearly and concisely.

6. **Ecosystem and Interoperability**: The Rust ecosystem has matured rapidly, with crates (packages) available for nearly everything you might need. Need to talk to databases? Process JSON? Build web APIs? There are mature, well-maintained libraries for all of these. Furthermore, Rust has excellent foreign function interface (FFI) capabilities, making it easy to integrate with existing C libraries or even interoperate with your .NET code when needed.

## Why not Rust

1. **Learning Curve**: I'll be honest with you—Rust has a steeper learning curve than many languages, including C#. The ownership and borrowing concepts will challenge you, especially in your first few weeks. You'll likely experience what Rustaceans call "fighting with the borrow checker" as you learn to structure your code to satisfy Rust's strict rules. Don't be discouraged! The struggle is temporary, and most developers report a "click moment" when these concepts suddenly make sense. The compiler provides extremely helpful error messages that guide you toward correct code, and the community is exceptionally welcoming to beginners.

2. **Ecosystem Maturity**: While the Rust ecosystem has grown impressively, it's still younger than .NET's. Some specialized libraries might be less mature or feature-complete than their .NET counterparts. You may occasionally need to implement functionality yourself that would be available out-of-the-box in .NET. The good news is that the ecosystem is growing rapidly, and many libraries are already production-ready.

3. **Team Adoption**: If you're working on a team, transitioning to Rust requires bringing everyone along on the learning journey. Finding experienced Rust developers for hiring can be more challenging than finding .NET developers, though this is changing as Rust's popularity continues to grow. You'll need to factor in training time and potential short-term productivity impacts when considering adoption.

4. **Compile Times**: Rust is slower than compiling code than .NET. The compiler toolchain is complex, and as you'll see throughout the workshop there are a lot of safety checks that happen at compile time. This is beneficial, as it removes a whole class of runtime bugs. Equally, you will find yourself sat around waiting for compilation to happen. Particularly if you're working on a large code base with lots of dependencies.

5. **Strictness**: The Rust compiler is strict. At some point in your Rust journey you'll find yourself swearing at the compiler wondering why the code you've written won't compile. The strictness is intentional, it stops you being a lazy programmer. Eventually, you'll come to appreciate this feature. I promise.

## When to choose Rust

Given these pros and cons, when should you as a .NET developer consider using Rust?

- **Performance-critical systems**: When every millisecond counts
- **Resource-constrained environments**: Edge computing, IoT devices, or high-density services
- **Systems programming**: OS components, embedded systems, or drivers
- **Security-sensitive applications**: Where memory safety issues could lead to exploits
- **High-scale concurrent systems**: Handling thousands of simultaneous connections efficiently

Rust doesn't need to replace all your C# code. Many organizations are adopting Rust strategically for specific components where its strengths provide the most benefit, while continuing to use .NET for other parts of their systems.

Throughout this workshop, you'll learn how to assess where Rust makes sense for you and your projects, and how to leverage your existing .NET knowledge to become productive in Rust.
