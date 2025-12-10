use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{one_of, u16},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::delimited,
};

fn parse_state(input: &str) -> IResult<&str, u16> {
    delimited(
        tag("["),
        map(many1(one_of(".#")), |symbols| {
            symbols
                .iter()
                .enumerate()
                .filter_map(|(i, &symbol)| if symbol == '#' { Some(i as u16) } else { None })
                .map(|i| 2_u32.pow(i as u32) as u16)
                .sum::<u16>()
        }),
        tag("]"),
    )
    .parse(input)
}

fn parse_button(input: &str) -> IResult<&str, u16> {
    delimited(
        tag("("),
        map(separated_list1(tag(","), u16), |button| {
            button
                .into_iter()
                .map(|i| 2_u32.pow(i as u32) as u16)
                .sum::<u16>()
        }),
        tag(")"),
    )
    .parse(input)
}

fn parse_buttons(input: &str) -> IResult<&str, Vec<u16>> {
    separated_list1(tag(" "), parse_button).parse(input)
}

fn parse_joltages(input: &str) -> IResult<&str, Vec<u16>> {
    delimited(tag("{"), separated_list1(tag(","), u16), tag("}")).parse(input)
}

type Machine = (u16, Vec<u16>, Vec<u16>);
pub fn read_input(input: &str) -> IResult<&str, Vec<Machine>> {
    separated_list1(
        tag("\n"),
        (
            parse_state,
            tag(" "),
            parse_buttons,
            tag(" "),
            parse_joltages,
        )
            .map(|(state, _, buttons, _, joltages)| (state, buttons, joltages)),
    )
    .parse(input)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(
            read_input(&format!(
                "{}\n{}",
                "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
                "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}"
            )),
            Ok((
                "",
                vec![
                    (6, vec![8, 10, 4, 12, 5, 3], vec![3, 5, 4, 7]),
                    (2, vec![29, 12, 17, 7, 30], vec![7, 5, 12, 7, 2])
                ]
            ))
        );
    }
}
