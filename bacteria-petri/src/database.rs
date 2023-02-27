use std::fs::{OpenOptions};
use std::io::Write;
use std::time::Instant;

pub fn log(now: Instant, action: String, log: &String) {
    let mut path = OpenOptions::new()
                .create(true)
                .append(true)
                .open(log)
                .unwrap();
    let elapsed_time = now.elapsed().as_secs();
    writeln!(path, "[{}m {}s] {}", elapsed_time/60, elapsed_time%60, action)
    .expect("unable to write to file");
}