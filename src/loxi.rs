use crate::lexer;
use crate::parser;
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub type Result = std::result::Result<(), Box<dyn Error>>;

fn run(source: &str) -> Result {
    let tokens = lexer::lex(source)?;
    let ast = parser::parse(&tokens)?;
    println!("{ast}");
    Ok(())
}

// Run the given source file.
pub fn run_file(filename: &str) -> Result {
    let path = Path::new(filename);
    let source = fs::read_to_string(&path)?;

    run(&source)
}

// Receive input from stdin and run each line.
pub fn run_repl() -> Result {
    loop {
        let mut line = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut line)?;

        // Exit loop on Ctrl+D.
        if line.is_empty() {
            break;
        }

        let result = run(&line);

        if let Err(error) = result {
            eprintln!("{}", error);
        }
    }

    Ok(())
}

mod tests {
    //use super::*;

    #[test]
    fn dummy() {}
}
