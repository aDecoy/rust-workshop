fn main() {
    let mut user = User {
        name: "James".to_string(),
    };

    say_hello(&user);

    user.update_name("John");

    say_hello(&user);
}

fn say_hello(user: &User) {
    println!("Hello, {}!", user.name);
}

struct User {
    name: String,
}

impl User {
    fn update_name(&mut self, new_name: &str) -> Result<(), String> {
        if new_name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }

        self.name = new_name.to_string();

        Ok(())
    }
}
