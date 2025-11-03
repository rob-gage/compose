// Copyright Rob Gage 2025

use super::Term;

/// A function that takes a stack as its input and produces a stack as its output
#[derive(Clone)]
pub enum Function<'a> {
    Contiguous (&'a [Term]),
    Composed (Vec<Term>)
}

impl <'a> Function<'a> {

    /// Returns the `Term`s making up the body of this `Function`
    pub fn body(&self) -> &[Term] {
        match self {
            Function::Contiguous (body) => body,
            Function::Composed (body) => body
        }
    }

}