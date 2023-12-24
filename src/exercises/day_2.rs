use crate::exercises::read_lines;

const NUM_REDS: i32 = 12;
const NUM_GREEN: i32 = 13;
const NUM_BLUE: i32 = 14;

pub fn run() -> Result<(), &'static str> {
    let file_path = "./src/exercises/day_2.data";
    match read_lines(file_path) {
        Ok(lines) => {
            for (i, line) in lines.enumerate() {
                if let Ok(s) = line {

                }
            }
        }
        Err(e) => eprintln!("Error opening '{}': {}", file_path, e),
    };
    Ok(())
}
