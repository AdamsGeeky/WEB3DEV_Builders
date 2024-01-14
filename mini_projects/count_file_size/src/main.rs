use std::env;
use std::fs;

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a file path argument is provided
    if args.len() != 2 {
        println!("Usage: {} <file_path>", args[0]);
        return;
    }

    // Get the file path from the command-line argument
    let file_path = &args[1];

    // Attempt to get the metadata of the file
    match fs::metadata(file_path) {
        Ok(metadata) => {
            // Check if the path corresponds to a file
            if metadata.is_file() {
                // Get the file size in bytes
                let file_size = metadata.len();

                // Display the result
                println!("File size of {}: {} bytes", file_path, file_size);
            } else {
                println!("Error: The specified path is not a file.");
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
