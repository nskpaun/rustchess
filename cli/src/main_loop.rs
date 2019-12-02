use std::io;

pub fn execute_main_loop() {
    println!("Welcome to chess in Rust!");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("Your input: {}", guess);
}