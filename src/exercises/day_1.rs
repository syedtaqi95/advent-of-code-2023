use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() -> Result<(), &'static str> {
    let mut result = 0;
    let file_path = "./src/exercises/day_1.data";
    match read_lines(file_path) {
        Ok(lines) => {
            for line in lines {
                if let Ok(word) = line {
                    let value = get_calibration_value(&word)?;
                    result += value;
                    println!("{}: {}", word, value);
                }
            }

            println!("Sum = {}", result);
        }
        Err(e) => eprintln!("Error opening '{}': {}", file_path, e),
    };
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_calibration_value(word: &str) -> Result<u32, &'static str> {
    // Find first digit
    let digit_map: HashMap<String, u32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .map(|&(key, value)| (key.to_string(), value))
    .collect();
    let start = find_first_digit_in_word(word, &digit_map).unwrap();

    // Reverse the word and digit map
    let reversed_word: &str = &word.chars().rev().collect::<String>();
    let mut reversed_digit_map: HashMap<String, u32> = HashMap::new();
    for (key, value) in &digit_map {
        let reversed_key_string: String = key.chars().rev().collect::<String>();
        reversed_digit_map.insert(reversed_key_string, *value);
    }

    // Find last digit
    let end = find_first_digit_in_word(reversed_word, &reversed_digit_map).unwrap();

    // Result
    Ok(10 * start + end)
}

fn find_first_digit_in_word(word: &str, digit_map: &HashMap<String, u32>) -> Option<u32> {
    for (i, c) in word.chars().enumerate() {
        if c.is_digit(10) {
            // Digit found
            return Some(c.to_digit(10).unwrap());
        } else {
            // Check if substring is a spelled out digit
            for (digit_word, digit) in digit_map {
                let digit_word_len: usize = digit_word.chars().count();
                if let Some(substr) = word.get(i..i + digit_word_len) {
                    if substr == *digit_word {
                        return Some(*digit);
                    }
                }
            }
        }
    }

    None
}
