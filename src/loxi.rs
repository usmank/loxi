use crate::lexer;
use crate::parser;
use dirs;
use rustyline::Editor;
use rustyline::error::ReadlineError;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

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
    let home_dir = dirs::home_dir().unwrap_or(PathBuf::from("."));
    let history_file = home_dir.join(".loxi.history");

    let mut rl = Editor::<()>::new()?;
    if rl.load_history(&history_file).is_err() {
        eprintln!("No history loaded");
    }

    loop {
        let line = rl.readline("> ");

        match line {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                if let Err(error) = run(&line) {
                    eprintln!("{}", error);
                }
            }
            Err(ReadlineError::Interrupted) => break,
            Err(ReadlineError::Eof) => break,
            Err(error) => eprintln!("Error: {}", error),
        }
    }

    rl.save_history(&history_file)?;
    Ok(())
}

mod tests {
    //use super::*;

    #[test]
    fn dummy() {}
}
