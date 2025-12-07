use std::fs;

mod board;
mod element;
mod manifold;

use manifold::Manifold;

fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as argument");
        std::process::exit(1)
    });
    let board_string =
        fs::read_to_string(input_filename).expect("Should have been able to read the file");

    let manifold = Manifold::new(&board_string);
    let result = manifold.count_activated_splitters();
    println!("Result: {}", result);
}
