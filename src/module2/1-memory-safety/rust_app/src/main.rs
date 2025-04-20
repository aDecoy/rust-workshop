//
// THIS FILE DEMONSTRATES HOW RUST PREVENTS DATA RACES AT COMPILE TIME
//
// This example shows how Rust's ownership system prevents concurrent mutable access
// to the same data, making multithreaded code safer.
//

#[tokio::main]
async fn main() {
    // Create a user that we want to modify from multiple async tasks
    let mut user = User{
        name: "James".to_string(),
    };

    // This task takes ownership of 'user' using 'move'
    // The value is now owned by this async task and can no longer be accessed outside it
    let handle = tokio::spawn(async move {
        println!("First task modifying user");
        user.update_name("John");
    });

    // COMPILER ERROR! The line below will not compile because:
    // - 'user' was moved into the previous task
    // - Ownership rules prevent using a value after it's been moved
    // - This is how Rust catches data races at compile time
    let handle_2 = tokio::spawn(async move {
        println!("Second task trying to modify user");
        user.update_name("Doe"); // Error: use of moved value: `user`
    });

    // This code would allow a data race in other languages like C#
    // but Rust completely prevents it by enforcing ownership at compile time
    
    // Wait for the task to complete
    handle.await.unwrap();
    
    // This line would also cause a compiler error because 'user' was moved
    // println!("{}", user.name);
}

struct User {
    name: String,
}

impl User {
    fn update_name(&mut self, name: &str) {
        println!("Updating name from '{}' to '{}'", self.name, name);
        self.name = name.to_string();
    }
}

// LESSON: Rust's ownership system prevents data races at compile time,
// making concurrent code safer without runtime overhead.
