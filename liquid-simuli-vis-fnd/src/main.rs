use cell::Generate;
mod cell;

pub const HAYFLICK_LIMIT: u16 = 4;

pub fn split(cell: &mut cell::Cell, vec: &mut Vec<(String, u16)>) {
        cell.cycles += 1;
        let mut _cell1 = cell::Cell::generate(vec);
        let mut _cell2 = cell::Cell::generate(vec);
        _cell2.cycles += HAYFLICK_LIMIT / 2;
        _cell2.cycles += HAYFLICK_LIMIT / 2;
}

fn main() {
    let mut cell_list: Vec<(String, u16)> = Vec::new();

    let mut cell1 = cell::Cell::generate(&mut cell_list);
    split(&mut cell1, &mut cell_list);

    for (name, cycles) in cell_list {
        println!("{} {}", name, cycles);
    }
    println!("{}", cell1.cycles);
}
