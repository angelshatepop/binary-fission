use cell::*; mod cell;
use std::{time::Instant, fs::*, thread};


fn main() {
    remove_dir_all("simuli/target").err();
    let now = Instant::now();
    let log: String = "log.txt".to_string();
    let depth: u32 = 0;
    let mut cellsalpha: Vec<cell::Cell> = Vec::with_capacity(100);
    let mut cellsomega: Vec<cell::Cell> = Vec::with_capacity(100);

    let alpha: Cell = Generate::generate_alpha("alpha".to_string(), &mut cellsalpha, now, &log);
    let alphan = alpha.clone().name;

    let omega: Cell = Generate::generate_alpha("omega".to_string(), &mut cellsomega, now, &"log.txt".to_string());
    let omegan = omega.clone().name;

    let handle_omega = thread::spawn(move || {
        <cell::Cell as BinaryFission>::binary_fission(omega, 
            &mut cellsomega, 
            now, 
            &"log.txt".to_string(), 
            depth, 
            &omegan)
            .unwrap();
    });
    let handle_alpha = thread::spawn(move ||{
        <cell::Cell as BinaryFission>::binary_fission(alpha, 
            &mut cellsalpha, 
            now, 
            &log, 
            depth, 
            &alphan)
            .unwrap();
    });

    handle_omega.join().unwrap();
    handle_alpha.join().unwrap();
}