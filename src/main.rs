extern crate loxi;

use std::cmp::Ordering;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len().cmp(&2) {
        Ordering::Greater => println!("Usage: loxi [script]"),
        Ordering::Equal => loxi::loxi::run_file(&args[1]),
        Ordering::Less => loxi::loxi::run_repl(),
    }
}
