use cell::*;
use std::time::Instant;
mod cell;

fn main() {
    let now = Instant::now();
    let log: String = "log.txt".to_string();
    let mut cell_list: Vec<cell::Cell> = Vec::new();

    //initialize test
    let mut alpha_cell = Generate::generate_alpha("alpha".to_string(), &mut cell_list, now, &log);
    let (mut cell1, mut _cell2) = <cell::Cell as BinaryFission>::binary_fission(&mut alpha_cell, &mut cell_list, now, &log).unwrap();
    //before test
    println!("\n{:?}", cell1);
    for cell in &cell_list{
        println!("{}", cell.name);
    }
    //test
    let (mut _cell3, mut _cell4) = <cell::Cell as BinaryFission>::binary_fission(&mut cell1, &mut cell_list, now, &log).unwrap();
    //after test
    println!("\n{:?}", cell1);
    for cell in &cell_list{
        println!("{}", cell.name);
    }
    //error test
    let (mut _cell5, mut _cell6) = 
        <cell::Cell as BinaryFission>::binary_fission(&mut cell1, &mut cell_list, now, &log).unwrap();
}