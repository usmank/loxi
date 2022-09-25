use crate::ast::{Expression, LiteralValue};
use crate::lexer::Token;
use crate::result::Result;

pub fn parse(tokens: &[Token]) -> Result<()> {
    println!("Tokens: ");

    for token in tokens {
        print!("{} ", token);
    }

    println!();

    Ok(())
}
