use chrono::prelude::*;

fn main() {
    let mut name = String::new();
    println!("Enter product name");

    if name == "" {
        std::io::stdin().read_line(&mut name).unwrap();
    }
    println!("{name}");
}
