// Copyright (c) 2023 Adam Burucs. MIT license.

use rand::prelude::*;
/**
 * A cryptographically secure random number generator that uses the ChaCha algorithm.
 * https://crates.io/crates/rand_chacha
 * https://rust-random.github.io/rand/rand_chacha/
 */
use rand_chacha::ChaCha20Rng;
use std::{
    fs::{self, File},
    io::{self, BufRead},
    path::Path,
};

#[cfg(windows)]
const NL: &str = "\r\n";
#[cfg(not(windows))]
const NL: &str = "\n";

const VOWELS: [&str; 5] = ["a", "e", "i", "o", "u"];
const CONSONANTS: [&str; 21] = [
    "b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v", "w", "x",
    "y", "z",
];
const NUMBERS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
const SPECIALS: [&str; 6] = ["!", "+", "#", "/", "$", "?"];

// Size of character set. Used for entropy calculation.
// Multiplier of 2 used because of lower and uppercase chars.
const SET_SIZE: u8 =
    (VOWELS.len() as u8 + CONSONANTS.len() as u8) * 2 + NUMBERS.len() as u8 + SPECIALS.len() as u8;

#[derive(Debug, PartialEq)]
enum PasswordStrength {
    VeryWeak,
    Weak,
    Strong,
    VeryStrong,
}

#[derive(Debug, PartialEq)]
struct Entropy {
    bits: u8,
    strength: PasswordStrength,
}

trait InRange {
    fn in_range(self, begin: Self, end: Self) -> bool;
}

impl InRange for f64 {
    fn in_range(self, begin: f64, end: f64) -> bool {
        self >= begin && self < end
    }
}

