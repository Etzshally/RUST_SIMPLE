use std::io;

fn main() {
    // Prompt the user for input
    println!("Enter something:");

    // Create a mutable string to store the user input
    let mut user_input = String::new();

    // Read the user input
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    // Print the user input back
    println!("You entered: {}", user_input);
}
