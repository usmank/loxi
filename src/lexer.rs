use std::fmt;
use itertools::{MultiPeek, multipeek};
use result::{Error, Result};

const RADIX: u32 = 10;

pub fn lex(source: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();

    'line_loop: for (i, line) in source.lines().enumerate() {
        let char_indices = line.char_indices();
        let mut iter = multipeek(char_indices);

        'char_loop: while let Some((j, c)) = iter.next() {
            let source_position = (i + 1, j + 1);
            let (token_type, lexeme, literal) = match c {
                '(' => (TokenType::LeftParen, &line[j..j + 1], Literal::None),
                ')' => (TokenType::RightParen, &line[j..j + 1], Literal::None),
                '{' => (TokenType::LeftBrace, &line[j..j + 1], Literal::None),
                '}' => (TokenType::RightBrace, &line[j..j + 1], Literal::None),
                ',' => (TokenType::Comma, &line[j..j + 1], Literal::None),
                '.' => (TokenType::Dot, &line[j..j + 1], Literal::None),
                '-' => (TokenType::Minus, &line[j..j + 1], Literal::None),
                '+' => (TokenType::Plus, &line[j..j + 1], Literal::None),
                ';' => (TokenType::Semicolon, &line[j..j + 1], Literal::None),
                '*' => (TokenType::Asterisk, &line[j..j + 1], Literal::None),
                '!' => {
                    match iter.peek() {
                        Some(&(_, '=')) => {
                            iter.next();
                            (TokenType::BangEqual, &line[j..j + 2], Literal::None)
                        }
                        _ => (TokenType::Bang, &line[j..j + 1], Literal::None),
                    }
                }
                '=' => {
                    match iter.peek() {
                        Some(&(_, '=')) => {
                            iter.next();
                            (TokenType::EqualEqual, &line[j..j + 2], Literal::None)
                        }
                        _ => (TokenType::Equal, &line[j..j + 1], Literal::None),
                    }
                }
                '<' => {
                    match iter.peek() {
                        Some(&(_, '=')) => {
                            iter.next();
                            (TokenType::LessThanOrEqual, &line[j..j + 2], Literal::None)
                        }
                        _ => (TokenType::LessThan, &line[j..j + 1], Literal::None),
                    }
                }
                '>' => {
                    match iter.peek() {
                        Some(&(_, '=')) => {
                            iter.next();
                            (TokenType::GreaterThanOrEqual, &line[j..j + 2], Literal::None)
                        }
                        _ => (TokenType::GreaterThan, &line[j..j + 1], Literal::None),
                    }
                }
                '/' => {
                    match iter.peek() {
                        Some(&(_, '/')) => {
                            // Encountered a comment, ignore the rest and continue to next line.
                            continue 'line_loop;
                        }
                        _ => (TokenType::Slash, &line[j..j + 1], Literal::None),
                    }
                }
                '"' => {
                    match string(&mut iter, line, j) {
                        Some((lexeme, literal)) => (TokenType::Str, lexeme, literal),
                        None => {
                            return Err(Error::SyntaxError {
                                message: format!("String literal missing closing '\"'"),
                                source_position: source_position,
                            });
                        }
                    }
                }
                // Number
                c if c.is_digit(RADIX) => {
                    let (lexeme, literal) = number(&mut iter, line, j);
                    (TokenType::Number, lexeme, literal)
                }
                // Identifier
                c if c.is_alphabetic() || c == '_' => {
                    let lexeme = identifier(&mut iter, line, j);
                    (map_lexeme_to_keyword(lexeme), lexeme, Literal::None)
                }
                // Ignore whitespace
                c if c.is_whitespace() => continue,

                // Default case
                c => {
                    return Err(Error::SyntaxError {
                        message: format!("Unrecognized character '{}'", c),
                        source_position: source_position,
                    });
                }
            };
            tokens.push(Token {
                token_type: token_type,
                lexeme: lexeme,
                literal: literal,
                source_position: source_position,
            });
        }
    }

    // Append EOF token once we hit the end.
    tokens.push(Token {
        token_type: TokenType::Eof,
        lexeme: "",
        literal: Literal::None,
        source_position: (0, 0),
    });

    Ok(tokens)
}

