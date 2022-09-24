use std::env;
use cipherio::*;

fn main() {
    // get the command line arguments
    let args: Vec<String> = env::args().collect();
    
    // check if substitution key is provided
    if args.len() < 2 {
        panic!("selection-cipher.exe <cipher-key>");
    }

    is_key_valid(&args[1]);
    
    // get the text to encode
    let text = ask_input();
    
    // substitution
    let encoded = substitute(text, &args[1]);
    
    println!("{}", encoded);
}