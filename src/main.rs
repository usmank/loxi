use exitcode;
use loxi::loxi;
use std::cmp::Ordering;
use std::env;

fn process_error_and_exit(result: &loxi::Result) {
    match result {
        Ok(_) => std::process::exit(exitcode::OK),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1)
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len().cmp(&2) {
        Ordering::Equal => {
            let result = loxi::run_file(&args[1]);
            process_error_and_exit(&result);
        }
        Ordering::Less => {
            let result = loxi::run_repl();
            process_error_and_exit(&result);
        }
        Ordering::Greater => {
            println!("Usage: loxi [script]");
            std::process::exit(exitcode::USAGE);
        }
    };
}
