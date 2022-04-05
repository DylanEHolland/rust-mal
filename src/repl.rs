use std::io::{self, BufRead};
use crate::parser;

fn eval() {

}

fn print() {

}

fn read(line: &str) {
    println!("> {}", line);
    parser::parse();
}

pub fn run() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    loop {
        let line = iterator.next().unwrap().unwrap();
        read(&line);
    }
}