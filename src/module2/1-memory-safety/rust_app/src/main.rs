#[tokio::main]
async fn main() {
    let mut user = User{
        name: "James".to_string(),
    };

    let handle = tokio::spawn(async move {
        user.update_name("John");
    });

    // Compilation error, value used here after move
    let handle_2 = tokio::spawn(async move {
        user.update_name("Doe");
    });

    handle.await.unwrap();
    handle.await.unwrap();

    println!("{}", user.name);
}

struct User {
    name: String,
}

impl User {
    fn update_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}
