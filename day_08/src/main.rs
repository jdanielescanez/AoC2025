use std::fs;

mod network;

use network::Network;

fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as argument");
        std::process::exit(1)
    });
    let network_string =
        fs::read_to_string(input_filename).expect("Should have been able to read the file");

    let network = Network::new(&network_string);
    let mut efficient_circuits = network.get_efficient_circuits(1000);
    efficient_circuits.sort_by_key(|circuit| circuit.len());
    let result_part_1 = efficient_circuits
        .into_iter()
        .rev()
        .map(|circuit| circuit.len())
        .take(3)
        .product::<usize>();
    let result_part_2 = network.get_x_product_of_last_pair();

    println!("Result part 1: {}", result_part_1);
    println!("Result part 2: {}", result_part_2);
}
