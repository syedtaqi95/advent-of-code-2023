use crate::exercises::read_lines;

pub fn run() -> Result<(), &'static str> {
    let file_path = "./src/exercises/day_2.data";
    let mut total: i32 = 0;

    match read_lines(file_path) {
        Ok(lines) => {
            for (_, line) in lines.enumerate() {
                if let Ok(l) = line {
                    total += calculate_score(&l).unwrap();
                }
            }
        }
        Err(e) => eprintln!("Error opening '{}': {}", file_path, e),
    };
    println!("Total = {}", total);
    Ok(())
}

fn calculate_score(line: &str) -> Result<i32, &'static str> {
    Ok(0)
}
