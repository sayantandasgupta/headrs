use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> Result<io::Lines<BufReader<File>>, String>
where
    P: AsRef<Path>,
{
    let file = File::open(&filename)
        .map_err(|e| format!("Failed to open file {}: {}", filename.as_ref().display(), e))?;

    let reader = BufReader::new(file);
    Ok(reader.lines())
}

pub fn print_num_lines(all_lines : io::Lines<BufReader<File>>, n:i32) {
    for (index, line) in all_lines.enumerate() {
        if index == (n).try_into().unwrap() {
            break;
        }

        match line {
            Ok(content) => println!("{}", content),
            Err(err) => eprintln!("Failed to Read Content : {}", err),
        }
    }
}
