use std::path::Path;

use crate::bytes;
use crate::lines;

pub fn read_lines(file_paths: &Vec<String>, num_lines: i32) {
    for file in file_paths {
        let file_name = Path::new(file).file_name().unwrap().to_str().unwrap();

        if file_paths.len() > 1 {
            println!("==> {} <== ", file_name);
        }

        match lines::read_lines(file, num_lines) {
            Ok(lines) => {
                for line in lines {
                    println!("{}", line);
                }
            }
            Err(e) => eprintln!("Failed to extract lines: {}", e),
        }
    }
}

pub fn read_bytes(file_paths: &Vec<String>, num_bytes: usize) {
    for file in file_paths {
        let file_name = Path::new(file).file_name().unwrap().to_str().unwrap();

        if file_paths.len() > 1 {
            println!("==> {} <== ", file_name);
        }

        match bytes::read_bytes(file, num_bytes) {
            Ok(bytes) => println!("{}", bytes),
            Err(e) => eprintln!("Failed to extract bytes: {}", e),
        }
    }
}
