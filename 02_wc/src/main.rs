use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("No file specified");
        std::process::exit(1);
    }

    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("No such file or directory");

    let newline_count = contents.matches('\n').count();
    let word_count = contents
        .split(|c| c == ' ' || c == '\n')
        .filter(|s| !s.is_empty())
        .count();
    let byte_count = contents.chars().count();

    println!(
        "{:3} {:3} {:3} {}",
        newline_count, word_count, byte_count, file_path
    );
}
