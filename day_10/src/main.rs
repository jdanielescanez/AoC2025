mod parser;

use itertools::Itertools;
use std::fs;

use crate::parser::read_input;

fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as argument");
        std::process::exit(1)
    });
    let machines_string =
        fs::read_to_string(input_filename).expect("Should have been able to read the file");

    let (_, machines) = read_input(&machines_string).unwrap();

    let result = get_result_part_1(machines);
    println!("Result: {}", result);
}

fn get_result_part_1(machines: Vec<(u16, Vec<u16>, Vec<u16>)>) -> usize {
    machines
        .into_iter()
        .map(|(state, buttons, _joltages)| {
            (0..buttons.len())
                .find(|total_presses| {
                    buttons
                        .iter()
                        .cloned()
                        .combinations(*total_presses)
                        .any(|combination| {
                            combination.into_iter().reduce(|a, b| a ^ b) == Some(state)
                        })
                })
                .unwrap()
        })
        .sum::<usize>()
}
