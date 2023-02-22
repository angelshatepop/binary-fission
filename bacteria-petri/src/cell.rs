use rand::{Rng, distributions::Alphanumeric};

pub struct Cell { 
    pub name: String, 
    pub cycles: u16,
}
pub trait Generate {
    fn generate(vec: &mut Vec<(String, u16)>) -> Self;
}
pub trait BinaryFission {
    fn binary_fission(cell: &mut Cell, vec: &mut Vec<(String, u16)>);
}
impl Generate for Cell {
    fn generate(vec: &mut Vec<(String, u16)>) -> Cell {
        let name: String = rand::thread_rng().sample_iter(&Alphanumeric).take(7).map(char::from).collect();
        let cycles: u16 = 0;
        let cell = Cell {name, cycles};
        vec.push((cell.name.clone(), cell.cycles.clone()));
        cell
    }
}
impl BinaryFission for Cell {
    fn binary_fission(cell: &mut Cell, vec: &mut Vec<(String, u16)>) {
        let mut i = 0;
        loop {
            if i < 2 {
                let mut cell = Cell::generate(vec);
                println!("new cell from split: {}", cell.name);
                i += 1;
            }
        }
    }
}