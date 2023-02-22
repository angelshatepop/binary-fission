use cell::Generate;
use cell::BinaryFission;
mod cell;


fn main() {
    let mut cell_list: Vec<(String, u16)> = Vec::new();

    let mut cell1 = cell::Cell::generate(&mut cell_list);
    println!("{}", cell1.name);
    cell::Cell::binary_fission(&mut cell1, &mut cell_list);
    
}