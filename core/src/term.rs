// Copyright Rob Gage 2025

use crate::{
    Combinator,
    Data,
    Function,
    FunctionIndex,
    FunctionStorage,
    Stack,
};

/// A concatenative programming term that can represent data or operations
#[derive(Clone, Debug)]
pub enum Term {

    /// Application of a named function
    ///
    /// The `isize` represents the index in the function storage of the function, or it represents
    /// recursion of the named function the application is in if it is `isize::MAX`. Recursions
    /// are transformed into negative `isize` indices, pointing to functions stored at runtime
    Application (FunctionIndex),

    /// A combinator that performs an operation on the stack
    Combinator (Combinator),

    /// Data as a term that will be pushed to the `Stack`
    Data (Data),

}

impl Term {

    /// Evaluate the `Term`
    pub fn evaluate(
        &self,
        function_storage: &FunctionStorage,
        stack: &mut Stack
    ) -> Result<(), String> {
        match self {

            Term::Application (function_index) => {
                let function: Function = function_storage.get(*function_index);
                function.evaluate(function_storage, stack)
            },

            Term::Combinator (combinator) => stack.evaluate_combinator(
                &function_storage,
                combinator.clone()
            ).map_err(str::to_string),

            Term::Data (data) => {
                stack.push(data.clone());
                Ok(())
            }

        }
    }

}