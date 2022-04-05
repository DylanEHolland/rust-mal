use std::io::{self, BufRead};

pub fn run() {
    let stdin = io::stdin();

    let mut iterator = stdin.lock().lines();

    loop {
        let line = iterator.next().unwrap().unwrap();
        println!("> {}", line);
    }
}