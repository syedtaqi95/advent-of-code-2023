use crate::exercises::read_lines;

pub fn run() -> Result<(), &'static str> {
    let file_path = "./src/exercises/day_2.data";
    match read_lines(file_path) {
        Ok(lines) => {
            for (i, line) in lines.enumerate() {
                if let Ok(s) = line {
                    println!("{}: {}\n", i, s);
                }
            }
        }
        Err(e) => eprintln!("Error opening '{}': {}", file_path, e),
    };
    Ok(())
}
