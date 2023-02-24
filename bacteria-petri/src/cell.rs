use rand::{Rng, distributions::Alphanumeric};
use std::fs::{OpenOptions};
use std::io::Write;
use std::time::Instant;
#[derive(Clone)]
pub struct Cell { 
    pub name: String,
    pub is_child: Option<bool>,
    pub has_split: Option<bool>
}
pub trait Log {
    fn log(now: Instant, action: String, log: &String);
}
impl Log for Cell {
    fn log(now: Instant, action: String, log: &String) {
        let mut path = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(log)
                    .unwrap();
        let elapsed_time = now.elapsed().as_secs();
        if elapsed_time > 59{
            writeln!(path, "[{}:{}]", elapsed_time / 60, elapsed_time%60)
            .expect("unable to write to file");
        }
        else{
            writeln!(path, "[0:{}] {}", format!("{}", elapsed_time), action)
            .expect("unable to write to file");
        }
    }
}

pub trait Generate {
    fn generate(name: String, vec: &mut Vec<Cell>, start: Instant, log: &String) -> Self;
    fn generatechild(parent: &mut Cell, vec: &mut Vec<Cell>) -> Self;
}
impl Generate for Cell {
    fn generate(name: String, vec: &mut Vec<Cell>, start: Instant, log: &String) -> Self {
        let cell = Cell{name, is_child: Some(false), has_split: Some(false)};
        Cell::log(start, format!("cell: '{}' has been [SPAWNED]", cell.name), log);
        let cell_copy = cell.clone();
        vec.push(cell);

        cell_copy
    }
    fn generatechild(parent: &mut Cell, vec: &mut Vec<Cell>) -> Self {
        let bname: String = rand::thread_rng().sample_iter(&Alphanumeric).take(7).map(char::from).collect();
        let name = format!("{}-{}", bname, parent.name);
        let cell = Cell{name, is_child: Some(true), has_split: Some(false)};

        let cell_copy = cell.clone();
        vec.push(cell);
        cell_copy
    }
} 
pub trait BinaryFission {
    fn binary_fission(parent: &mut Cell, vec: &mut Vec<Cell>, start: Instant, log: &String) -> Option<(Cell, Cell)>;
}
impl BinaryFission for Cell {
    fn binary_fission(parent: &mut Cell, vec: &mut Vec<Cell>, start: Instant, log: &String) -> Option<(Cell, Cell)>{
        if parent.has_split == Some(false){
            let (cell1, cell2) = (Cell::generatechild(parent, vec), Cell::generatechild(parent, vec));
            Cell::log(start, format!("cell: '{}' has [SPLIT] => {} / {}", parent.name, cell1.name, cell2.name), log);
            parent.has_split = Some(true);
            return Some((cell1, cell2));
        }
        else{
            return None;
        }
    }
}