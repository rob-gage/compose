// Copyright Rob Gage 2025

use crate::{
    Function,
    Term,
};

/// An environment that stores defined `Function`s
pub struct Environment {
    term_buffer: Vec<Term>,
    function_slices: Vec<(usize, usize)>,
}

impl Environment {

    /// Creates a new `Environment`
    pub const fn new() -> Self {
        Self { term_buffer: Vec::new(), function_slices: Vec::new() }
    }

}

/// A reference to a `Function` in an `Environment`
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

    /// Sets the body of this function using a slice of `Term`s
    pub fn set_body(&self, environment: &mut Environment, body: &[Term]) {
        let start: usize = environment.term_buffer.len();
        environment.term_buffer.extend_from_slice(body);
        let end: usize = environment.term_buffer.len();
        environment.function_slices[self.0] = (start, end);
    }

}

/// A reference to a lambda in an `Environment`
pub type LambdaReference = FunctionReference<Vec<usize>>;

impl LambdaReference {

    /// Fetches the lambda as a `Function` from its `Environment`
    pub fn fetch<'a>(&self, environment: &'a Environment) -> Function<'a> {
        let mut body: Vec<Term> = Vec::new();
        for &index in &self.0 {
            body.extend_from_slice(FunctionReference (index).fetch(environment).body());
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
        FunctionReference (vec![function_reference.0])
    }

}