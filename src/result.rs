use crate::lexer::SourcePosition;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    SyntaxError {
        message: String,
        source_position: SourcePosition,
    },
    MultipleErrors(Vec<Error>),
}

impl std::error::Error for Error {}

impl<'a> fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::SyntaxError {
                message: ref m,
                source_position: (l, c),
            } => write!(f, "Syntax Error [ln: {}, col: {}]: {}", l, c, m),
            Error::MultipleErrors(ref errors) => {
                for error in errors {
                    write!(f, "{}", error)?;
                }
                Ok(())
            }
        }
    }
}
