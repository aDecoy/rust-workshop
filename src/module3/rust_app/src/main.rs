#[tokio::main]
async fn main() {
    // Basic data types
    let str_example = "Hello";
    println!("{}", str_example);
    let string_example = "Hello".to_string();
    println!("{}", string_example);
    
    let integer_example = 10;
    println!("{}", str_example);

    let float_example = 10.0;
    println!("{}", float_example);

    let vec_example = vec![1, 2, 3];
    println!("{}", vec_example.len());

    let bool_example = true;
    println!("{}", bool_example);

    let mut mutable_string = "Hello";
    mutable_string = "Hello World";
    println!("{}", mutable_string);

    // Shadowing
    let mut str_example = "This string is now mutable";
    str_example = "And can be edited";
    println!("{}", str_example);

    // Shadowing also works inside a code block
    {
        let str_example = "This is a new value";
        println!("{}", str_example);
    }

    // The value of str_example here is still the same as before the code block
    println!("{}", str_example);


    // Mut keyword declares this variable as something that can be mutated in the future
    let mut user = User::new("James");

    user.say_hello();

    // ERROR: cannot borrow as mutable
    user.update_name("John");

    user.say_hello();

    let premium_user = user.update_to_premium();

    // Calling say_hello here will cause an error because the original instance of user has been dropped
    // user.say_hello();
    premium_user.say_hello();
    
}

struct User {
    name: String,
}

impl User {
    // &mut self is used because you want to mutate the data in this instance of the struct
    fn update_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    // &self is used because you want to reference the data of this instance, not take ownership of it. Read but not write
    fn say_hello(&self) {
        // String interpolation
        println!("Hello! I'm {}, I'm a standard user", self.name);
    }

    // Using just 'self' is a rare case where you want to take ownership of the original instance and use something new
    // calling this function will prevent the original instance from being used, as this function
    // takes ownership and then drop the original instance
    fn update_to_premium(self) -> PremiumUser {
        PremiumUser {
            name: self.name,
            is_premium: true
        }
    }

    // no 'self' at all defines a static method. Called using User::new()
    fn new(name: &str) -> User {
        User {
            name: name.to_string(),
        }
    }
}

struct PremiumUser {
    name: String,
    is_premium: bool,
}

impl PremiumUser {
    fn say_hello(&self) {
        println!("Hello! I'm {}. I'm a premium user.", self.name);
    }
}