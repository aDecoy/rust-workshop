// Tell the compiler that there is another module in this
// application. By convention the name of the file should match
// the name of the module

#[tokio::main]
async fn main() {
    rust_users_lib::run().await;
}
