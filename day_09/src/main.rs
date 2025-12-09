use std::fs;

#[derive(PartialEq, Copy, Clone, PartialOrd)]
struct TilePosition {
    row: usize,
    column: usize,
}

impl TilePosition {
    pub fn new(line: &str) -> TilePosition {
        let [row, column] = line
            .split(',')
            .map(|coordinate| {
                coordinate
                    .parse::<usize>()
                    .expect("Coordinate must be a non negative number")
            })
            .collect::<Vec<usize>>()[..2]
        else {
            unreachable!()
        };
        TilePosition { row, column }
    }

    pub fn get_rectangle_area(&self, other: TilePosition) -> usize {
        (self.row.abs_diff(other.row) + 1) * (self.column.abs_diff(other.column) + 1)
    }
}

fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as argument");
        std::process::exit(1)
    });
    let theater_string =
        fs::read_to_string(input_filename).expect("Should have been able to read the file");

    let positions = theater_string.split('\n').map(TilePosition::new);
    let areas = positions.clone().flat_map(|i| {
        positions.clone().filter_map(move |j| {
            if i < j {
                Some(i.get_rectangle_area(j))
            } else {
                None
            }
        })
    });

    let result = areas.max().unwrap();
    println!("Result: {}", result);
}
