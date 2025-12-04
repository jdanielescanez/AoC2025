use std::fs;

fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as argument");
        std::process::exit(1)
    });
    let banks_string =
        fs::read_to_string(input_filename).expect("Should have been able to read the file");

    let banks = banks_string.split('\n').map(|bank| {
        bank.chars()
            .map(|joltage| joltage.to_digit(10).expect("Invalid joltage") as u64)
            .enumerate()
            .collect::<Vec<(usize, u64)>>()
    });

    const MAX_TURNED_BATTERIES: usize = 12;

    let result = banks
        .map(|bank| {
            (0..MAX_TURNED_BATTERIES).fold((0, bank), |acc, turned_batteries| {
                let current_bank = acc.1.clone();
                let (position_best, best_joltage) = current_bank
                    .into_iter()
                    .rev()
                    .skip(MAX_TURNED_BATTERIES - (1 + turned_batteries))
                    .max_by_key(|&(_, joltage)| joltage)
                    .unwrap();
                let next_bank = acc
                    .1
                    .into_iter()
                    .filter(|(i, _)| i > &position_best)
                    .collect::<Vec<_>>();

                (10 * acc.0 + best_joltage, next_bank)
            })
        })
        .map(|(joltage, _)| joltage)
        .sum::<u64>();

    println!("Result: {}", result);
}
