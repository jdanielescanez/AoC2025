use core::panic;
use std::fs;

type Id = usize;
#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Debug)]
struct RangeId {
    lower: Id,
    upper: Id,
}

impl RangeId {
    pub fn new(line: &str) -> RangeId {
        let [lower, upper] = line
            .split('-')
            .map(|limit| limit.parse::<Id>().expect("Invalid ID in range"))
            .collect::<Vec<Id>>()[0..=1]
        else {
            panic!("Invalid range")
        };
        RangeId { lower, upper }
    }
}

struct Cafeteria {
    ranges: Vec<RangeId>,
    available_ids: Vec<Id>,
}

impl Cafeteria {
    pub fn new(cafeteria_string: String) -> Self {
        let [ranges, available_ids] = cafeteria_string.split("\n\n").collect::<Vec<&str>>()[0..=1]
        else {
            panic!("Invalid cafeteria format")
        };
        let mut raw_ranges = ranges
            .split('\n')
            .map(RangeId::new)
            .collect::<Vec<RangeId>>();
        raw_ranges.sort();

        let ranges = raw_ranges
            .clone()
            .into_iter()
            .filter(|&range| {
                !raw_ranges.iter().any(|&compared_range| {
                    compared_range != range
                        && compared_range.lower <= range.lower
                        && range.lower <= compared_range.upper
                        && compared_range.lower <= range.upper
                        && range.upper <= compared_range.upper
                })
            })
            .collect::<Vec<RangeId>>();
        let available_ids = available_ids
            .split('\n')
            .map(|id| id.parse::<Id>().expect("Invalid available ID"))
            .collect::<Vec<Id>>();
        Cafeteria {
            ranges,
            available_ids,
        }
    }

    pub fn count_available_fresh(&self) -> usize {
        self.available_ids
            .iter()
            .filter(|&id| {
                self.ranges
                    .iter()
                    .any(|range| range.lower <= *id && id <= &range.upper)
            })
            .count()
    }

    pub fn count_range_fresh(&self) -> usize {
        let ranges_to_compare = (0..self.ranges.len()).map(|i| {
            (
                self.ranges[i],
                self.ranges
                    .iter()
                    .enumerate()
                    .filter(move |&(j, _)| i < j)
                    .map(|(_, range)| range),
            )
        });

        ranges_to_compare
            .into_iter()
            .map(|(range, rest)| {
                let upper = rest
                    .filter(|compared_range| {
                        compared_range.lower <= range.upper && range.upper <= compared_range.upper
                    })
                    .map(|container_range| container_range.lower - 1)
                    .min()
                    .unwrap_or(range.upper);
                RangeId {
                    lower: range.lower,
                    upper,
                }
            })
            .map(|range| range.upper + 1 - range.lower)
            .sum()
    }
}

fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as argument");
        std::process::exit(1)
    });
    let cafeteria_string =
        fs::read_to_string(input_filename).expect("Should have been able to read the file");

    let cafeteria = Cafeteria::new(cafeteria_string);
    let result_part_1 = cafeteria.count_available_fresh();
    let result_part_2 = cafeteria.count_range_fresh();
    println!("Result part 1: {}", result_part_1);
    println!("Result part 2: {}", result_part_2);
}
