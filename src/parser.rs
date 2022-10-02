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
//            | "+" | "-"  | "*" | "/" | "," | "?"
//            | ":"
//
// PRECEDENCE (Lowest to highest)
// Name         Operators   Associates
// ----         ---------   ----------
// Comma        ,           Left
// Ternary      ? :         Right
// Equality     == !=       Left
// Comparison   > >= < <=   Left
// Term         - +         Left
// Factor       / *         Left
// Unary        ! -         Right
//
// STRATIFIED GRAMMAR
// expression → comma
// comma      → ternary ( "," ternary )*
// ternary    → ( equality "?" ternary ":" ternary ) | equality
// equality   → comparison ( ( "==" | "!=" ) comparison )*
// comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )*
// term       → factor ( ( "+" | "-" ) factor )*
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
    let mut expr = ternary(iter)?;

    while let Some(&token) = match_token(iter, TokenType::Comma) {
        expr = Box::new(Expression::Binary {
            operator: token,
            left: expr,
            right: ternary(iter)?,
        });
    }

    Ok(expr)
}

fn ternary<'a, I>(iter: &mut Peekable<I>) -> Result<'a>
where
    I: Iterator<Item = &'a Token<'a>>,
{
    let mut expr = equality(iter)?;

    if let Some(&token) = match_token(iter, TokenType::QuestionMark) {
        let then_expr = ternary(iter)?;

        if let Some(_) = match_token(iter, TokenType::Colon) {
            let else_expr = ternary(iter)?;

            expr = Box::new(Expression::Ternary {
                operator: token,
                left: expr,
                middle: then_expr,
                right: else_expr,
            })
        }
    }

    Ok(expr)
}

fn equality<'a, I>(iter: &mut Peekable<I>) -> Result<'a>
where
    I: Iterator<Item = &'a Token<'a>>,
{
    let mut expr = comparison(iter)?;

    while let Some(&token) = match_token_any(iter, &[TokenType::BangEqual, TokenType::EqualEqual]) {
        expr = Box::new(Expression::Binary {
            operator: token,
            left: expr,
            right: comparison(iter)?,
        });
    }

    Ok(expr)
}

fn comparison<'a, I>(iter: &mut Peekable<I>) -> Result<'a>
where
    I: Iterator<Item = &'a Token<'a>>,
{
    let mut expr = term(iter)?;

    let tokens_to_match = [
        TokenType::GreaterThan,
        TokenType::GreaterThanOrEqual,
        TokenType::LessThan,
        TokenType::LessThanOrEqual,
    ];

    while let Some(&token) = match_token_any(iter, &tokens_to_match) {
        expr = Box::new(Expression::Binary {
            operator: token,
            left: expr,
            right: term(iter)?,
        });
    }

    Ok(expr)
}

fn term<'a, I>(iter: &mut Peekable<I>) -> Result<'a>
where
    I: Iterator<Item = &'a Token<'a>>,
{
    let mut expr = factor(iter)?;

    while let Some(&token) = match_token_any(iter, &[TokenType::Plus, TokenType::Minus]) {
        expr = Box::new(Expression::Binary {
            operator: token,
            left: expr,
            right: factor(iter)?,
        })
    }

    Ok(expr)
}

fn factor<'a, I>(iter: &mut Peekable<I>) -> Result<'a>
where
    I: Iterator<Item = &'a Token<'a>>,
{
    let mut expr = unary(iter)?;

    while let Some(&token) = match_token_any(iter, &[TokenType::Asterisk, TokenType::Slash]) {
        expr = Box::new(Expression::Binary {
            operator: token,
            left: expr,
            right: unary(iter)?,
        });
    }

    Ok(expr)
}

fn unary<'a, I>(iter: &mut Peekable<I>) -> Result<'a>
where
    I: Iterator<Item = &'a Token<'a>>,
{
    if let Some(&token) = match_token_any(iter, &[TokenType::Bang, TokenType::Minus]) {
        Ok(Box::new(Expression::Unary {
            operator: token,
            right: unary(iter)?,
        }))
    } else {
        primary(iter)
    }
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

            if match_token(iter, TokenType::RightParen).is_none() {
                return Err(Error::ParseError {
                    message: "expected ')'".to_string(),
                    source_position: token.source_position,
                });
            }

            Ok(Box::new(Expression::Grouping(inner_expr)))
        }
        _ => Err(create_error(token.source_position)),
    }
}

// Peek ahead and check if the token type matches the specified 'token_type'.
// Advance the iterator and return 'Some(token)' if true, and 'None' otherwise.
fn match_token<'a, I>(iter: &mut Peekable<I>, token_type: TokenType) -> Option<&'a Token<'a>>
where
    I: Iterator<Item = &'a Token<'a>>,
{
    if iter.peek()?.token_type != token_type {
        return None;
    }

    iter.next()
}

// Peek ahead and check if the token type matches any of the specified
// 'token_types'.  Advance the iterator and return 'Some(token)' if true, and
// 'None' otherwise.
fn match_token_any<'a, I>(
    iter: &mut Peekable<I>,
    token_types: &[TokenType],
) -> Option<&'a Token<'a>>
where
    I: Iterator<Item = &'a Token<'a>>,
{
    if !token_types.contains(&iter.peek()?.token_type) {
        return None;
    }

    iter.next()
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
