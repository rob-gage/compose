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
pub struct Function {
    /// The `FunctionStorage` containing this `Function`
    function_storage: Arc<UnsafeCell<FunctionStorage>>,
    /// The index of this `Function` in its `FunctionStorage`
    index: usize,
}

impl Function {

    /// Returns the body of `Term`s composing this function
    pub fn body(&self) -> &[Term] {
        let storage: &mut FunctionStorage = unsafe { &mut *self.function_storage.get() };
        storage.get_body(self.index)
    }

    /// Evaluate the `Function`
    pub fn evaluate(&self, stack: &mut Stack) -> Result<(), String> {
        let storage: &mut FunctionStorage = unsafe { &mut *self.function_storage.get() };
        stack.evaluate_function_body(storage, self.body())
            .map_err(|s| s.to_string())
    }

    /// Create a new `Function` from a `FunctionStorage` and its index within the storage
    pub fn new(
        function_storage: &Arc<UnsafeCell<FunctionStorage>>,
        index: usize
    ) -> Self { Self {
        function_storage: function_storage.clone(),
        index,
    } }

}



/// The index of a `Function` in a `FunctionStorage`
pub struct FunctionIndex (usize);

impl FunctionIndex {

    /// Returns the `FunctionIndex` as a `usize`
    const fn index(&self) -> usize { self.0 }

    /// Creates a new `FunctionIndex`
    const fn new(index: usize) -> Self { Self (index) }

}



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
    pub const fn next_index(&self) -> FunctionIndex { FunctionIndex::new(self.functions.len()) }

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