use std::path::Path;

use clap::Parser;

mod bytes;
mod lines_utils;

#[derive(Parser, Debug)]
#[command(
    version = "1.0",
    about = "Print the first 10 lines of each FILE to standard output"
)]
struct Cli {
    // Path of the file
    file_paths: Vec<String>,

    #[arg(
        short = 'c',
        long = "bytes",
        help = "print the first NUM bytes of each file;\n with the leading '-', print all but the last\nNUM lines of each file"
    )]
    bytes: Option<usize>,

    #[arg(
        short = 'n',
        long = "lines",
        help = "print the first NUM lines instead of the first 10;\n with the leading '-', print all but the last \n NUM lines of each file"
    )]
    lines: Option<i32>,
}

fn main() {
    let mut args = Cli::parse();

    match &args.lines {
        Some(lines) => {
            args.bytes = None;
            for file in &args.file_paths {
                let file_name = Path::new(file).file_name().unwrap().to_str().unwrap();

                println!("==> {} <== ", file_name);

                lines_utils::get_lines(file, *lines)
            }
        }
        None => match &args.bytes {
            Some(bytes) => {
                for file in &args.file_paths {
                    let file_name = Path::new(file).file_name().unwrap().to_str().unwrap();

                    println!("==> {} <== ", file_name);

                    match bytes::read_bytes(file, *bytes) {
                        Ok(bytes) => println!("{}", bytes),
                        Err(e) => eprintln!("Failed to extract bytes: {}", e),
                    }
                }
            }
            None => {
                for file in &args.file_paths {
                    let file_name = Path::new(file).file_name().unwrap().to_str().unwrap();

                    println!("==> {} <== ", file_name);

                    lines_utils::get_lines(file, 10)
                }
            }
        },
    }
}
