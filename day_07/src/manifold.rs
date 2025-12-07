use crate::board::Board;
use crate::element::Element;

pub struct Manifold {
    start: (usize, usize),
    board: Board<Element>,
}

impl Manifold {
    pub fn new(board_string: &str) -> Manifold {
        let board = board_string
            .split('\n')
            .map(|row| row.chars().map(Element::new).collect())
            .collect();
        let board = Board::new(board);
        let start = board.find(Element::Start).expect("Expected start");
        Manifold { start, board }
    }

    pub fn count_rays(&self) -> usize {
        let mut rays = Board::new(vec![
            vec![0; self.board.get_number_of_columns()];
            self.board.get_number_of_rows()
        ]);
        rays.set(self.start.0 as i32, self.start.1 as i32, 1);
        let mut beams = vec![self.start];

        while let Some(mut beam) = beams.pop() {
            loop {
                let (row, column) = beam;
                if let Some(element) = self.board.get(row as i32 + 1, column as i32) {
                    let current_rays = rays.get(row as i32, column as i32).unwrap();
                    match element {
                        Element::Empty => {
                            let next_rays = rays.get((row + 1) as i32, column as i32).unwrap();
                            rays.set((row + 1) as i32, column as i32, next_rays + current_rays);
                            if next_rays == 0 {
                                beams.push((row + 1, column));
                            }
                            beams.sort();
                            beams.reverse();
                            break;
                        }
                        Element::Splitter => {
                            let rays_on_left =
                                rays.get((row + 1) as i32, (column - 1) as i32).unwrap();
                            let rays_on_right =
                                rays.get((row + 1) as i32, (column + 1) as i32).unwrap();
                            if rays_on_left == 0 {
                                beams.push((row + 1, column - 1));
                            }
                            if rays_on_right == 0 {
                                beams.push((row + 1, column + 1));
                            }
                            rays.set(
                                (row + 1) as i32,
                                (column - 1) as i32,
                                rays_on_left + current_rays,
                            );
                            rays.set(
                                (row + 1) as i32,
                                (column + 1) as i32,
                                rays_on_right + current_rays,
                            );
                            beams.sort();
                            beams.reverse();
                            break;
                        }
                        _ => (),
                    }
                    beam = (row + 1, column);
                } else {
                    break;
                }
            }
        }

        (0..self.board.get_number_of_columns())
            .map(|column| {
                rays.get((self.board.get_number_of_rows() - 1) as i32, column as i32)
                    .unwrap()
            })
            .sum::<usize>()
    }
}
