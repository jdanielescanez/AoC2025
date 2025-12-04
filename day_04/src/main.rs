use std::fs;

mod board;
mod element;

use board::Board;
use element::Element;

fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as argument");
        std::process::exit(1)
    });
    let board_string =
        fs::read_to_string(input_filename).expect("Should have been able to read the file");

    let board = board_string
        .split('\n')
        .map(|row| row.chars().map(Element::new).collect())
        .collect();

    let mut board = Board::new(board);
    let n = board.get_number_of_rows();
    let m = board.get_number_of_columns();
    let mut result = 0;
    loop {
        let positions = (0..n).flat_map(move |row| (0..m).map(move |column| (row, column)));

        let current_board = board.clone();
        let positions_to_remove = positions.filter(|(row, column)| {
            current_board
                .get(*row as i32, *column as i32)
                .is_some_and(|element| element == Element::Roll)
                && current_board
                    .get_neighbours(*row as i32, *column as i32)
                    .into_iter()
                    .filter(|&neighbour| neighbour == Element::Roll)
                    .count()
                    < 4
        });
        let current_result = positions_to_remove.clone().count();
        if current_result == 0 {
            break;
        }
        result += positions_to_remove.clone().count();

        for (row, column) in positions_to_remove {
            board.set(row as i32, column as i32, Element::Empty);
        }
    }

    println!("Result: {}", result);
}
