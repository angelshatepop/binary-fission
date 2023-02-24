use cell::*;
use std::time::Instant;
mod cell;

fn main() {
    let mut list: Vec<(String, u16)> = Vec::new();
    let now = Instant::now();
    let log: String = "log.txt".to_string();
    let mut cell1 = Generate::generate("alpha".to_string(), &mut list, now, &log);
    <cell::Cell as BinaryFission>::binary_fission(&mut cell1, &mut list, now, &log);
    for cell in &list{
        println!("{:?}", cell);
    }
    <cell::Cell as BinaryFission>::binary_fission(&mut cell1, &mut list, now, &log);
    for cell in &list{
        println!("{:?}", cell);
    }
    <cell::Cell as BinaryFission>::binary_fission(&mut cell1, &mut list, now, &log);
    for cell in &list{
        println!("{:?}", cell);
    }
}