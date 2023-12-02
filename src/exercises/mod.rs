pub mod day_1;

pub fn run_day(day_number: u32) -> Result<(), &'static str> {
    match day_number {
        1 => day_1::run(),
        _ => Err("Could not find exercise file. Day must be between 1 and 25."),
    }
}
