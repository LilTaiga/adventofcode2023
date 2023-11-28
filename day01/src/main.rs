use std::io::{stdout, Write};

fn main() {
    print!("Type your name: ");
    stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    println!("Your name is: {}", input.trim());
}
