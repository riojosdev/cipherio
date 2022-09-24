use std::io;

pub fn is_key_valid(key: &String) {
    // check if there is a substitute character for all alphabets
    if key.len() != 26 {
        panic!("Please enter 26 characters");
    } 
    
    // check if there are no repeated characters
    let mut i = 0;
    while i < key.len() {
        let mut j = i + 1;
        while j < key.len() {
            if key.chars().nth(i).unwrap() == key.chars().nth(j).unwrap() {
                panic!("Repeated character not supported!");
            }
            j += 1;
        }
        i += 1;
    }
}

pub fn ask_input() -> String {
    println!(r#"________/\\\\\\\\\______________________/\\\__________________________________________________________        "#);
    println!(r#" _____/\\\////////______________________\/\\\__________________________________________________________       "#);
    println!(r#"  ___/\\\/____________/\\\___/\\\\\\\\\__\/\\\_______________________________________/\\\_______________      "#);
    println!(r#"   __/\\\_____________\///___/\\\/////\\\_\/\\\_____________/\\\\\\\\___/\\/\\\\\\\__\///______/\\\\\____     "#);
    println!(r#"    _\/\\\______________/\\\_\/\\\\\\\\\\__\/\\\\\\\\\\____/\\\/////\\\_\/\\\/////\\\__/\\\___/\\\///\\\__    "#);
    println!(r#"     _\//\\\____________\/\\\_\/\\\//////___\/\\\/////\\\__/\\\\\\\\\\\__\/\\\___\///__\/\\\__/\\\__\//\\\_   "#);
    println!(r#"      __\///\\\__________\/\\\_\/\\\_________\/\\\___\/\\\_\//\\///////___\/\\\_________\/\\\_\//\\\__/\\\__  "#);
    println!(r#"       ____\////\\\\\\\\\_\/\\\_\/\\\_________\/\\\___\/\\\__\//\\\\\\\\\\_\/\\\_________\/\\\__\///\\\\\/___ "#);
    println!(r#"        _______\/////////__\///__\///__________\///____\///____\//////////__\///__________\///_____\/////_____"#);

    println!("Enter the text to encode: ");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Failed to read the text!");
    text
}

pub fn substitute(input: String, key: &String) -> String {
    let mut encoded = String::new();
    for letter in input.trim().chars() {
        let ascii = letter as u8;

        if ascii > 96 && ascii < 123 {
            encoded = encoded + &format!("{}", key.chars().nth((ascii - 97) as usize).unwrap().to_lowercase());
        } else if ascii > 64 && ascii < 91 {
            encoded = encoded + &format!("{}", key.chars().nth((ascii - 65) as usize).unwrap().to_uppercase());
        } else {
            encoded = encoded + &format!("{}", letter);
        }
    }
    encoded
}

#[cfg(test)]
mod tests {
    use crate::substitute;

    #[test]
    fn it_substitutes() {
        let input = String::from("Hello, Cipherio!");
        let key = String::from("qwertyuiopasdfghjklzxcvbnm");
        let encoded = substitute(input, &key);
        assert_eq!(encoded, "Itssg, Eohitkog!");
    }
}