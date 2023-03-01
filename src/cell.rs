#[path = "log.rs"] mod log;
use rand::{Rng, distributions::Alphanumeric};
use std::{time, thread::sleep};

#[derive(Clone, Debug, PartialEq)]
pub struct Cell { 
    pub name: String,
    pub is_child: Option<bool>,
    pub has_split: Option<bool>
}

pub trait Generate {
    fn generate_alpha(name: String, vec: &mut Vec<Cell>, start: time::Instant, log: &String) -> Self;
}
impl Generate for Cell {
    fn generate_alpha(name: String, vec: &mut Vec<Cell>, start: time::Instant, log: &String) -> Self {
        let cell: Cell = Cell{name, is_child: Some(false), has_split: Some(false)};
        log::log(start, format!("cell: '{}' has [SPAWNED] ? {:?}", cell.name, cell), log.to_string());
        vec.push(cell.clone());

        return cell;
    }
} 
pub trait BinaryFission {
    fn generate_child(parent: &mut Cell, vec: &mut Vec<Cell>, pname: String) -> Self;
    fn binary_fission(parent: Cell, vec: &mut Vec<Cell>, start: time::Instant, log: &String, depth: u32, name: &String) -> Option<(Cell, Cell)>;
}

impl BinaryFission for Cell {
    fn generate_child(_parent: &mut Cell, vec: &mut Vec<Cell>, pname: String) -> Self {
        let bname: String = rand::thread_rng().sample_iter(&Alphanumeric).take(7).map(char::from).collect();
        let name: String = format!("{}-{}", bname, pname);
        let cell: Cell = Cell{name, is_child: Some(true), has_split: Some(false)};
        vec.push(cell.clone());

        return cell;
    }
    fn binary_fission(mut parent: Cell, vec: &mut Vec<Cell>, start: time::Instant, log: &String, depth: u32, name: &String) -> Option<(Cell, Cell)>{
        sleep(time::Duration::from_millis(1000));
        const MAX_DEPTH: u32 = 100;
        if depth >= MAX_DEPTH {
            std::process::abort();
        }

        if parent.has_split == Some(false){
            let parent_i = vec.iter().position(|r| *r == parent).unwrap();
            let cell1 = Self::generate_child(&mut parent, vec, name.to_string());
            let cell2 = Self::generate_child(&mut parent, vec, name.to_string());
            vec.remove(parent_i);
            log::log(start, format!("cell: '{}' has [SPLIT] => {} / {} ++cell: '{}' [REMOVED]", 
                                                parent.name, 
                                                cell1.name,
                                                cell2.name, 
                                                parent.name), log.to_string());
            let child1 = Self::binary_fission(cell1, vec, start, log, depth+1, name);
            let child2 = Self::binary_fission(cell2, vec, start, log, depth+1, name);
            
            match (child1, child2) {
                (Some(c1), Some(c2)) => Some((c1.0, c2.1)),
                _ => None,
            }
        } else{
            panic!("cell {} has attempted to split more than once", parent.name);
        }
    }
}