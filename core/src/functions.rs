// Copyright Rob Gage 2025

use crate::{
    Stack,
    Term
};
use std::collections::HashMap;

/// A function inside a `FunctionStorage`
pub struct Function<T = usize> (T);

impl<T> Function<T> {

    /// Helper method to evaluate the `Term`s in a `Function`
    fn evaluate_terms(
        function_storage: &FunctionStorage,
        stack: &mut Stack,
        terms: &[Term],
    ) -> Result<(), String> {
        for term in terms {
            match term {
                Term::Application (index) => {
                    let function: Function = Function::from_function_index(*index);
                    function.evaluate(function_storage, stack)?;
                },
                Term::Combinator (combinator) => stack.evaluate_combinator(
                    &function_storage,
                    combinator.clone()
                )?,
                Term::Data (data) => stack.push(data.clone()),
                Term::Recursion => Self::evaluate_terms(function_storage, stack, terms)?,
            }
        }
        Ok (())
    }

}

impl Function<usize> {

    /// Evaluates a function using a `FunctionStorage` and a `Stack`
    pub fn evaluate(
        &self,
        function_storage: &FunctionStorage,
        stack: &mut Stack
    ) -> Result<(), String> {
        let terms: &[Term] = function_storage.get_body(self.0);
        Self::evaluate_terms(function_storage, stack, terms)?;
        Ok (())
    }

    /// Creates a new `Function` from an index into a `FunctionStorage`
    pub const fn from_function_index(index: usize) -> Self { Function (index) }

}

impl Function<Vec<usize>> {

    /// Evaluates a function using a `FunctionStorage` and a `Stack`
    pub fn evaluate(
        &self,
        function_storage: &FunctionStorage,
        stack: &mut Stack
    ) -> Result<(), String> {
        for index in self.0.iter() {
            let terms: &[Term] = function_storage.get_body(*index);
            Self::evaluate_terms(function_storage, stack, terms)?;
        }
        Ok (())
    }

    /// Creates a new `Function` from multiple composed indices into a `FunctionStorage`
    pub fn from_function_indices(indices: &[usize]) -> Self {
        Function (indices.to_vec())
    }

}



/// Stores `Functions`
pub struct FunctionStorage {
    /// The stored function bodies
    function_bodies: HashMap<usize, Vec<Term>>,
    /// The next index to store a function at
    next_index: usize,
}

impl FunctionStorage  {

    /// Gets the body of a `Function` with a given `usize` index as `Term`s
    fn get_body(&self, index: usize) -> &[Term] {
        &self.function_bodies.get(&index)
            .expect("`Function` was originally from this `FunctionStorage` and not another, and \
            was never removed")
    }

    /// Creates a new `FunctionStorage`
    pub fn new() -> FunctionStorage {
        FunctionStorage {
            function_bodies: HashMap::new(),
            next_index: 0,
        }
    }

    /// Removes a `Function` from the `FunctionStorage`
    pub fn remove(&mut self, index: usize) {
        self.function_bodies.remove(&index);
    }

    /// Reserves a `usize` index to store a function body
    pub fn reserve(&mut self) -> usize {
        let index: usize = self.next_index;
        self.next_index += 1;
        index
    }

    /// Stores a `&[Term]` and returns its index as a `usize`
    pub fn store(&mut self, index: usize, function_body: &[Term]) {
        self.function_bodies.insert(index, function_body.to_vec());
    }

}