use rand::{Rng, distributions::Alphanumeric};

pub struct Cell { 
    pub name: String, 
    pub cycles: u16,
}
pub trait Generate {
    fn generate(vec: &mut Vec<(String, u16)>) -> Self;
}

impl Generate for Cell {
    fn generate(vec: &mut Vec<(String, u16)>) -> Cell {
        let name: String = rand::thread_rng().sample_iter(&Alphanumeric).take(5).map(char::from).collect();
        let cycles: u16 = 0;
        let cell = Cell {name, cycles};
        vec.push((cell.name.clone(), cell.cycles.clone()));
        cell
    }
}