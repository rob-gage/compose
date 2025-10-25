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


pub struct Function<'a> (&'a [Term]);



impl Function<'_> {

    pub const fn body(&self) -> &[Term] { self.0 }

    pub fn evaluate(
        &self,
        function_storage: &FunctionStorage,
        stack: &mut Stack
    ) -> Result<(), String> {
        for term in self.0 {
            match term {
                Term::Application (function_index) => {
                    let function: Function = function_storage.get(*function_index);
                    function.evaluate(function_storage, stack)?
                },
                Term::Combinator (combinator) => stack.evaluate_combinator(
                    &function_storage,
                    combinator.clone()
                ).map_err(str::to_string)?,
                Term::Data (data) => stack.push(data.clone()),
            }
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
    pub fn reserve(&mut self, length: usize) -> FunctionIndex {
        let start: usize = self.term_buffer.len(); // start of function in term buffer
        let end: usize = start + length; // end of function in term buffer
        self.functions.push(start..end);
        self.term_buffer.reserve(length);
        FunctionIndex (self.functions.len() - 1)
    }

    /// Stores a function in the `FunctionStorage` at a `FunctionIndex`
    pub fn store(&mut self, index: FunctionIndex, function: &[Term]) {
        let range: Range<usize> = self.functions[index.0].clone();
        debug_assert_eq!(range.end - range.start, function.len(), "Not enough space reserved in \
        `FunctionStorage` for function");
        self.term_buffer.splice(range, function.iter().cloned());
    }

}