use lexer;
use parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

// Run the given source file.
pub fn run_file(filename: &str) {
    let path = Path::new(filename);

    let mut file = match File::open(&path) {
        Err(reason) => panic!("Couldn't open {}: {}", path.display(), reason.description()),
        Ok(f) => f,
    };

    let mut source = String::new();
    file.read_to_string(&mut source).expect("Couldn't read from file");

    run(&source);
}

// Receive input from stdin and run each line.
pub fn run_repl() {
    loop {
        let mut line = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut line).expect("Couldn't read from stdin");

        run(&line);
    }
}

fn run(source: &str) {
    let tokens = lexer::lex(source).unwrap();
    parser::parse(&tokens);
}