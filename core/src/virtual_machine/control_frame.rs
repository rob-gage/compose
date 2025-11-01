// Copyright Rob Gage 2025

use crate::Term;
use super::{
    ControlAction,
    DataStack,
    FunctionStorage,
};

/// Represents a function being executed
pub struct ControlFrame<'a> {
    /// The `Term`s making up this `Function`
    terms: &'a [Term],
    /// The next term to be evaluated
    term_counter: usize,
}

impl<'a> ControlFrame<'a> {

    /// Evaluates one `Term` in the `ControlFrame`, and increments the `TermCounter`
    pub fn step(
        &'a mut self,
        data_stack: &mut DataStack,
        function_storage: &'a FunctionStorage
    ) -> Result<ControlAction<'a>, String> {
        match &self.terms[self.term_counter] {
            Term::Application (index) => {
                let body: &'a [Term] = function_storage.get_body(*index);
                self.term_counter += 1;
                Ok (ControlAction::Push (body))
            },
            Term::Combinator (combinator) => {
                data_stack.evaluate_combinator(&function_storage, combinator.clone())?;
                self.term_counter += 1;
                Ok (ControlAction::Continue)
            },
            Term::Data (data) => {
                data_stack.push(data.clone());
                self.term_counter += 1;
                Ok (ControlAction::Continue)
            },
            Term::Recursion => {
                self.term_counter += 1;
                Ok (ControlAction::Push (self.terms))
            },
        }

    }

    /// Creates a `ControlFrame` from `Term`s
    pub const fn from_terms(terms: &'a [Term]) -> Self {
        Self {
            terms,
            term_counter: 0,
        }
    }

}