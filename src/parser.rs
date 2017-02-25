use lexer::Token;


pub fn parse(tokens: &Vec<Token>) {
    println!("Tokens: ");

    for token in tokens {
        print!("{} ", token);
    }
}