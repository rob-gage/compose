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
        &'a self,
        stack: &'a mut DataStack,
        environment: &'a Environment,
    ) -> ControlAction<'a> {
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