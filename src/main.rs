use clap::Parser;

mod lines_utils;

#[derive(Parser, Debug)]
#[command(
    version = "1.0",
    about = "Print the first 10 lines of each FILE to standard output"
)]
struct Cli {
    // Path of the file
    file_path: String,

    #[arg(
        short = 'n',
        long = "lines",
        help = "print the first NUM lines instead of the first 10;\n with the leading '-', print all but the last\n NUM lines of each file"
    )]
    lines: Option<i32>,
}

fn main() {
    let args = Cli::parse();

    match lines_utils::read_lines(&args.file_path) {
        Ok(all_lines) => match &args.lines {
            Some(num_lines) => lines_utils::print_num_lines(all_lines, *num_lines),
            None => lines_utils::print_num_lines(all_lines, 10),
        },
        Err(error) => eprintln!("{}", error),
    }
}
