use std::fmt;
use std::result;
use lexer::SourcePosition;

pub type Result<T> = result::Result<T, Error>;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    SyntaxError {
        message: &'static str,
        source_position: SourcePosition,
    },
    MultipleErrors(Vec<Error>),
}

impl<'a> fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::SyntaxError { message: m, source_position: (l, c) } => {
                write!(f, "Syntax Error @ {}:{}: {}", l, c, m)
            }
            Error::MultipleErrors(ref errors) => {
                for error in errors {
                    try!(write!(f, "{}", error));
                }
                Ok(())
            }
        }
    }
}