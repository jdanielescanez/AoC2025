#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Element {
    Start,
    Empty,
    Splitter,
    Beam,
}

impl Element {
    pub fn new(c: char) -> Self {
        match c {
            'S' => Element::Start,
            '.' => Element::Empty,
            '^' => Element::Splitter,
            invalid_char => panic!("Invalid char: {invalid_char}"),
        }
    }
}
