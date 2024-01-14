use std::env;

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a string argument is provided
    if args.len() != 2 {
        println!("Usage: {} <string>", args[0]);
        return;
    }

    // Get the input string from the command-line argument
    let input_string = &args[1];

    // Count the number of characters in the string
    let char_count = input_string.chars().count();

    // Display the result
    println!("Number of characters: {}", char_count);
}
