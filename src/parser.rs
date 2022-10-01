// GRAMMAR
// expression → literal
//            | unary
//            | binary
//            | grouping
//
// literal    → NUMBER | STRING | "true" | "false" | "nil"
// grouping   → "(" expression ")"
// unary      → ( "-" | "!" ) expression
// binary     → expression operator expression
// operator   → "==" | "!=" | "<" | "<=" | ">" | ">="
//            | "+"  | "-"  | "*" | "/" | ","
//
// PRECEDENCE (Lowest to highest)
// Name         Operators   Associates
// ----         ---------   ----------
// Comma        ,           Left
// Equality     == !=       Left
// Comparison   > >= < <=   Left
// Term         - +         Left
// Factor       / *         Left
// Unary        ! -         Right
//
// STRATIFIED GRAMMAR
// expression → comma
// comma      → equality ( "," equality )*
// equality   → comparison ( ( "==" | "!=" ) comparison)*
// comparison → term ( ( ">" | ">=" | "<" | "<=" ) term)*
// term       → factor ( ( "+" | "-" ) factor)*
// factor     → unary ( ( "*" | "/" ) unary )*
// unary      → ( "-" | "!" ) unary | primary
// primary    → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")"

use crate::ast::{Expression, LiteralValue};
use crate::lexer::{Token, TokenType};
use crate::result::Error;
use std::iter::Peekable;

pub type Result<'a> = crate::result::Result<Box<Expression<Token<'a>>>>;

pub fn parse<'a>(tokens: &'a [Token]) -> Result<'a> {
    let mut iter = tokens.iter().peekable();

    expression(&mut iter)
}

fn expression<'a, I>(iter: &mut Peekable<I>) -> Result<'a>
where
    I: Iterator<Item = &'a Token<'a>>,
{
    comma(iter)
}

fn comma<'a, I>(iter: &mut Peekable<I>) -> Result<'a>
where
    I: Iterator<Item = &'a Token<'a>>,
{
    let mut expr = equality(iter)?;

    while let Some(&token) = iter.peek() {
        expr = match token.token_type {
            TokenType::Comma => {
                iter.next();

                Box::new(Expression::Binary {
                    operator: *token,
                    left: expr,
                    right: equality(iter)?,
                })
            }
            _ => break,
        }
    }

    Ok(expr)
}

fn equality<'a, I>(iter: &mut Peekable<I>) -> Result<'a>
where
    I: Iterator<Item = &'a Token<'a>>,
{
    let mut expr = comparison(iter)?;

    while let Some(&token) = iter.peek() {
        expr = match token.token_type {
            TokenType::BangEqual | TokenType::EqualEqual => {
                iter.next();

                Box::new(Expression::Binary {
                    operator: *token,
                    left: expr,
                    right: comparison(iter)?,
                })
            }
            _ => break,
        };
    }

    Ok(expr)
}

fn comparison<'a, I>(iter: &mut Peekable<I>) -> Result<'a>
where
    I: Iterator<Item = &'a Token<'a>>,
{
    let mut expr = term(iter)?;

    while let Some(&token) = iter.peek() {
        expr = match token.token_type {
            TokenType::GreaterThan
            | TokenType::GreaterThanOrEqual
            | TokenType::LessThan
            | TokenType::LessThanOrEqual => {
                iter.next();

                Box::new(Expression::Binary {
                    operator: *token,
                    left: expr,
                    right: term(iter)?,
                })
            }
            _ => break,
        };
    }

    Ok(expr)
}

fn term<'a, I>(iter: &mut Peekable<I>) -> Result<'a>
where
    I: Iterator<Item = &'a Token<'a>>,
{
    let mut expr = factor(iter)?;

    while let Some(&token) = iter.peek() {
        expr = match token.token_type {
            TokenType::Plus | TokenType::Minus => {
                iter.next();

                Box::new(Expression::Binary {
                    operator: *token,
                    left: expr,
                    right: factor(iter)?,
                })
            }
            _ => break,
        };
    }

    Ok(expr)
}

fn factor<'a, I>(iter: &mut Peekable<I>) -> Result<'a>
where
    I: Iterator<Item = &'a Token<'a>>,
{
    let mut expr = unary(iter)?;

    while let Some(&token) = iter.peek() {
        expr = match token.token_type {
            TokenType::Asterisk | TokenType::Slash => {
                iter.next();

                Box::new(Expression::Binary {
                    operator: *token,
                    left: expr,
                    right: unary(iter)?,
                })
            }
            _ => break,
        };
    }

    Ok(expr)
}

fn unary<'a, I>(iter: &mut Peekable<I>) -> Result<'a>
where
    I: Iterator<Item = &'a Token<'a>>,
{
    let expr = if let Some(&token) = iter.peek() {
        match token.token_type {
            TokenType::Bang | TokenType::Minus => {
                iter.next();

                Box::new(Expression::Unary {
                    operator: *token,
                    right: unary(iter)?,
                })
            }
            _ => primary(iter)?,
        }
    } else {
        panic!("AAAAAHHH")
    };

    Ok(expr)
}

fn primary<'a, I>(iter: &mut Peekable<I>) -> Result<'a>
where
    I: Iterator<Item = &'a Token<'a>>,
{
    let create_error = |position| Error::ParseError {
        message: "expected expression".to_string(),
        source_position: position,
    };

    let &token = iter.peek().ok_or_else(|| create_error((0, 0)))?;

    match token.token_type {
        TokenType::Number(n) => {
            iter.next();
            Ok(Box::new(Expression::Literal(LiteralValue::Number(n))))
        }
        TokenType::Str(s) => {
            iter.next();
            Ok(Box::new(Expression::Literal(LiteralValue::String(
                s.to_string(),
            ))))
        }
        TokenType::True => {
            iter.next();
            Ok(Box::new(Expression::Literal(LiteralValue::True)))
        }
        TokenType::False => {
            iter.next();
            Ok(Box::new(Expression::Literal(LiteralValue::False)))
        }
        TokenType::Nil => {
            iter.next();
            Ok(Box::new(Expression::Literal(LiteralValue::Nil)))
        }
        TokenType::LeftParen => {
            iter.next();
            let inner_expr = expression(iter)?;

            if let Some(&Token { token_type, .. }) = iter.peek() {
                if *token_type != TokenType::RightParen {
                    return Err(Error::ParseError {
                        message: "expected ')'".to_string(),
                        source_position: token.source_position,
                    });
                }
                iter.next();
            }

            Ok(Box::new(Expression::Grouping(inner_expr)))
        }
        _ => Err(create_error(token.source_position)),
    }
}

// Consume tokens until we hit a synchronization point. A synchronization point
// is either a semicolon or the start of a new statement (i.e. the keywork
// class, fun, var, etc.).
#[allow(dead_code)]
fn synchronize<'a, I>(iter: &mut Peekable<I>)
where
    I: Iterator<Item = &'a Token<'a>>,
{
    while let Some(&token) = iter.peek() {
        match token.token_type {
            TokenType::Semicolon => {
                iter.next();
                break;
            }
            TokenType::Class
            | TokenType::Fun
            | TokenType::Var
            | TokenType::For
            | TokenType::If
            | TokenType::While
            | TokenType::Print
            | TokenType::Return => break,
            _ => (),
        }
    }
}
