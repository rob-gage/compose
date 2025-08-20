// Copyright Rob Gage 2025

use std::ops::Range;
use super::Term;

/// Stores resolved function definitions
pub struct FunctionStorage {
    /// The functions in this `FunctionStorage`represented by their range in the term buffer
    functions: Vec<Range<usize>>,
    /// The buffer storing the terms composing the bodies of these functions
    term_buffer: Vec<Term>,
}

impl FunctionStorage {

    /// Gets the `&[Term]` body of a function from a `FunctionStorage`
    pub fn get_body(&self, index: usize) -> &[Term] {
        let range: Range<usize> = self.functions[index].clone();
        &self.term_buffer[range]
    }

    /// Create a new `FunctionStorage`
    pub fn new() -> Self { Self {
        functions: vec![],
        term_buffer: vec![],
    } }

    /// Returns the index of the next function to be added to this `FunctionStorage`
    pub const fn next_index(&self) -> usize {
        self.functions.len()
    }

    /// Stores a new function in this `NamespaceStorage`, returning its index
    pub fn store_function(&mut self, body: &[Term]) -> usize {
        let start: usize = self.term_buffer.len(); // start of function in term buffer
        let end: usize = start + body.len(); // end of function in term buffer
        self.term_buffer.extend_from_slice(body); // append the terms to the buffer
        let range: Range<usize> = start..end; // store the range for this function in term buffer
        self.functions.push(range);
        self.functions.len() - 1
    }

}