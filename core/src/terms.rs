// Copyright Rob Gage 2025

use crate::{
    Combinator,
    Data,
    FunctionIndex,
};

/// A concatenative programming term that can represent data or operations
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Term
where
    Self: Sized
{

    /// Application of a named function
    Application (FunctionIndex),

    /// A combinator that performs an operation on the stack
    Combinator (Combinator),

    /// Data as a term that will be pushed to the `Stack`
    Data (Data),

}



/// Represents a sequence of `Term`s
pub trait TermSequence<'a> {

    /// The type used to represent positions of `Term`s within the `TermSequence`
    type Index: Clone + Copy;

    /// The start index used for the implementor
    const START: Self::Index;

    /// Returns the next `Term` in this `TermSequence` if it is not empty
    fn next(&self, index: Self::Index) -> (Option<&'a Term>, Self::Index);

}

impl<'a> TermSequence<'a> for &'a [Term] {

    type Index = usize;

    const START: Self::Index = 0;

    fn next(&self, index: usize) -> (Option<&'a Term>, usize) {
        if let Some (term) = self.get(index) {
            (Some (term), index + 1)
        } else { (None, index) }
    }

}

impl <'a> TermSequence<'a> for Vec<&'a [Term]> {

    type Index = (usize, usize);

    const START: Self::Index = (0, 0);

    fn next(&self, index: (usize, usize)) -> (Option<&'a Term>, (usize, usize)) {
        if let Some (slice) = self.get(index.0) {
            if let Some (term) = slice.get(index.1) {
                (Some (term), (index.0, index.1 + 1))
            } else {
                self.next((index.0 + 1, 0))
            }
        } else { (None, index) }
    }

}