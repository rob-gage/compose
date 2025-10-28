// Copyright Rob Gage 2025


use crate::{
    Stack,
    Term,
    TermSequence,
};
use std::{
    marker::PhantomData,
    ops::Range,
};


/// A function that can be evaluated on a `VirtualMachine`
pub struct Function<'a, TS>
where
    TS: TermSequence<'a>
{
    /// The terms making up the function body
    body: TS,
    _phantom: PhantomData<&'a ()>,
}



impl<'a, TS> Function<'a, TS>
where
    TS: TermSequence<'a>
{

    pub fn evaluate(
        &self,
        function_storage: &'a FunctionStorage,
        stack: &mut Stack
    ) -> Result<(), String> {
        let mut index: TS::Index = TS::START;
        loop {
            let (Some (term), new_index) = self.body.next(index) else { break };
            match term {
                Term::Application (function_index) => {
                    let function: Function<'a, &'a [Term]> = function_storage.get(*function_index);
                    function.evaluate(function_storage, stack)?
                },
                Term::Combinator (combinator) => stack.evaluate_combinator(
                    &function_storage,
                    combinator.clone()
                ).map_err(str::to_string)?,
                Term::Data (data) => stack.push(data.clone()),
            }
            index = new_index
        }
        Ok (())
    }

}



/// The index of a `Function` in a `FunctionStorage`
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FunctionIndex (usize);



/// Stores resolved function definitions
pub struct FunctionStorage<'a> {
    /// The functions in this `FunctionStorage`represented by their range in the term buffer
    functions: Vec<Range<usize>>,
    /// The buffer storing the terms composing the bodies of these functions
    term_buffer: Vec<Term>,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> FunctionStorage<'a> {

    /// Gets the `&[Term]` body of a function from a `FunctionStorage`
    pub fn get(&'a self, index: FunctionIndex) -> Function<'a, &'a [Term]> {
        let range: Range<usize> = self.functions[index.0].clone();
        Function { body: &self.term_buffer[range], _phantom: PhantomData }
    }

    pub fn get_composed(&'a self, indices: &[FunctionIndex]) -> Function<'a, Vec<&'a [Term]>> {
        Function {
            body: indices.iter()
                .map(|index| self.get(*index).body)
                .collect(),
            _phantom: PhantomData,
        }
    }

    /// Create a new `FunctionStorage`
    pub fn new() -> Self {
        Self { functions: vec![], term_buffer: vec![], _phantom: PhantomData }
    }

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