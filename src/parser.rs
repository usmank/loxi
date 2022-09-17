use lexer::Token;
use result::Result;

pub fn parse(tokens: &[Token]) -> Result<()> {
    println!("Tokens: ");

    for token in tokens {
        print!("{}", token);
    }

    println!();

    Ok(())
}
