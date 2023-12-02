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
    // First digit
    let mut start = 0;
    while start < word.len() {
        if let Some(start_char) = word.chars().nth(start) {
            if start_char.is_digit(10) {
                break;
            }
            start += 1;
        }
    }

    // Last digit
    let mut end = word.len() - 1;
    while end >= start {
        if let Some(end_char) = word.chars().nth(end) {
            if end_char.is_digit(10) {
                break;
            }
            end -= 1;
        }
    }

    if let (Some(first), Some(last)) = (word.chars().nth(start), word.chars().nth(end)) {
        let result = convert_digits_to_num(first, last);
        return Ok(result);
    }
    Err("No digits found")
}

fn convert_digits_to_num(first: char, last: char) -> u32 {
    let num1 = first.to_digit(10).unwrap();
    let num2 = last.to_digit(10).unwrap();
    num1 * 10 + num2
}
