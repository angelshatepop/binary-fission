use cell::*;
use std::time::Instant;
mod cell;
use std::fs::remove_dir_all;

fn main() {
    remove_dir_all("simuli/target").err();
    let now = Instant::now();
    let log: String = "log.txt".to_string();
    let mut cell_list: Vec<cell::Cell> = Vec::with_capacity(100);
    let alpha_cell = Generate::generate_alpha("alpha".to_string(), &mut cell_list, now, &log);

    let (mut _cell1, mut _cell2) = <cell::Cell as BinaryFission>::binary_fission(alpha_cell, &mut cell_list, now, &log).unwrap();

    for cell in &cell_list{
        println!("{:?}", cell);
    }
}
