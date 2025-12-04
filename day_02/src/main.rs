use std::fs;

#[derive(Debug)]
struct IdRange {
    first: u64,
    last: u64,
}

fn log10(x: u64) -> f64 {
    (x as f64).log10()
}

fn split_by_number_of_digits(ranges: Vec<IdRange>) -> Vec<IdRange> {
    ranges
        .into_iter()
        .flat_map(|range| {
            let IdRange { first, last } = range;
            let first_digits = log10(first).floor() as u32;
            let last_digits = log10(last).floor() as u32;
            if first_digits != last_digits {
                let next_power_of_ten = 10_u64.pow(first_digits + 1);
                split_by_number_of_digits(vec![
                    IdRange {
                        first,
                        last: next_power_of_ten - 1,
                    },
                    IdRange {
                        first: next_power_of_ten,
                        last,
                    },
                ])
            } else {
                vec![range]
            }
        })
        .collect()
}

fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as argument");
        std::process::exit(1)
    });
    let id_ranges_string =
        fs::read_to_string(input_filename).expect("Should have been able to read the file");
    let raw_ranges = id_ranges_string
        .split(',')
        .map(|range| {
            range
                .split('-')
                .map(|number| number.parse::<u64>().expect("Invalid range"))
                .collect::<Vec<u64>>()
        })
        .map(|range| IdRange {
            first: range[0],
            last: range[1],
        })
        .collect();

    let same_number_of_digits_ranges = split_by_number_of_digits(raw_ranges);
    let invalid_ids = same_number_of_digits_ranges
        .iter()
        .filter_map(|&IdRange { first, last }| {
            let number_of_digits = log10(last).ceil() as u32;
            if number_of_digits % 2 == 1 {
                None
            } else {
                let divisor = 10_u64.pow(number_of_digits / 2) + 1;
                Some(
                    (first.div_ceil(divisor)..=last / divisor)
                        .map(|quotient| quotient * divisor)
                        .collect::<Vec<u64>>(),
                )
            }
        })
        .flatten();

    let result = invalid_ids.sum::<u64>();
    println!("Result: {}", result);
}
