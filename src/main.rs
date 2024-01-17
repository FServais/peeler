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

    // Check that shift is positive
    if shift < 0 {
        return caesar_cipher(input, shift + alphabet.chars().count() as i32);
    }

    // Create maps from alphabet string
    let (alphabet_index_to_char, alphabet_char_to_index) = create_alphabet_maps(&alphabet);

    for c in input.chars() {
        let i: &usize;
        let lower_c = c.to_ascii_lowercase();
        match alphabet_char_to_index.get(&lower_c) {
            None => {
                println!("Character {} not in alphabet", c);
                output.push(c);
                continue;
            },
            _ => {
                i = alphabet_char_to_index.get(&lower_c).unwrap();
            }
        }
        let new_i = (i + shift as usize) % 26;

        match alphabet_index_to_char.get(&new_i) {
            None => {
                println!("Character at index {} not in alphabet", new_i);
                output.push(c);
                continue;
            },
            _ => {
                output.push(*alphabet_index_to_char.get(&new_i).unwrap());
            }
        }
    }

    output
}

// Creates 2 maps from an alphabet string
fn create_alphabet_maps(alphabet: &str) -> (HashMap<usize, char>, HashMap<char, usize>) {
    let mut alphabet_index_to_char = HashMap::new();
    let mut alphabet_char_to_index = HashMap::new();

    for (i, c) in alphabet.chars().enumerate() {
        alphabet_index_to_char.insert(i, c);
        alphabet_char_to_index.insert(c, i);
    }

    (alphabet_index_to_char, alphabet_char_to_index)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caesar_cipher() {
        assert_eq!(caesar_cipher("hello", 3), "khoor");
        assert_eq!(caesar_cipher("HELLO", 3), "khoor");
        assert_eq!(caesar_cipher("hello", 0), "hello");
        assert_eq!(caesar_cipher("hello", 26), "hello");
        assert_eq!(caesar_cipher("hello", 27), "ifmmp");
        assert_eq!(caesar_cipher("hello", -1), "gdkkn");
    }

    #[test]
    fn test_create_alphabet_maps() {
        let alphabet = "abcdefghijklmnopqrstuvwxyz";
        let (index_to_char, char_to_index) = create_alphabet_maps(alphabet);

        for (i, c) in alphabet.chars().enumerate() {
            assert_eq!(index_to_char.get(&i), Some(&c));
            assert_eq!(char_to_index.get(&c), Some(&i));
        }
    }
}