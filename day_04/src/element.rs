#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Element {
    Empty,
    Roll,
}

impl Element {
    pub fn new(c: char) -> Self {
        match c {
            '.' => Element::Empty,
            '@' => Element::Roll,
            invalid_char => panic!("Invalid char: {invalid_char}"),
        }
    }
}
