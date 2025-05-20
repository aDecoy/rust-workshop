use chrono::Local;

fn main() {
    let name = "James";
    let local_time = Local::now();

    println!(
        "Hello, my name is {} and the current time is {}",
        name, local_time
    );
}