// Maps the given lexeme to the corresponding TokenType.
fn map_lexeme_to_keyword<'a>(lexeme: &'a str) -> TokenType {
    match lexeme {
        "and" => TokenType::And,
        "class" => TokenType::Class,
        "else" => TokenType::Else,
        "false" => TokenType::False,
        "for" => TokenType::For,
        "fun" => TokenType::Fun,
        "if" => TokenType::If,
        "nil" => TokenType::Nil,
        "or" => TokenType::Or,
        "print" => TokenType::Print,
        "return" => TokenType::Return,
        "super" => TokenType::Super,
        "this" => TokenType::This,
        "true" => TokenType::True,
        "var" => TokenType::Var,
        "while" => TokenType::While,
        _ => TokenType::Identifier,
    }
}

fn string<'a, I>(iter: &mut MultiPeek<I>,
                 line: &'a str,
                 start: usize)
                 -> Option<(&'a str, Literal<'a>)>
    where I: Iterator<Item = (usize, char)>
{
    while let Some(&(_, c)) = iter.peek() {
        match c {
            '"' => break,
            _ => {
                iter.next();
            }
        }
    }

    // Either we found the closing double quote, or we have an untermintated string.
    if let Some((i, '"')) = iter.next() {
        Some((&line[start..i + 1], Literal::Str(&line[start + 1..i])))
    } else {
        None
    }
}

fn number<'a, I>(iter: &mut MultiPeek<I>, line: &'a str, start: usize) -> (&'a str, Literal<'a>)
    where I: Iterator<Item = (usize, char)>
{
    // Scan for zero or more digits making up the integral part of the number.
    let integer_length = digits(iter);

    // Scan for one or more digits after the decimal point which forms the fractional part of the number.
    let fraction_length = if let Some(&(_, '.')) = iter.peek() {
        if let Some(&(_, d)) = iter.peek() {
            if d.is_digit(RADIX) {
                iter.next();
                let fraction_length = digits(iter);

                // + 1 to account for the decimal point.
                fraction_length + 1
            } else {
                0
            }
        } else {
            0
        }
    } else {
        0
    };

    let lexeme = &line[start..start + integer_length + fraction_length + 1];
    (lexeme, Literal::Number(lexeme.parse().unwrap()))
}

// Returns a count of the number consecutive digits. The iterator is advanced so that it points at the last digit in the sequence.
fn digits<I>(iter: &mut MultiPeek<I>) -> usize
    where I: Iterator<Item = (usize, char)>
{
    let mut result: usize = 0;

    while let Some(&(_, c)) = iter.peek() {
        if c.is_digit(RADIX) {
            result += 1;
            iter.next();
        } else {
            iter.reset_peek();
            break;
        }
    }

    result
}

fn identifier<'a, I>(iter: &mut MultiPeek<I>, line: &'a str, start: usize) -> &'a str
    where I: Iterator<Item = (usize, char)>
{
    let mut length = 0;

    while let Some(&(_, c)) = iter.peek() {
        match c {
            c if c.is_alphanumeric() => {
                length += 1;
                iter.next();
            }
            _ => break,
        }
    }

    &line[start..start + length + 1]
}

#[derive(Debug)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub literal: Literal<'a>,
    pub source_position: SourcePosition,
}

#[derive(Debug)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Asterisk,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Identifier,
    Str,
    Number,
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

#[derive(Debug)]
pub enum Literal<'a> {
    None,
    Str(&'a str),
    Number(f64),
}

// Source position is defined by a tuple containing the line number and character index.
pub type SourcePosition = (usize, usize);

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.lexeme)
    }
}