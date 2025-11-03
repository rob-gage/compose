// Copyright Rob Gage 2025

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
    index: usize,
}

impl<'a> ControlFrame<'a> {

    /// Creates a `ControlFrame` from `Term`s
    pub const fn from_function(function: Function<'a>) -> Self {
        Self {
            function,
            index: 0,
        }
    }

    /// Runs one step in the evaluation process for this `ControlFrame`
    pub fn run_step(
        mut self,
        data_stack: &mut DataStack,
        function_storage: &'a FunctionStorage,
    ) -> ControlAction<'a> {
        let Some (term) = self.function.body().get(self.index)
        else { return ControlAction::Pop };
        match term {
            Term::Application (function_index) => {
                let function: Function = function_storage.get(*function_index);
                ControlAction::Push (function)
            },
            Term::Combinator (combinator) => match data_stack.evaluate_combinator(
                function_storage,
                combinator.clone()
            ) {
                Ok (_) => {
                    self.index += 1;
                    ControlAction::Continue (self)
                },
                Err (error) => ControlAction::Error (error.to_string()),
            },
            Term::Data (data) => {
                data_stack.push(data.clone());
                self.index += 1;
                ControlAction::Continue (self)
            },
            Term::Recursion => ControlAction::Push (self.function.clone()),
        }
    }

}