fn generate_words(length: u8) -> Result<String, String> {
    if (13..=64).contains(&length) {
        let mut generated_words: String = Default::default();
        let mut i = 0;
        while i < length {
            let mut rng = ChaCha20Rng::from_entropy();
            if i % 2 == 0 {
                let random_consonant = rng.gen_range(0..20);
                if i == 0 || i % 4 == 0 {
                    let uppercase = str::to_uppercase(CONSONANTS[random_consonant]);
                    generated_words.push_str(&uppercase);
                } else {
                    generated_words.push_str(CONSONANTS[random_consonant]);
                }
            } else {
                let random_vowel = rng.gen_range(0..4);
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
            let mut rng = ChaCha20Rng::from_entropy();
            let random_number = rng.gen_range(0..9);
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
    let mut rng = ChaCha20Rng::from_entropy();
    let random_special = rng.gen_range(0..5);
    SPECIALS[random_special].to_string()
}

fn generate_password(length: Option<u8>) -> Result<String, String> {
    match length {
        Some(length_value) => {
            if (16..=64).contains(&length_value) {
                let mut generated_password: String = Default::default();
                let words = generate_words(length_value - 3);
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
            } else {
                Err(String::from(
                    "Error: generate password should have a length between 16 and 64.",
                ))
            }
        }
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

fn generate_list(count: Option<u8>) -> Result<Vec<String>, String> {
    match count {
        Some(count_value) => {
            if (16..=255).contains(&count_value) {
                let mut list: Vec<String> = Vec::new();
                for _n in 0..count_value {
                    let generated_password = generate_password(None);
                    match generated_password {
                        Ok(psw_value) => list.push(psw_value),
                        Err(e) => return Err(e),
                    }
                }
                Ok(list)
            } else {
                Err(String::from(
                    "Error: generate list should have a count between 16 and 255.",
                ))
            }
        }
        None => {
            let mut list: Vec<String> = Vec::new();
            for _n in 0..16 {
                let generated_password = generate_password(None);
                match generated_password {
                    Ok(psw_value) => list.push(psw_value),
                    Err(e) => return Err(e),
                }
            }
            Ok(list)
        }
    }
}

fn save_list(list: Vec<String>, file_name: String) -> std::io::Result<()> {
    fs::write(file_name, list.join(NL)).expect("Error: couldn't write passwords to file.");
    Ok(())
}

fn read_list_util<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_list(filename: String) -> Result<Vec<String>, String> {
    if let Ok(lines) = read_list_util(filename) {
        let mut vec = vec![];
        for line in lines.flatten() {
            vec.push(line);
        }
        Ok(vec)
    } else {
        Err(String::from("Error: cannot read file."))
    }
}

// Note: this is just a rough, simplified calculation only.
// Most of the characters in the generated password are alphabetic.
// So that is only a set size of 26 * 2 = 52.
//
// Todo: modify algorithm to calculate correct entropy.
fn check_entropy(password: String) -> Result<Entropy, String> {
    if password.len() >= 16 {
        let password_length = password.len();
        let bits = (u128::pow(SET_SIZE.into(), password_length as u32) as f64)
            .log2()
            .round();
        let strength: PasswordStrength = match bits {
            x if x.in_range(0.0, 35.0) => PasswordStrength::VeryWeak,
            x if x.in_range(36.0, 59.0) => PasswordStrength::Weak,
            x if x.in_range(60.0, 119.0) => PasswordStrength::Strong,
            x if x.in_range(120.0, 512.0) => PasswordStrength::VeryStrong,
            _ => PasswordStrength::VeryWeak,
        };
        Ok(Entropy {
            bits: bits as u8,
            strength,
        })
    } else {
        Err(String::from(
            "Error when checking entropy: password length should be at least 16 characters.",
        ))
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
    let p = generate_password(None);
    match p {
        Ok(ok_p) => {
            println!("{:#?}", ok_p);
        }
        Err(e) => {
            println!("Error {:#?}", e);
        }
    }
    let p32 = generate_password(Some(32));
    match p32 {
        Ok(ok_p32) => {
            println!("{:#?}", ok_p32);
        }
        Err(e) => {
            println!("Error {:#?}", e);
        }
    }
    let ln = generate_list(None);
    match ln {
        Ok(ok_ln) => {
            println!("{:#?}", ok_ln);
        }
        Err(e) => {
            println!("Error {:#?}", e);
        }
    }
    let ls = generate_list(Some(32));
    match ls {
        Ok(ok_ls) => {
            println!("{:#?}", ok_ls);
            let _sl = save_list(ok_ls, String::from("passwords.txt"));
            let rl = read_list(String::from("passwords.txt"));
            match rl {
                Ok(_) => println!("File read ok"),
                Err(e) => println!("{e}"),
            }
        }
        Err(e) => {
            println!("Error {:#?}", e);
        }
    }

    let ce = check_entropy(String::from("HokiTiwoYaloM83#"));
    match ce {
        Ok(ok_ce) => {
            println!();
            println!("Check entropy");
            println!("{:#?}", ok_ce);
        }
        Err(e) => {
            println!("Error {:#?}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        check_entropy, generate_list, generate_numbers, generate_password, generate_specials,
        generate_words, Entropy, PasswordStrength::Strong,
    };
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

    #[test]
    fn test_generate_password_some() {
        let generated_password = generate_password(Some(32)).unwrap();
        assert_eq!(generated_password.len(), 32);
        let pattern = Regex::new(r"([a-zA-Z0-9!+#/$?]){16,64}").unwrap();
        assert!(pattern.is_match(&generated_password));
    }

    #[test]
    fn test_generate_list_none() {
        let generated_list = generate_list(None).unwrap();
        assert_eq!(generated_list.len(), 16);
    }

    #[test]
    fn test_generate_list_some() {
        let generated_list = generate_list(Some(32)).unwrap();
        assert_eq!(generated_list.len(), 32);
    }

    #[test]
    fn test_check_entropy_ok() {
        let password: String = String::from("HokiTiwoYaloM83#");
        let entropy = check_entropy(password);
        assert_eq!(
            entropy,
            Ok(Entropy {
                bits: 97,
                strength: Strong,
            })
        );
    }

    #[test]
    fn test_check_entropy_err() {
        let password: String = String::from("toka");
        let entropy = check_entropy(password);
        assert_eq!(
            entropy,
            Err(String::from(
                "Error when checking entropy: password length should be at least 16 characters.",
            ))
        );
    }
}
