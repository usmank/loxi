use std::result;
use lexer::SourcePosition;

pub type Result<T> = result::Result<T, Error>;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    SyntaxError(SourcePosition),
    MultipleErrors(Vec<Error>),
}