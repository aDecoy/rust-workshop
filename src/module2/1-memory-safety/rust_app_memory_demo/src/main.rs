//
// RUST MEMORY SAFETY EXAMPLES
//
// This file demonstrates multiple aspects of Rust's memory safety features
// and provides a proper solution for safe concurrent access to shared data.
//

// These examples illustrate Rust's core memory safety principles:
// 1. Ownership - Each value has a single owner
// 2. Borrowing - References to values must follow strict rules
// 3. Lifetimes - References must not outlive the data they point to

// EXAMPLE 1: Ownership prevents use-after-move errors (uncomment to try)
/*
fn example1() {
    println!("\n--- EXAMPLE 1: OWNERSHIP PREVENTS USE-AFTER-MOVE ---");
    
    let user = User {
        name: "James".to_string(),
    };
    
    // The 'say_hello' function takes ownership of 'user'
    say_hello(user);
    
    // COMPILER ERROR: This line won't compile because 'user' was moved
    // user.update_name("John");  // Error: use of moved value
    
    // This pattern would compile fine in C# but might cause memory errors in C/C++
    // Rust prevents these errors at compile time through ownership rules
}

// This function takes ownership of the User value
fn say_hello(user: User) {
    println!("Hello, {}", user.name);
    // When this function ends, 'user' is dropped (memory freed)
}
*/

// EXAMPLE 2: Borrowing rules prevent data races (uncomment to try)
/*
fn example2() {
    println!("\n--- EXAMPLE 2: BORROWING PREVENTS DATA RACES ---");
    
    let mut user = User {
        name: "James".to_string(),
    };
    
    // Create an immutable reference to 'user'
    let name_ref = &user.name;
    
    // COMPILER ERROR: Can't borrow as mutable while already borrowed as immutable
    // user.update_name("John");  // Error: cannot borrow as mutable
    
    println!("Name is still: {}", name_ref);
    
    // After the last use of 'name_ref', we can now borrow as mutable
    user.update_name("John");
    println!("Name updated to: {}", user.name);
}
*/

// EXAMPLE 3: SAFE CONCURRENT ACCESS SOLUTION
// This example shows how to safely share mutable data between threads using
// standard Rust concurrency primitives: Arc (for sharing) and Mutex (for synchronization)

use std::sync::{Arc, Mutex};
use std::thread;

#[tokio::main]
async fn main() {
    // We'll solve the same problem from the C# example, but in a memory-safe way

    // Arc = Atomic Reference Count: Allows multiple ownership across threads
    // Mutex = Mutual Exclusion: Ensures only one thread can access the data at a time
    let user = Arc::new(Mutex::new(User {
        name: "James".to_string(),
        update_count: 0,
    }));

    println!("--- SAFE CONCURRENT ACCESS IN RUST ---");
    
    // Before modification
    {
        // Lock the mutex to access the data (this blocks until lock is acquired)
        let locked_user = user.lock().unwrap();
        println!("Starting with Name: {}, UpdateCount: {}", 
            locked_user.name, locked_user.update_count);
        // Lock is automatically released when 'locked_user' goes out of scope
    }

    // Create clones of the Arc to share ownership with multiple tasks
    let user_clone1 = Arc::clone(&user);
    let user_clone2 = Arc::clone(&user);

    // Spawn two tasks that will try to modify the user concurrently
    let handle1 = tokio::spawn(async move {
        // In a real application, you might do some async work here
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        
        // Safely modify the user by acquiring the mutex
        let mut locked_user = user_clone1.lock().unwrap();
        println!("Task 1: Updating name to 'John'");
        locked_user.name = "John".to_string();
        locked_user.update_count += 1;
        println!("Task 1: Updated count to {}", locked_user.update_count);
        // Lock is released when 'locked_user' goes out of scope
    });

    let handle2 = tokio::spawn(async move {
        // Slight delay to make concurrent access more likely
        tokio::time::sleep(tokio::time::Duration::from_millis(60)).await;
        
        // Safely modify the user by acquiring the mutex
        let mut locked_user = user_clone2.lock().unwrap();
        println!("Task 2: Updating name to 'Doe'");
        locked_user.name = "Doe".to_string();
        locked_user.update_count += 1;
        println!("Task 2: Updated count to {}", locked_user.update_count);
        // Lock is released when 'locked_user' goes out of scope
    });

    // Wait for both tasks to complete
    _ = handle1.await;
    _ = handle2.await;

    // After modification - safely access the final state
    {
        let locked_user = user.lock().unwrap();
        println!("\nFinal Name: {}, UpdateCount: {}", 
            locked_user.name, locked_user.update_count);
        println!("Unlike C#, the UpdateCount is ALWAYS 2 because Rust guarantees");
        println!("safe concurrent access through the type system and Mutex.");
    }
    
    println!("\nLESSON: Rust requires explicit handling of shared state with");
    println!("        Arc and Mutex, but guarantees thread safety at compile time.");
}

struct User {
    name: String,
    update_count: i32,
}

impl User {
    fn update_name(&mut self, name: &str) {
        println!("Updating name from '{}' to '{}'", self.name, name);
        self.name = name.to_string();
    }
}

// The key differences between Rust and C# approaches:
//
// 1. In C#, shared mutable state is the default, making data races easy to introduce
//
// 2. In Rust, sharing mutable state requires explicit use of synchronization primitives:
//    - Arc: Provides shared ownership across threads
//    - Mutex: Ensures exclusive access to the data
//
// 3. Rust's combination of ownership, borrowing, and type-based synchronization guarantees
//    thread safety at compile time, eliminating entire classes of bugs that would only
//    be caught at runtime (or not at all) in other languages