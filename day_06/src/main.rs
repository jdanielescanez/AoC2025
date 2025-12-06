use std::fs;

enum Operand {
    Add,
    Mul,
}

fn transpose<T>(vector: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Copy,
{
    let rows = vector.len();
    let cols = vector[0].len();

    (0..cols)
        .map(|col| (0..rows).map(|row| vector[row][col]).collect())
        .collect()
}

fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as argument");
        std::process::exit(1)
    });
    let cephalopod_string =
        fs::read_to_string(input_filename).expect("Should have been able to read the file");

    let mut lines = cephalopod_string.split('\n').collect::<Vec<&str>>();
    let operands = lines
        .pop()
        .unwrap()
        .split_ascii_whitespace()
        .map(|operand| match operand {
            "+" => Operand::Add,
            "*" => Operand::Mul,
            _ => panic!("Invalid operand {operand}"),
        })
        .collect::<Vec<Operand>>();

    let operators_matrix = lines
        .into_iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();
    let operators_matrix = transpose(operators_matrix);

    let result = operands
        .into_iter()
        .zip(operators_matrix)
        .map(|(operand, operators)| match operand {
            Operand::Add => operators.iter().sum::<u64>(),
            Operand::Mul => operators.iter().product(),
        })
        .sum::<u64>();

    println!("Result: {}", result);
}
