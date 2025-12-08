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

    let mut efficient_circuits = Network::new(&network_string).get_efficient_circuits(1000);
    efficient_circuits.sort_by_key(|circuit| circuit.len());
    let result = efficient_circuits
        .into_iter()
        .rev()
        .map(|circuit| circuit.len())
        .take(3)
        .product::<usize>();
    println!("Result: {}", result);
}
