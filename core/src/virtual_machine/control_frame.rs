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
pub struct ControlFrame<'e> {
    /// The `Function` that was applied to create this `ControlFrame`
    pub function: Function<'e>,
    /// The index of the next term to be evaluated in the `Function`
    index: UnsafeCell<usize>,
}

impl<'e> ControlFrame<'e> {

    /// Creates a `ControlFrame` from `Term`s
    pub const fn from_function(function: Function<'e>) -> Self {
        Self {
            function,
            index: UnsafeCell::new(0),
        }
    }

    /// Runs one step in the evaluation process for this `ControlFrame`
    pub fn run_step(
        &'e self,
        stack: &'e mut DataStack,
        environment: &'e Environment<'e>,
    ) -> ControlAction<'e> {
        let Some (term) = self.function.body().get(unsafe { *self.index.get() })
        else { return ControlAction::Pop };
        let action: ControlAction = match term {
            Term::Application (reference) => {
                let function: Function = reference.fetch(environment);
                ControlAction::Push (function)
            },
            Term::Combinator (combinator) => combinator.evaluate(stack, environment),
            Term::Data (data) => {
                stack.push(data.clone());
                ControlAction::Continue
            },
            Term::Recursion => ControlAction::Push (self.function.clone()),
        };
        unsafe { *self.index.get() += 1}
        action
    }

}