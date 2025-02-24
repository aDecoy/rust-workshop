// EXAMPLE 1
#[tokio::main]
async fn main() {
    let user = User{
        name: "James".to_string(),
    };

    // Say hello function takes ownership of the user variable
    say_hello(user);

    // You are now trying to use a variable that has been dropped from memory
    user.update_name("John");

    say_hello(user);
    
}

// The say hello function takes 'ownership' of the user variable, and it's underlying memory
// When a variable goes out of scope, the value is 'dropped' from memory
fn say_hello(user: User) {
    println!("Hello, {}", user.name);
}

struct User {
    name: String,
}

impl User {
    fn update_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}


// EXAMPLE 2
// #[tokio::main]
// async fn main() {
//     let user = User{
//         name: "James".to_string(),
//     };

//     // Ampersand denotes a borrow
//     say_hello(&user);

//     // ERROR: cannot borrow as mutable
//     user.update_name("John");

//     say_hello(&user);
    
// }

// // The '&' indicates that the variable is only borrowed, which means the ownership stays with the caller
// fn say_hello(user: &User) {
//     println!("Hello, {}", user.name);
// }

// struct User {
//     name: String,
// }

// impl User {
//     fn update_name(&mut self, name: &str) {
//         self.name = name.to_string();
//     }
// }

// #[tokio::main]
// async fn main() {
//     // Mut keyword declares this variable as something that can be mutated in the future
//     let mut user = User{
//         name: "James".to_string(),
//     };
// 
//     say_hello(&user);
// 
//     // ERROR: cannot borrow as mutable
//     user.update_name("John");
// 
//     say_hello(&user);
//     
// }
// 
// fn say_hello(user: &User) {
//     println!("Hello, {}", user.name);
// }
// 
// struct User {
//     name: String,
// }
// 
// impl User {
//     // Function signature also is explicit about needing a reference to a variable that is also mutable
//     fn update_name(&mut self, name: &str) {
//         self.name = name.to_string();
//     }
// }