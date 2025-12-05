use core::panic;
use std::fs;

type Id = u64;
struct RangeId {
    lower: Id,
    upper: Id,
}

impl RangeId {
    pub fn new(line: &str) -> RangeId {
        let [lower, upper] = line
            .split('-')
            .map(|limit| limit.parse::<u64>().expect("Invalid ID in range"))
            .collect::<Vec<u64>>()[0..=1]
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
        let ranges = ranges
            .split('\n')
            .map(RangeId::new)
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

    pub fn count_fresh(&self) -> usize {
        self.available_ids
            .iter()
            .filter(|&id| {
                self.ranges
                    .iter()
                    .any(|range| range.lower <= *id && id <= &range.upper)
            })
            .count()
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
    let result = cafeteria.count_fresh();
    println!("Result: {}", result);
}
