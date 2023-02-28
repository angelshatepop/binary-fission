#[path = "log.rs"] mod log;
use rand::{Rng, distributions::Alphanumeric};
use std::time::Instant;

#[derive(Clone, Debug, PartialEq)]
pub struct Cell { 
    pub name: String,
    pub is_child: Option<bool>,
    pub has_split: Option<bool>
}

pub trait Generate {
    fn generate_alpha(name: String, vec: &mut Vec<Cell>, start: Instant, log: &String) -> Self;
}
impl Generate for Cell {
    fn generate_alpha(name: String, vec: &mut Vec<Cell>, start: Instant, log: &String) -> Self {
        let cell: Cell = Cell{name, is_child: Some(false), has_split: Some(false)};
        log::log(start, format!("cell: '{}' has [SPAWNED] ? {:?}", cell.name, cell), log);
        vec.push(cell.clone());

        return cell;
    }
} 
pub trait BinaryFission {
    fn generate_child(parent: &mut Cell, vec: &mut Vec<Cell>) -> Self;
    fn binary_fission(parent: Cell, vec: &mut Vec<Cell>, start: Instant, log: &String) -> Option<(Cell, Cell)>;
}

impl BinaryFission for Cell {
    fn generate_child(parent: &mut Cell, vec: &mut Vec<Cell>) -> Self {
        let bname: String = rand::thread_rng().sample_iter(&Alphanumeric).take(7).map(char::from).collect();
        let name: String = format!("{}-{}", bname, parent.name);
        let cell: Cell = Cell{name, is_child: Some(true), has_split: Some(false)};
        vec.push(cell.clone());

        return cell;
    }
    fn binary_fission(mut parent: Cell, vec: &mut Vec<Cell>, start: Instant, log: &String) -> Option<(Cell, Cell)>{
        if parent.has_split == Some(false){
            let parent_i = vec.iter().position(|r| *r == parent).unwrap();
            let cell1 = Self::generate_child(&mut parent, vec);
            let cell2 = Self::generate_child(&mut parent, vec);
            vec.remove(parent_i);
            log::log(start, format!("cell: '{}' has [SPLIT] => {} / {} ++cell: '{}' [REMOVED]", 
                                                parent.name, 
                                                cell1.name,
                                                cell2.name, 
                                                parent.name), log);
            return Some((cell1, cell2));
        } else{
            panic!("cell {} has attempted to split more than once", parent.name);
        }
    }
}