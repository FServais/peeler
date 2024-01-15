use std::collections::HashMap;

fn main() {
    // Parse command line arguments
    let command = std::env::args().nth(1).expect("missing command");
    let input_word = std::env::args().nth(2).expect("missing input word");

    println!("Command: {}", command);
    println!("Input word: {}", input_word);

    if command == "caesar" {
        let input_shift = std::env::args().nth(3).expect("missing shift");
        let cipher = caesar_cipher(&input_word, input_shift.parse::<i32>().unwrap());
        println!("Caesar cipher: {}", cipher);
    }


}

// Computes the Caesar cipher of a string
fn caesar_cipher(input: &str, shift: i32) -> String {
    let mut output = String::new();

    let alphabet = "abcdefghijklmnopqrstuvwxyz".to_string();

    let mut alphabet_index_to_char = HashMap::new();
    let mut alphabet_char_to_index = HashMap::new();

    for (i, c) in alphabet.chars().enumerate() {
        alphabet_index_to_char.insert(i, c);
        alphabet_char_to_index.insert(c, i);
    }


    for c in input.chars() {
        let i = alphabet_char_to_index.get(&c).unwrap();
        let new_i = (i + shift as usize) % 26;
        let new_c = alphabet_index_to_char.get(&new_i).unwrap();
        output.push(*new_c);
    }

    output
}
