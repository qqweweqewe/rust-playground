use std::io;

fn main() {
    println!("Hi there! What's your name?");

    let mut name: String = String::new();
    io::stdin().read_line(&mut name).expect("something went wrong");

    println!("Hello, {}!", name.trim());

} 
