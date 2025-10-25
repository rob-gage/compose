// Copyright Rob Gage 2025

use std::{
    cell::UnsafeCell,
    ops::Range,
    sync::Arc,
};
use crate::{
    Stack,
    Term,
};

/// A function that can be evaluated
pub struct Function<'a> (&'a [Term]);

impl Function<'_> {

    /// Evaluate the `Function`
    pub fn evaluate(
        &self,
        function_storage: &FunctionStorage,
        stack: &mut Stack
    ) -> Result<(), String> {
        for term in self.0 {
            term.evaluate(&function_storage, stack)?;
        }
        Ok (())
    }

}



/// The index of a `Function` in a `FunctionStorage`
#[derive(Clone, Copy, Debug)]
pub struct FunctionIndex (usize);



/// Stores resolved function definitions
pub struct FunctionStorage {
    /// The functions in this `FunctionStorage`represented by their range in the term buffer
    functions: Vec<Range<usize>>,
    /// The buffer storing the terms composing the bodies of these functions
    term_buffer: Vec<Term>,
}

impl FunctionStorage {

    /// Gets the `&[Term]` body of a function from a `FunctionStorage`
    pub fn get(&self, index: FunctionIndex) -> Function {
        let range: Range<usize> = self.functions[index.0].clone();
        Function (&self.term_buffer[range])
    }

    /// Create a new `FunctionStorage`
    pub fn new() -> Self { Self {
        functions: vec![],
        term_buffer: vec![],
    } }

    /// Reserves a place to store a function with a given length in terms
    pub const fn reserve(&mut self, length: usize) -> FunctionIndex {
        let start: usize = self.term_buffer.len(); // start of function in term buffer
        let end: usize = start + length; // end of function in term buffer
        self.functions.push(start..end);
        self.term_buffer.reserve(length);
        FunctionIndex (self.functions.len() - 1)
    }

    /// Stores a new function in this `NamespaceStorage`, returning its index
    pub fn store_function(&mut self, body: &[Term]) -> FunctionIndex {
        let start: usize = self.term_buffer.len(); // start of function in term buffer
        let end: usize = start + body.len(); // end of function in term buffer
        self.term_buffer.extend_from_slice(body); // append the terms to the buffer
        let range: Range<usize> = start..end; // store the range for this function in term buffer
        self.functions.push(range);
        FunctionIndex::new(self.functions.len() - 1)
    }

}