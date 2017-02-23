use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

// Run the given source file.
// TODO: Finish it!
pub fn run_file(filename: &str) {
    let path = Path::new(filename);

    let mut file = match File::open(&path) {
        Err(reason) => panic!("Couldn't open {}: {}", path.display(), reason.description()),
        Ok(f) => f,
    };

    let mut source = String::new();
    file.read_to_string(&mut source).expect("Couldn't read from file");
    println!("{}", source);
}

// Receive input from stdin and run each line.
// TODO: Finish it!
pub fn run_repl() {
    loop {
        let mut buffer = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer).expect("Couldn't read from stdin");
        print!("{}", buffer);
    }
}