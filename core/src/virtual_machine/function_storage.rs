// Copyright Rob Gage 2025

use crate::{
    DataStack,
    Term
};
use std::{
    collections::HashMap,
    marker::PhantomData,
};
use super::Function;



/// Stores `Functions`
pub struct FunctionStorage<'a> {
    /// The stored function bodies
    function_bodies: HashMap<usize, Vec<Term>>,
    /// The next index to store a function at
    next_index: usize,
    phantom_data: PhantomData<&'a ()>
}

impl<'a> FunctionStorage<'a>  {

    /// Gets the body of a `Function` with a given `usize` index as `Term`s
    pub fn get(&'a self, index: usize) -> Function<'a> {
        Function::Contiguous (&self.function_bodies[&index])
    }

    /// Gets a composed function from a slice of `usize` indices
    pub fn get_composed(&self, indices: &[usize]) -> Function<'a> {
        let mut terms: Vec<Term> = Vec::new();
        for index in indices {
            terms.extend(self.get(*index).body().iter().cloned());
        }
        Function::Composed (terms)
    }

    /// Creates a new `FunctionStorage`
    pub fn new() -> Self {
         Self {
            function_bodies: HashMap::new(),
            next_index: 0,
             phantom_data: PhantomData
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





/// A function inside a `FunctionStorage`
pub struct FunctionReference<T = usize> (T);

impl<T> FunctionReference<T> {

    /// Helper method to evaluate the `Term`s in a `Function`
    fn evaluate_terms(
        function_storage: &FunctionStorage,
        stack: &mut DataStack,
        terms: &[Term],
    ) -> Result<(), String> {
        for term in terms {
            match term {
                Term::Application (index) => {
                    let function: FunctionReference = FunctionReference::from_function_index(*index);
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

impl FunctionReference<usize> {

    /// Evaluates a function using a `FunctionStorage` and a `Stack`
    pub fn evaluate(
        &self,
        function_storage: &FunctionStorage,
        stack: &mut DataStack
    ) -> Result<(), String> {
        unimplemented!()
    }

    /// Creates a new `Function` from an index into a `FunctionStorage`
    pub const fn from_function_index(index: usize) -> Self { FunctionReference(index) }

}

impl FunctionReference<Vec<usize>> {

    /// Evaluates a function using a `FunctionStorage` and a `Stack`
    pub fn evaluate(
        &self,
        function_storage: &FunctionStorage,
        stack: &mut DataStack
    ) -> Result<(), String> {
        unimplemented!()
    }

    /// Creates a new `Function` from multiple composed indices into a `FunctionStorage`
    pub fn from_function_indices(indices: &[usize]) -> Self {
        unimplemented!()
    }

}