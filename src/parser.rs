use lexer::Token;

pub fn parse(tokens: &[Token]) {
    println!("Tokens: ");

    for token in tokens {
        print!("{}", token);
    }
    println!();
}
