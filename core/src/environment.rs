// Copyright Rob Gage 2025

use crate::{
    Function,
    Term,
};
use std::{
    collections::HashMap,
    marker::PhantomData,
};

/// An environment that stores defined `Function`s
pub struct Environment<'a> {
    term_buffer: Vec<Term>,
    function_slices: Vec<(usize, usize)>,
    phantom_data: PhantomData<&'a ()>
}

impl<'a> Environment<'a> {

    /// Creates a new `Environment`
    pub const fn new() -> Self {
        Self { term_buffer: Vec::new(), function_slices: Vec::new(), phantom_data: PhantomData }
    }

    /// Stores a `&[Term]` and returns its index as a `usize`
    pub fn store_function(&mut self, index: usize, terms: &[Term]) {
        unimplemented!()
    }

}

/// A reference to a `Function` in an `Environment`
pub struct FunctionReference<T = usize> (T);

impl FunctionReference {

    /// Fetches the `Function` from its `Environment`
    pub fn fetch<'a>(&self, environment: &'a Environment) -> Function<'a> {
        let (start, end): (usize, usize) = environment.function_slices[self.0];
        Function::Contiguous(&environment.term_buffer[start..end])
    }

    /// Reserves a `FunctionReference` in an `Environment`
    pub fn reserve(environment: &mut Environment) -> Self {
        environment.function_slices.push((0, 0));
        Self (environment.function_slices.len() - 1)
    }

    /// Creates a new `FunctionReference` to be used for composed lambdas
    pub const fn composed() -> FunctionReference<Vec<usize>>{
        FunctionReference (vec![])
    }

}

impl FunctionReference<Vec<usize>> {

    /// Fetches the `Function` from its `Environment`
    pub fn fetch<'a>(&self, environment: &'a Environment) -> Function<'a> {
        let mut body: Vec<Term> = Vec::new();
        for &index in &self.0 {
            body.extend_from_slice(FunctionReference (index).fetch(environment).body());
        }
        Function::Composed (body)
    }

    /// Joins a `FunctionReference` to this composed `FunctionReference`
    pub fn join(mut self, other: FunctionReference) -> Self {
        self.0.push(other.0);
        self
    }

}