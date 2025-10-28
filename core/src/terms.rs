// Copyright Rob Gage 2025

use crate::{
    Combinator,
    Data,
    FunctionIndex,
};

/// A concatenative programming term that can represent data or operations
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Term {

    /// Application of a named function
    Application (FunctionIndex),

    /// A combinator that performs an operation on the stack
    Combinator (Combinator),

    /// Data as a term that will be pushed to the `Stack`
    Data (Data),

}



/// Represents a sequence of `Term`s
pub trait TermSequence<'a> {

    /// Returns the next `Term` in this `TermSequence` if it is not empty
    fn next(&mut self) -> Option<&'a Term>;

}

impl<'a> TermSequence<'a> for (&'a [Term], usize) {
    fn next(&mut self) -> Option<&'a Term> {
        if let Some (output) = self.0.get(self.1) {
            self.1 += 1;
            Some (output)
        } else { None }
    }

}

impl <'a> TermSequence<'a> for (&[&'a [Term]], usize, usize) {
    fn next(&mut self) -> Option<&'a Term> {
        loop {
            let slice: &'a [Term] = self.0.get(self.1)?;
            if let Some (term) = slice.get(self.2) {
                self.2 += 1;
                return Some (term);
            } else {
                self.1 += 1;
                self.2 = 0;
                continue;
            }
        }
    }

}