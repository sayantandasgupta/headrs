use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn read_lines<P>(filename: P, num_lines: i32) -> Result<Vec<String>, String>
where
    P: AsRef<Path>,
{
    let file = File::open(&filename)
        .map_err(|e| format!("Failed to open file {}: {}", filename.as_ref().display(), e))?;

    let reader = BufReader::new(file);
    let all_lines = reader.lines();

    let mut result: Vec<String> = Vec::new();
    let mut count: i32 = 0;
    for line in all_lines {
        if count == num_lines {
            break;
        }
        match line {
            Ok(content) => {
                count += 1;
                result.push(content)
            }
            Err(err) => return Err(format!("Failed to read line: {}", err)),
        }
    }

    Ok(result)
}
