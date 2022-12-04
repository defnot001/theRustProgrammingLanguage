use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("src/assets/codes.txt");

    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let replaced = contents.replace( ":", "");

    let codes = replaced.split_whitespace();

    for code in codes {
        match i64::from_str_radix(code, 16) {
            Ok(value) => println!("{}", value),
            Err(e) => println!("Error: {}", e)
        }
    }
}
