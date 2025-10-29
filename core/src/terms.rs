// Copyright Rob Gage 2025

use crate::{Combinator, Data, Stack};
use std::{
    iter::repeat,
    marker::PhantomData,
};

/// A concatenative programming term that can represent data or operations
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Term
where
    Self: Sized
{

    /// Application of a named function
    Application (TermSequenceReference),

    /// A combinator that performs an operation on the stack
    Combinator (Combinator),

    /// Data as a term that will be pushed to the `Stack`
    Data (Data),

}



/// A buffer that stores `Term`s
pub struct TermBuffer (Vec<Term>);

impl TermBuffer {

    /// Gets a slice of `Term`s from this `TermBuffer`
    pub fn get(&self, index: TermSequenceReference) -> TermSequence
    { TermSequence::Borrowed (&self.0[index.0..index.1]) }

    /// Gets a slice of `Term`s composed of multiple `TermBufferIndex`s
    pub fn get_composed(&self, indices: &[TermSequenceReference]) -> TermSequence {
        let mut terms: Vec<Term> = Vec::new();
        for index in indices {
            let sequence: TermSequence = self.get(*index);
            terms.extend_from_slice(sequence.terms());
        }
        TermSequence::Owned (terms)
    }

    /// Creates a new `TermBuffer`
    pub const fn new() -> Self { Self (Vec::new()) }

    /// Reserves space for a sequence of terms with a given length
    pub fn reserve(&mut self, length: usize) -> TermSequenceReference {
        let start: usize = self.0.len();
        self.0.extend(repeat(Term::Application (TermSequenceReference(0, 0))).take(length));
        let end: usize = self.0.len();
        TermSequenceReference (start, end)
    }

    /// Stores a slice of `Term`s in this `TermBuffer` at a given `TermBufferIndex`
    pub fn store(&mut self, index: TermSequenceReference, terms: &[Term])
    { self.0.splice(index.0..index.1, terms.iter().cloned()); }

}



/// An index to a `TermSequence` inside a `TermBuffer`
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TermSequenceReference (usize, usize);



/// A sequence of `Term`s
pub enum TermSequence<'a> {
    Borrowed (&'a [Term]),
    Owned (Vec<Term>),
}

impl<'a> TermSequence<'a> {

    /// Returns the `Term`s making up this `TermSequence` as a slice
    pub fn terms(&'a self) -> &'a [Term] {
        match self {
            TermSequence::Borrowed (slice) => slice,
            TermSequence::Owned (slice) => slice,
        }
    }

    /// Evaluates this `TermSequence` on a `Stack` with a `TermBuffer` for context
    pub fn evaluate(&self, term_buffer: &TermBuffer, stack: &mut Stack) -> Result<(), String> {
        for term in self.terms() {
            match term {
                Term::Application (function_index) => {
                    let terms: Self = term_buffer.get(*function_index);
                    terms.evaluate(term_buffer, stack)?
                },
                Term::Combinator (combinator) => stack.evaluate_combinator(
                    &term_buffer,
                    combinator.clone()
                )?,
                Term::Data (data) => stack.push(data.clone()),
            }
        }
        Ok (())
    }

}