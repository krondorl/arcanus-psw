use rand::Rng;

const VOWELS: [&str; 5] = ["a", "e", "i", "o", "u"];
const CONSONANTS: [&str; 21] = [
    "b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v", "w", "x",
    "y", "z",
];
const NUMBERS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
const SPECIALS: [&str; 6] = ["!", "+", "#", "/", "$", "?"];

fn generate_words(length: u8) -> Result<String, String> {
    if (13..=64).contains(&length) {
        let mut generated_words: String = Default::default();
        let mut i = 0;
        while i < length {
            if i % 2 == 0 {
                let random_consonant = rand::thread_rng().gen_range(0..20);
                if i == 0 || i % 4 == 0 {
                    let uppercase = str::to_uppercase(CONSONANTS[random_consonant]);
                    generated_words.push_str(&uppercase);
                } else {
                    generated_words.push_str(CONSONANTS[random_consonant]);
                }
            } else {
                let random_vowel = rand::thread_rng().gen_range(0..4);
                generated_words.push_str(VOWELS[random_vowel]);
            }
            i += 1;
        }
        Ok(generated_words)
    } else {
        Err(String::from(
            "Error by generating words: length parameter should be between 13 and 64.",
        ))
    }
}

fn generate_numbers(length: u8) -> Result<String, String> {
    if (1..=4).contains(&length) {
        let mut generated_numbers: String = Default::default();
        let mut i = 0;
        while i < length {
            let random_number = rand::thread_rng().gen_range(0..9);
            generated_numbers.push_str(NUMBERS[random_number]);
            i += 1;
        }
        Ok(generated_numbers)
    } else {
        Err(String::from(
            "Error by generating words: length parameter should be between 1 and 4.",
        ))
    }
}

fn generate_specials() -> String {
    let random_special = rand::thread_rng().gen_range(0..5);
    SPECIALS[random_special].to_string()
}

fn generate_password(length: Option<u8>) -> Result<String, String> {
    match length {
        Some(_) => todo!(),
        None => {
            let mut generated_password: String = Default::default();
            let words = generate_words(13);
            match words {
                Ok(words_value) => {
                    generated_password.push_str(&words_value);
                    let numbers = generate_numbers(2);
                    match numbers {
                        Ok(numbers_value) => {
                            generated_password.push_str(&numbers_value);
                            let specials = generate_specials();
                            generated_password.push_str(&specials);
                            Ok(generated_password)
                        }
                        Err(e) => Err(e),
                    }
                }
                Err(e) => Err(e),
            }
        }
    }
}

fn main() {
    println!("Arcanus password generator");
    let w = generate_words(13);
    println!("{:#?}", w);
    let n = generate_numbers(4);
    println!("{:#?}", n);
    let s = generate_specials();
    println!("{:#?}", s);
    let p = generate_password(None).unwrap();
    println!("{:#?}", p);
}

#[cfg(test)]
mod tests {
    use crate::{generate_numbers, generate_password, generate_specials, generate_words};
    use regex::Regex;

    #[test]
    fn test_generate_words_13() {
        let words_length = 13;
        let generated_words = generate_words(words_length);
        match generated_words {
            Ok(words) => {
                assert_eq!(words.len(), words_length.into());
                let pattern = Regex::new(r"([a-zA-Z]){1,64}").unwrap();
                assert!(pattern.is_match(&words));
            }
            Err(_e) => panic!(),
        }
    }

    #[test]
    fn test_generate_words_0() {
        let words_length = 0;
        let generated_words = generate_words(words_length);
        match generated_words {
            Ok(_words) => panic!(),
            Err(e) => assert_eq!(
                e,
                String::from(
                    "Error by generating words: length parameter should be between 13 and 64.",
                )
            ),
        }
    }

    #[test]
    fn test_generate_numbers_1() {
        let numbers_length = 1;
        let generated_numbers = generate_numbers(numbers_length);
        match generated_numbers {
            Ok(numbers) => {
                assert_eq!(numbers.len(), numbers_length.into());
                let pattern = Regex::new(r"([0-9]){1,64}").unwrap();
                assert!(pattern.is_match(&numbers));
            }
            Err(_) => panic!(),
        }
    }

    #[test]
    fn test_generate_numbers_0() {
        let numbers_length = 0;
        let generated_numbers = generate_numbers(numbers_length);
        match generated_numbers {
            Ok(_numbers) => panic!(),
            Err(e) => assert_eq!(
                e,
                String::from(
                    "Error by generating words: length parameter should be between 1 and 4.",
                )
            ),
        }
    }

    #[test]
    fn test_generate_specials() {
        let generated_specials = generate_specials();
        assert_eq!(generated_specials.len(), 1);
        let pattern = Regex::new(r"([!+#/$?])").unwrap();
        assert!(pattern.is_match(&generated_specials));
    }

    #[test]
    fn test_generate_password_none() {
        let generated_password = generate_password(None).unwrap();
        assert_eq!(generated_password.len(), 16);
    }
}
