use std::{
    cmp::{max, min},
    collections::HashSet,
};

#[derive(PartialEq, Eq, Clone, Copy)]
struct JuctionBox {
    x: usize,
    y: usize,
    z: usize,
}

impl JuctionBox {
    pub fn new(line: &str) -> JuctionBox {
        let [x, y, z] = line
            .split(',')
            .map(|number| number.parse::<usize>().expect("Invalid coordinate"))
            .collect::<Vec<usize>>()[..3]
        else {
            panic!("Invalid line")
        };
        JuctionBox { x, y, z }
    }

    pub fn distance(&self, other: JuctionBox) -> f64 {
        ((self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)) as f64)
            .sqrt()
    }
}

pub struct Network {
    juction_boxes: Vec<JuctionBox>,
}

impl Network {
    pub fn new(network_string: &str) -> Network {
        let juction_boxes = network_string.split('\n').map(JuctionBox::new).collect();
        Network { juction_boxes }
    }

    pub fn get_efficient_circuits(&self, pairs_to_connect: usize) -> Vec<HashSet<usize>> {
        let posible_conections = (0..self.juction_boxes.len()).flat_map(|i| {
            (0..self.juction_boxes.len())
                .filter_map(move |j| if i < j { Some((i, j)) } else { None })
        });
        let mut distances = posible_conections.fold(Vec::new(), |mut acc, (i, j)| {
            acc.push((
                self.juction_boxes[i].distance(self.juction_boxes[j]),
                (i, j),
            ));
            acc
        });
        distances.sort_by(|(distance1, _), (distance2, _)| distance1.total_cmp(distance2));

        distances.into_iter().take(pairs_to_connect).fold(
            Vec::new(),
            |mut circuits, (_, (i, j))| {
                let mut subcircuit = HashSet::new();
                subcircuit.insert(j);
                subcircuit.insert(i);

                let position_circuit_i = circuits.iter().position(|circuit| circuit.contains(&i));
                let position_circuit_j = circuits.iter().position(|circuit| circuit.contains(&j));
                match (position_circuit_i, position_circuit_j) {
                    (Some(position_circuit_i), Some(position_circuit_j)) => {
                        if position_circuit_i == position_circuit_j {
                            circuits[position_circuit_i].extend(&subcircuit);
                        } else {
                            let lower_position = min(position_circuit_i, position_circuit_j);
                            let upper_position = max(position_circuit_i, position_circuit_j);
                            let upper_subcircuit = circuits[upper_position].clone();
                            circuits.remove(upper_position);
                            circuits[lower_position].extend(&upper_subcircuit);
                        }
                    }
                    (Some(only_one_position), None) | (None, Some(only_one_position)) => {
                        circuits[only_one_position].extend(&subcircuit);
                    }
                    (None, None) => {
                        circuits.push(subcircuit);
                    }
                }
                circuits
            },
        )
    }
}
