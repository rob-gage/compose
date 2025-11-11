// Copyright Rob Gage 2025

use crate::{
    Term,
    Value,
};



/// An environment that stores defined `Function`s
pub struct Environment {
    term_buffer: Vec<Term>,
    term_slices: Vec<(usize, usize)>,
}

impl Environment {

    /// Creates a new `Environment`
    pub const fn new() -> Self {
        Self { term_buffer: Vec::new(), term_slices: Vec::new() }
    }

}



/// A `Function` that can be evaluated on a `VirtualMachine`
#[derive(Clone)]
pub enum Function<'a> {
    Contiguous (&'a [Term]),
    Composed (Vec<Term>)
}

impl<'a> Function<'_> {

    /// Returns the body of this function as a slice of `Term`s
    pub fn body(&self) -> &[Term] {
        match self {
            Self::Contiguous (terms) => terms,
            Self::Composed (terms) => terms
        }
    }
    
    /// Extends this `Function`'s body with a slice of `Term`s
    pub fn extended(self, terms: impl Iterator<Item = Term>) -> Self {
        let mut body: Vec<Term> = self.body().to_vec();
        body.extend(terms);
        Self::Composed (body)
    }
    
}



/// A reference to a `Function` in an `Environment`
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FunctionReference (usize);

impl FunctionReference {

    /// Gets the `Function` from its `Environment`
    pub fn get<'a>(&self, environment: &'a Environment) -> Function<'a> {
        let (start, end): (usize, usize) = environment.term_slices[self.0];
        Function::Contiguous(&environment.term_buffer[start..end])
    }

    /// Reserves a `FunctionReference` in an `Environment`
    pub fn reserve(environment: &mut Environment) -> Self {
        environment.term_slices.push((0, 0));
        Self (environment.term_slices.len() - 1)
    }

    /// Sets the body of this function using a slice of `Term`s
    pub fn set_body(&self, environment: &mut Environment, body: &[Term]) {
        let start: usize = environment.term_buffer.len();
        environment.term_buffer.extend_from_slice(body);
        let end: usize = environment.term_buffer.len();
        environment.term_slices[self.0] = (start, end);
    }

}



/// Represents a function that is treated as data on the `VirtualMachine`
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LambdaReference (Vec<usize>);

impl LambdaReference {

    /// Gets the lambda as a `Function` from its `Environment`
    pub fn get<'a>(&self, environment: &'a Environment) -> Function<'a> {
        let mut body: Vec<Term> = Vec::new();
        for &index in &self.0 {
            body.extend_from_slice(FunctionReference (index).get(environment).body());
        }
        Function::Composed (body)
    }

    /// Composes this lambda with another
    pub fn compose(mut self, other: LambdaReference) -> Self {
        self.0.extend(other.0);
        self
    }

    /// Creates a new `LambdaReference` from a `FunctionReference`
    pub fn from_function(
        function_reference: FunctionReference
    ) -> LambdaReference {
        Self (vec![function_reference.0])
    }

}