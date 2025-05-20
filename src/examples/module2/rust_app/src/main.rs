fn main() {
    let user = User {
        name: "James".to_string(),
    };

    say_hello(user);

    user.update_name("John");
}

fn say_hello(user: User) {
    println!("Hello, {}!", user.name);
}

struct User {
    name: String,
}

impl User {
    fn update_name(self, new_name: &str) {
        self.name = new_name.to_string();
    }
}
