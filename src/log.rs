use std::fs::{OpenOptions};
use std::{io::Write, time::Instant};
use std::{sync::mpsc, thread};

pub fn log(now: Instant, action: String, log: String) {
    let (sender, receiver) = mpsc::channel();
    let handle = thread::spawn(move|| {
        let mut path = OpenOptions::new()
                .create(true)
                .append(true)
                .open(log)
                .unwrap();
        let message = receiver.recv().unwrap();
        writeln!(path, "{}", message).expect("unable to log event");
    });

    let elapsed = now.elapsed().as_secs();
    let message = format!("[{}m {}s] {}", elapsed/60, elapsed%60, action); 
    sender.send(message).unwrap(); handle.join().unwrap();
}
