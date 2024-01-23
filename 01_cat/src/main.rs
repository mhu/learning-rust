use std::env;
use std::fs;
use std::path::Path;


fn main() {
    let args: Vec<String> = env::args().collect();

    for file_path in args.iter().skip(1) {
        if Path::new(file_path).exists() == false {
            eprintln!("{}: No such file or directory", file_path);
            std::process::exit(1);
        }
    }

    for file_path in args.iter().skip(1) {
        let contents = fs::read_to_string(file_path)
            .expect("No such file or directory");

        print!("{}", contents);
    }
}
