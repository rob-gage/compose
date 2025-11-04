// Copyright Rob Gage 2025

use std::cell::UnsafeCell;
use super::{
    Function,
    ControlAction,
    DataStack,
    FunctionStorage,
    Term,
};

/// Represents a function being executed
pub struct ControlFrame<'a> {
    /// The `Function` that was applied to create this `ControlFrame`
    pub function: Function<'a>,
    /// The index of the next term to be evaluated in the `Function`
    index: UnsafeCell<usize>,
}

impl<'a> ControlFrame<'a> {

    /// Creates a `ControlFrame` from `Term`s
    pub const fn from_function(function: Function<'a>) -> Self {
        Self {
            function,
            index: UnsafeCell::new(0),
        }
    }

    /// Runs one step in the evaluation process for this `ControlFrame`
    pub fn run_step(
        &'_ self,
        data_stack: &mut DataStack,
        function_storage: &'_ FunctionStorage,
    ) -> ControlAction<'_> {
        let Some (term) = self.function.body().get(unsafe { *self.index.get() })
        else { return ControlAction::Pop };
        match term {
            Term::Application (function_index) => {
                let function: Function = function_storage.get(*function_index);
                ControlAction::Push (&self.function)
            },
            Term::Combinator (combinator) => match data_stack.evaluate_combinator(
                function_storage,
                combinator.clone()
            ) {
                Ok (_) => {
                    unsafe { *self.index.get() += 1}
                    ControlAction::Continue
                },
                Err (error) => ControlAction::Error (error.to_string()),
            },
            Term::Data (data) => {
                data_stack.push(data.clone());
                unsafe { *self.index.get() += 1}
                ControlAction::Continue
            },
            Term::Recursion =>
                ControlAction::Push (&self.function),
        }
    }

}