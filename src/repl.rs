use std::io::{self, BufRead};
use crate::parser;

fn _eval() {

}

fn _print() {

}

fn read(line: &str) {
    parser::parse(line);
}

pub fn run() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    loop {
        let line = iterator.next().unwrap().unwrap();
        read(&line);
    }
}