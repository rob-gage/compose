// Copyright Rob Gage 2025

use std::cell::UnsafeCell;
use crate::Environment;
use super::{
    ControlStack,
    DataStack,
    Function,
    ControlAction,
    Term,
    VirtualMachine,
};

/// Represents a function being executed
pub struct ControlFrame<'vm> {
    /// The `Function` that was applied to create this `ControlFrame`
    function: Function<'vm>,
    /// The index of the next term to be evaluated in the `Function`
    index: usize,
}

impl<'a> ControlFrame<'a> {

    /// Creates a `ControlFrame` from `Term`s
    pub const fn from_function(function: Function<'a>) -> Self { Self { function, index: 0, } }

    /// Runs one step in the evaluation process for this `ControlFrame`
    pub fn run_step(
        &mut self,
        data_stack: &mut DataStack,
        environment: &'a Environment,
    ) -> ControlAction<'a> {
        let Some (term) = self.function.body().get(self.index)
        else { return ControlAction::Pop };
        let action: ControlAction = match term {
            Term::Application (reference) => {
                let function: Function = reference.fetch(environment);
                ControlAction::Push (function)
            },
            Term::Combinator (combinator) => combinator.evaluate(data_stack, environment),
            Term::Data (data) => {
                data_stack.push(data.clone());
                ControlAction::Continue
            },
            Term::Recursion => ControlAction::Push (self.function.clone()),
        };
        action
    }

}