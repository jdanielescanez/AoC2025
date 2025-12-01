use super::dial::Direction;

use nom::{
    IResult, Parser,
    character::complete::{newline, one_of, u32},
    combinator::map,
    multi::many1,
    sequence::pair,
    sequence::terminated,
};

pub fn read_input(input: &str) -> IResult<&str, Vec<(Direction, u32)>> {
    many1(terminated(
        map(pair(one_of("RL"), u32), |(c, num)| {
            (
                Direction::new(c).expect("Detected direction not available"),
                num,
            )
        }),
        newline,
    ))
    .parse(input)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(
            read_input(&format!("R1\nL2\nR3\nL4\nL5\nR6\nR7\nL8\n")),
            Ok((
                "",
                vec![
                    (Direction::Right, 1),
                    (Direction::Left, 2),
                    (Direction::Right, 3),
                    (Direction::Left, 4),
                    (Direction::Left, 5),
                    (Direction::Right, 6),
                    (Direction::Right, 7),
                    (Direction::Left, 8),
                ]
            ))
        );
    }
}
