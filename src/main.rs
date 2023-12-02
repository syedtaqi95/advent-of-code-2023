use std::env;
mod exercises;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <day_number>", args[0]);
        eprintln!("Please provide the day number as a command-line argument.");
        std::process::exit(1);
    }

    let day_number: u32 = match args[1].parse() {
        Ok(number) => number,
        Err(_) => {
            eprintln!("Error: could not parse day number. Must be an integer between 1 and 25.");
            std::process::exit(1);
        }
    };

    if let Err(e) = exercises::run_day(day_number) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
