// TODO: Temporary, remove later
#![allow(dead_code)]

extern crate itertools;

pub mod loxi;

mod ast;
mod binary_tree;
mod lexer;
mod parser;
mod result;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
