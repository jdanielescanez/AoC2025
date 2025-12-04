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
            .map(|joltage| joltage.to_digit(10).expect("Invalid joltage"))
            .enumerate()
            .collect::<Vec<(usize, u32)>>()
    });

    let result = banks
        .map(|bank| {
            let bank_without_last_joltage = bank.iter().rev().skip(1);
            let (position_best, best_joltage) = bank_without_last_joltage
                .max_by_key(|&(_, joltage)| joltage)
                .unwrap();
            let (_, second_best_joltage) = bank
                .iter()
                .filter(|&(i, _)| i > position_best)
                .max_by_key(|&(_, joltage)| joltage)
                .unwrap();
            10 * best_joltage + second_best_joltage
        })
        .sum::<u32>();

    println!("Result: {}", result);
}
