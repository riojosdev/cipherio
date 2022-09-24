use std::{env, io};

fn main() {
    // get the command line arguments
    let args: Vec<String> = env::args().collect();
    
    // check if substitution key is provided
    if args.len() < 2 {
        panic!("selection-cipher.exe <cipher-key>");
    }

    // check if there is a substitute character for all alphabets
    if args[1].len() != 26 {
        panic!("Please enter 26 characters");
    } 

    // check if there are no repeated characters
    let mut i = 0;
    while i < args[1].len() {
        let mut j = i + 1;
        while j < args[1].len() {
            if args[1].chars().nth(i).unwrap() == args[1].chars().nth(j).unwrap() {
                panic!("Repeated character not supported!");
            }
            j += 1;
        }
        i += 1;
    }

    // get the text to encode
    println!("Enter the text to encode: ");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Failed to read the text!");

    
}
