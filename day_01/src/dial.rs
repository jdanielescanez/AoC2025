pub const POINTER_LIMIT: u32 = 100;
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

    pub fn rotate(&mut self, dir: Direction, magnitude: u32) -> u32 {
        let is_prev_pointer_not_zero = self.pointer != 0;
        let pointer_before_rem = match dir {
            Direction::Right => (self.pointer + magnitude) as i32,
            Direction::Left => self.pointer as i32 - magnitude as i32,
        };
        self.pointer = pointer_before_rem.rem_euclid(POINTER_LIMIT as i32) as u32;

        let is_needed_correction =
            pointer_before_rem == 0 || (pointer_before_rem < 0 && is_prev_pointer_not_zero);

        let naive_division = (pointer_before_rem.abs() / POINTER_LIMIT as i32) as u32;
        if is_needed_correction {
            naive_division + 1
        } else {
            naive_division
        }
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

    #[test]
    fn test_rotate_counter() {
        let mut dial = Dial::new();
        let mut counter = 0;
        counter += dial.rotate(Direction::Left, 68);
        assert_eq!(dial.pointer, 82);
        assert_eq!(counter, 1);

        counter += dial.rotate(Direction::Left, 30);
        assert_eq!(dial.pointer, 52);
        assert_eq!(counter, 1);

        counter += dial.rotate(Direction::Right, 48);
        assert_eq!(dial.pointer, 0);
        assert_eq!(counter, 2);

        counter += dial.rotate(Direction::Left, 5);
        assert_eq!(dial.pointer, 95);
        assert_eq!(counter, 2);

        counter += dial.rotate(Direction::Right, 60);
        assert_eq!(dial.pointer, 55);
        assert_eq!(counter, 3);

        counter += dial.rotate(Direction::Left, 55);
        assert_eq!(dial.pointer, 0);
        assert_eq!(counter, 4);

        counter += dial.rotate(Direction::Left, 1);
        assert_eq!(dial.pointer, 99);
        assert_eq!(counter, 4);

        counter += dial.rotate(Direction::Left, 99);
        assert_eq!(dial.pointer, 0);
        assert_eq!(counter, 5);

        counter += dial.rotate(Direction::Right, 14);
        assert_eq!(dial.pointer, 14);
        assert_eq!(counter, 5);

        counter += dial.rotate(Direction::Left, 82);
        assert_eq!(dial.pointer, 32);
        assert_eq!(counter, 6);

        counter += dial.rotate(Direction::Right, 105);
        assert_eq!(dial.pointer, 37);
        assert_eq!(counter, 7);

        counter += dial.rotate(Direction::Left, 437);
        assert_eq!(dial.pointer, 0);
        assert_eq!(counter, 12);
    }
}
