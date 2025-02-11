#[tokio::main]
async fn main() {
    // Mut keyword declares this variable as something that can be mutated in the future
    let mut user = User::new("James");

    user.say_hello();

    // ERROR: cannot borrow as mutable
    user.update_name("John");

    user.say_hello();

    let mut premium_user = user.update_to_premium();

    // Calling say_hello here will cause an error because the original instance of user has been dropped
    // user.say_hello();
    premium_user.say_hello();
    
    premium_user.whats_my_age();
    
    premium_user.update_age(32);

    premium_user.whats_my_age();
    
    let mut max_loops = 10;

    loop {
        if max_loops == 0 {
            break;
        }
        
        println!("Looping in a loop...{}", max_loops);
        
        max_loops = max_loops - 1;
    }

    for i in 1..10 {
        println!("Looping in a for...{}", i);
    }
}


enum User {
    Standard{name: String, age: Option<i32>},
    Premium{name: String, age: Option<i32>, is_premium: bool}
}

impl User {
    // &mut self is used because you want to mutate the data in this instance of the struct
    fn update_name(&mut self, new_name: &str) {
        match self {
            // The '*' is used to dereference the value of the variable, so you can change it.
            // De-referncing refers to accessing the underlying value the reference points to
            User::Standard { name, age } => {*name = new_name.to_string()},
            User::Premium { name, age, is_premium } => {*name = new_name.to_string()}
        } 
    }
    // &mut self is used because you want to mutate the data in this instance of the struct
    fn update_age(&mut self, new_age: i32) {
        match self {
            User::Standard { name, age } => {*age = Some(new_age)},
            User::Premium { name, age, is_premium } => {*age = Some(new_age)}
        }
    }

    // &self is used because you want to reference the data of this instance, not take ownership of it. Read but not write
    fn say_hello(&self) {
        // String interpolation
        match &self {
            User::Standard { name, age } => {
                println!("Hello! I'm {}. I'm a standard user.", name);
            }
            User::Premium { name, age, is_premium    } => {
                println!("Hello! I'm {}. I'm a premium user.", name);
            }
        }
    }
    
    // The option type is an alternative to NULL values. It's an enum that has type Some(T) or None
    fn get_age(&self) -> &Option<i32> {
        match &self {
            User::Standard { name, age } => age,
            User::Premium { name, age, is_premium } => age
        }
    }

    // The option type is an alternative to NULL values. It's an enum that has type Some(T) or None
    fn whats_my_age(&self) {
        // Everything in Rust returns a value, so you can assign a variable to the result of a match
        let users_age = match &self {
            User::Standard { name, age } => age,
            User::Premium { name, age, is_premium } => age
        };

        // If let allows you to assign a variable and have an if condition in a single line
        if let Some(age) = users_age {
            println!("I'm {} years old.", age);
        } else {  
            println!("I don't know my age.");
        } 
    }

    // Using just 'self' is a rare case where you want to take ownership of the original instance and use something new
    // calling this function will prevent the original instance from being used, as this function
    // takes ownership and then drop the original instance
    fn update_to_premium(self) -> User {
        match self {
            User::Standard { name, age } => User::Premium { name, age, is_premium: true },
            User::Premium { .. } => self
        }
    }

    // no 'self' at all defines a static method. Called using User::new()
    fn new(name: &str) -> User {
        User::Standard { name: name.to_string(), age: None }
    }
}