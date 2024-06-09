use std::fs::File;
use std::io::{self, Read};

fn main() {
    println!("Please enter the file name:");

    // Read the file name from standard input
    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name).expect("Failed to read line");

    // Remove any trailing newline or carriage return
    let file_name = file_name.trim();

    // Attempt to open the file
        match File::open(file_name) {
            Ok(mut file) => {
            // File opened successfully, read and print its contents
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Failed to read the file");
            println!("File contents:\n{}", contents);
        }
        Err(error) => {
            // Error opening the file, print a friendly error message
            eprintln!("Error opening the file: {}", error);
        }
    }
}
