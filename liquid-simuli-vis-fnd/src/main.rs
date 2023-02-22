use cell::Generate;
use cell:BinaryFission;
mod cell;


fn main() {
    let mut cell_list: Vec<(String, u16)> = Vec::new();

    let mut cell1 = cell::Cell::generate(&mut cell_list);
    cell1.binary_fission;
}
