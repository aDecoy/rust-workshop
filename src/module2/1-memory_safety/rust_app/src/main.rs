#[tokio::main]
async fn main() {
    let user = User{
        name: "James".to_string(),
    };

    let handle = tokio::spawn(async move {
        user.update_name("John").await;
    });

    // Compilation error, value used here after move
    let handle_2 = tokio::spawn(async move {
        user.update_name("Doe").await;
    });

    handle.await.unwrap();
    handle.await.unwrap();

    println!("{}", user.name);
}

struct User {
    name: String,
}

impl User {
    async fn update_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}
