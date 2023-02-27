use cell::*;
use std::time::Instant;
mod cell;

fn main() {
    let now = Instant::now();
    let log: String = "log.txt".to_string();
    let mut cell_list: Vec<cell::Cell> = Vec::new();
    let mut alpha_cell = Generate::generate_alpha("alpha".to_string(), &mut cell_list, now, &log);
    let (mut _cell1, mut _cell2) = <cell::Cell as BinaryFission>::binary_fission(&mut alpha_cell, &mut cell_list, now, &log).unwrap();
}
