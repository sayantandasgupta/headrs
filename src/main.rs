use clap::Parser;

mod bytes;
mod lines;
mod utils;

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
            utils::read_lines(&args.file_paths, *lines);
        }
        None => match &args.bytes {
            Some(bytes) => {
                utils::read_bytes(&args.file_paths, *bytes);
            }
            None => {
                utils::read_lines(&args.file_paths, 10);
            }
        },
    }
}
