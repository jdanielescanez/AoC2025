use std::fs;

#[derive(Debug)]
enum Operator {
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
    let last_line_chars = lines.pop().unwrap().chars();
    let mut whitespace_counter = 0;
    let mut whitespaces = vec![];
    let mut operators = vec![];
    for char in last_line_chars {
        if char == ' ' {
            whitespace_counter += 1;
        } else {
            if whitespace_counter != 0 {
                whitespaces.push(whitespace_counter);
                whitespace_counter = 0;
            }
            if char == '+' {
                operators.push(Operator::Add);
            } else if char == '*' {
                operators.push(Operator::Mul);
            } else {
                panic!("Invalid char {char}")
            }
        }
    }
    whitespaces.push(whitespace_counter + 1);

    let mut padding = 0;
    let operands_matrix = whitespaces
        .iter()
        .enumerate()
        .map(|(i, whitespace_counter)| {
            let submatrix = lines
                .iter()
                .map(|line| {
                    let number = &line[padding + i..padding + whitespace_counter + i];
                    number
                        .to_string()
                        .replace(" ", "0")
                        .chars()
                        .collect::<Vec<char>>()
                })
                .collect::<Vec<Vec<char>>>();
            padding += whitespace_counter;
            submatrix
        })
        .collect::<Vec<Vec<Vec<char>>>>();

    let operators_matrix = operands_matrix
        .into_iter()
        .map(|submatrix| {
            transpose(submatrix)
                .into_iter()
                .map(|row| {
                    row.iter()
                        .collect::<String>()
                        .trim_end_matches('0')
                        .parse::<u64>()
                        .unwrap()
                })
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let result = operators
        .into_iter()
        .zip(operators_matrix)
        .map(|(operator, operands)| match operator {
            Operator::Add => operands.into_iter().sum::<u64>(),
            Operator::Mul => operands.into_iter().product(),
        })
        .sum::<u64>();

    println!("Result: {}", result);
}
