const POINTER_LIMIT: u32 = 100;
const INITIAL_POINTER: u32 = 50;

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    Right,
    Left,
}

impl Direction {
    pub fn new(tag: char) -> Option<Direction> {
        match tag {
            'R' => Some(Direction::Right),
            'L' => Some(Direction::Left),
            _ => None,
        }
    }
}

pub struct Dial {
    pointer: u32,
}

impl Dial {
    pub fn new() -> Dial {
        Dial {
            pointer: INITIAL_POINTER,
        }
    }

    pub fn rotate(&mut self, dir: Direction, magnitude: u32) {
        self.pointer = match dir {
            Direction::Right => (self.pointer + magnitude) as i32,
            Direction::Left => self.pointer as i32 - magnitude as i32,
        }
        .rem_euclid(POINTER_LIMIT as i32) as u32
    }

    pub fn is_pointing_zero(&self) -> bool {
        self.pointer == 0
    }
}

#[cfg(test)]
mod tests {
    use crate::dial::{Dial, Direction};

    #[test]
    fn test_normal_add_dial() {
        let mut dial = Dial::new();
        dial.rotate(Direction::Right, 15);
        assert_eq!(dial.pointer, 65);
    }

    #[test]
    fn test_overflow_add_dial() {
        let mut dial = Dial::new();
        dial.rotate(Direction::Right, 55);
        assert_eq!(dial.pointer, 5);
    }

    #[test]
    fn test_normal_sub_dial() {
        let mut dial = Dial::new();
        dial.rotate(Direction::Left, 15);
        assert_eq!(dial.pointer, 35);
    }

    #[test]
    fn test_overflow_sub_dial() {
        let mut dial = Dial::new();
        dial.rotate(Direction::Left, 55);
        assert_eq!(dial.pointer, 95);
    }
}
