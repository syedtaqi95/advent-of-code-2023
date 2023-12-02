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
                    println!("{}: {}", word, value)
                }
            }

            println!("Result = {}", result);
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
    let mut chars = word.chars();
    let (mut first_digit, mut last_digit) = (None, None);

    while let Some(c) = chars.next() {
        if c.is_digit(10) {
            if first_digit.is_none() {
                first_digit = Some(c);
            }
            last_digit = Some(c);
        }
    }

    if let (Some(first), Some(last)) = (first_digit, last_digit) {
        let num1 = first.to_digit(10).unwrap();
        let num2 = last.to_digit(10).unwrap();
        let result = num1 * 10 + num2;
        return Ok(result);
    }

    Err("No digit found")
}
