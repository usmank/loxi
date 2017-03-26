use itertools::multipeek;
use std::fmt;
use std::str;

// TODO: Return Result<Error, Vec<Token>> instead.
pub fn lex(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    'line_loop: for (i, line) in source.lines().enumerate() {
        let bytes = line.as_bytes();
        let line_number = i + 1;
        let mut iter = multipeek(bytes.iter().enumerate());

        'byte_loop: while let Some((j, c)) = iter.next() {
            let token = match *c {
                b'(' => {
                    Token::LeftParen {
                        lexeme: str::from_utf8(&bytes[j..j + 1]).unwrap(),
                        source_position: line_number,
                    }
                }
                b')' => {
                    Token::RightParen {
                        lexeme: str::from_utf8(&bytes[j..j + 1]).unwrap(),
                        source_position: line_number,
                    }
                }
                b'{' => {
                    Token::LeftBrace {
                        lexeme: str::from_utf8(&bytes[j..j + 1]).unwrap(),
                        source_position: line_number,
                    }
                }
                b'}' => {
                    Token::RightBrace {
                        lexeme: str::from_utf8(&bytes[j..j + 1]).unwrap(),
                        source_position: line_number,
                    }
                }
                b',' => {
                    Token::Comma {
                        lexeme: str::from_utf8(&bytes[j..j + 1]).unwrap(),
                        source_position: line_number,
                    }
                }
                b'.' => {
                    Token::Dot {
                        lexeme: str::from_utf8(&bytes[j..j + 1]).unwrap(),
                        source_position: line_number,
                    }
                }
                b'-' => {
                    Token::Minus {
                        lexeme: str::from_utf8(&bytes[j..j + 1]).unwrap(),
                        source_position: line_number,
                    }
                }
                b'+' => {
                    Token::Plus {
                        lexeme: str::from_utf8(&bytes[j..j + 1]).unwrap(),
                        source_position: line_number,
                    }
                }
                b';' => {
                    Token::Semicolon {
                        lexeme: str::from_utf8(&bytes[j..j + 1]).unwrap(),
                        source_position: line_number,
                    }
                }
                b'*' => {
                    Token::Asterisk {
                        lexeme: str::from_utf8(&bytes[j..j + 1]).unwrap(),
                        source_position: line_number,
                    }
                }
                b'!' => {
                    match iter.peek() {
                        Some(&(_, &b'=')) => {
                            iter.next();
                            Token::BangEqual {
                                lexeme: str::from_utf8(&bytes[j..j + 2]).unwrap(),
                                source_position: line_number,
                            }
                        }
                        _ => {
                            Token::Bang {
                                lexeme: str::from_utf8(&bytes[j..j + 1]).unwrap(),
                                source_position: line_number,
                            }
                        }
                    }
                }
                b'=' => {
                    match iter.peek() {
                        Some(&(_, &b'=')) => {
                            iter.next();
                            Token::EqualEqual {
                                lexeme: str::from_utf8(&bytes[j..j + 2]).unwrap(),
                                source_position: line_number,
                            }
                        }
                        _ => {
                            Token::Equal {
                                lexeme: str::from_utf8(&bytes[j..j + 1]).unwrap(),
                                source_position: line_number,
                            }
                        }
                    }
                }
                b'<' => {
                    match iter.peek() {
                        Some(&(_, &b'=')) => {
                            iter.next();
                            Token::LessThanOrEqual {
                                lexeme: str::from_utf8(&bytes[j..j + 2]).unwrap(),
                                source_position: line_number,
                            }
                        }
                        _ => {
                            Token::LessThan {
                                lexeme: str::from_utf8(&bytes[j..j + 1]).unwrap(),
                                source_position: line_number,
                            }
                        }
                    }
                }
                b'>' => {
                    match iter.peek() {
                        Some(&(_, &b'=')) => {
                            iter.next();
                            Token::GreaterThanOrEqual {
                                lexeme: str::from_utf8(&bytes[j..j + 2]).unwrap(),
                                source_position: line_number,
                            }
                        }
                        _ => {
                            Token::GreaterThan {
                                lexeme: str::from_utf8(&bytes[j..j + 1]).unwrap(),
                                source_position: line_number,
                            }
                        }
                    }
                }
                b'/' => {
                    match iter.peek() {
                        Some(&(_, &b'/')) => {
                            // Encountered a comment so continue to next line.
                            continue 'line_loop;
                        }
                        _ => {
                            Token::Slash {
                                lexeme: str::from_utf8(&bytes[j..j + 1]).unwrap(),
                                source_position: line_number,
                            }
                        }
                    }
                }
                // TODO: This should eventually emit an error instead of just continuing.
                _ => continue,
            };
            tokens.push(token);
        }
    }
    tokens
}

pub type SourcePosition = usize;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Token<'a> {
    LeftParen {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    RightParen {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    LeftBrace {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    RightBrace {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    Comma {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    Dot {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    Minus {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    Plus {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    Semicolon {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    Slash {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    Asterisk {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    Bang {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    BangEqual {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    Equal {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    EqualEqual {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    GreaterThan {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    GreaterThanOrEqual {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    LessThan {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    LessThanOrEqual {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    Identifier {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    String {
        lexeme: &'a str,
        source_position: SourcePosition,
        literal: String,
    },
    Number {
        lexeme: &'a str,
        source_position: SourcePosition,
        literal: f64,
    },
    And {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    Class {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    Else {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    False {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    Fun {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    For {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    If {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    Nil {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    Or {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    Print {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    Return {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    Super {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    This {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    True {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    Var {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    While {
        lexeme: &'a str,
        source_position: SourcePosition,
    },
    Eof,
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::LeftParen { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::RightParen { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::LeftBrace { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::RightBrace { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::Comma { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::Dot { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::Minus { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::Plus { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::Semicolon { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::Slash { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::Asterisk { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::Bang { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::BangEqual { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::Equal { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::EqualEqual { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::GreaterThan { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::GreaterThanOrEqual { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::LessThan { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::LessThanOrEqual { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::Identifier { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::String { lexeme, source_position: _, literal: _ } => write!(f, "{}", lexeme),
            Token::Number { lexeme: _, source_position: _, literal } => write!(f, "{}", literal),
            Token::And { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::Class { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::Else { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::False { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::Fun { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::For { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::If { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::Nil { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::Or { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::Print { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::Return { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::Super { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::This { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::True { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::Var { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::While { lexeme, source_position: _ } => write!(f, "{}", lexeme),
            Token::Eof => Ok(()),
        }
    }
}