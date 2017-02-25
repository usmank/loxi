extern crate loxi;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: loxi [script]");
    } else if args.len() == 2 {
        loxi::loxi::run_file(&args[1]);
    } else {
        loxi::loxi::run_repl();
    }
}