#[tokio::main]
async fn main() {
    // Basic data types
    let str_example: &str = "Hello";
    println!("{}", str_example);
    let string_example: String = "Hello".to_string();
    println!("{}", string_example);
    
    let integer_example: i32 = 10;
    println!("{}", str_example);

    let float_example: f32 = 10.0;
    println!("{}", float_example);
    
    let array_example: [i32; 3] = [1, 2, 3];
    println!("{}", array_example.len());

    let vec_example: Vec<i32> = vec![1, 2, 3];
    println!("{}", vec_example.len());

    let bool_example: bool = true;
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
    let mut user = User::new("dev@jameseastham.co.uk", "James");

    user.say_hello();

    // ERROR: cannot borrow as mutable
    user.update_email_address("John");

    user.say_hello();

    let premium_user = user.update_to_premium();

    // Calling say_hello here will cause an error because the original instance of user has been dropped
    user.say_hello();
    premium_user.say_hello();
    
}

struct User {
    email_address: String,
    name: String,
}

impl User {
    // no 'self' at all defines a static method. Called using User::new()
    fn new(email_address: &str, name: &str) -> User {
        User {
            email_address: email_address.to_string(),
            name: name.to_string(),
        }
    }
    
    // &mut self is used because you want to mutate the data in this instance of the struct
    fn update_email_address(&mut self, email_address: &str) {
        self.email_address = email_address.to_string();
    }

    // &self is used because you want to reference the data of this instance, not take ownership of it. Read but not write
    fn say_hello(&self) {
        // String interpolation
        println!("Hello! I'm {}, I'm a standard user", self.email_address);
    }

    // Using just 'self' is a rare case where you want to take ownership of the original instance and use something new
    // calling this function will prevent the original instance from being used, as this function
    // takes ownership and then drop the original instance
    fn update_to_premium(self) -> PremiumUser {
        PremiumUser {
            email_address: self.email_address,
            name: self.name,
            is_premium: true
        }
    }
}

struct PremiumUser {
    email_address: String,
    name: String,
    is_premium: bool,
}

impl PremiumUser {
    fn say_hello(&self) {
        println!("Hello! I'm {}. I'm a premium user.", self.email_address);
    }
}