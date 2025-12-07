use std::collections::HashSet;

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

    pub fn count_activated_splitters(&self) -> usize {
        let mut board = self.board.clone();
        let mut beams = vec![self.start];
        let mut marked_splitters = HashSet::new();
        while let Some(mut beam) = beams.pop() {
            loop {
                let (row, column) = beam;
                if let Some(element) = board.get(row as i32 + 1, column as i32) {
                    beam = (row + 1, column);
                    match element {
                        Element::Empty => {
                            board.set(row as i32, column as i32, Element::Beam);
                        }
                        Element::Beam => {
                            break;
                        }
                        Element::Splitter => {
                            marked_splitters.insert(beam);
                            beams.push((row, column - 1));
                            beams.push((row, column + 1));
                            break;
                        }
                        _ => (),
                    }
                } else {
                    break;
                }
            }
        }
        marked_splitters.len()
    }
}
