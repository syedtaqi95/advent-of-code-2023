use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub mod day_1;
pub mod day_2;

pub fn run_day(day_number: u32) -> Result<(), &'static str> {
    match day_number {
        1 => day_1::run(),
        2 => day_2::run(),
        _ => Err("Could not find exercise file. Day must be between 1 and 25."),
    }
}
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
