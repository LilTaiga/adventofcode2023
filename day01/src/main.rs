use std::io::{stdout, stdin, Write};

fn main() {
    print!("Type your name: ");
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    println!("Your name is: {}", input.trim());
}
