mod dial;
mod parser;
use std::fs;

use dial::Dial;
use parser::read_input;

fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as argument");
        std::process::exit(1)
    });
    let dial_string =
        fs::read_to_string(input_filename).expect("Should have been able to read the file");
    let (_, instructions) = read_input(&dial_string).unwrap();

    let mut dial = Dial::new();
    let result = instructions
        .into_iter()
        .fold(0, |acc, (dir, magnitude)| acc + dial.rotate(dir, magnitude));
    println!("Result: {}", result);
}
