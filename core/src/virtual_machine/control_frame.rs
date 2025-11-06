// Copyright Rob Gage 2025

use std::cell::UnsafeCell;
use super::{
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
        virtual_machine: &'a mut VirtualMachine<'a>,
    ) -> ControlAction<'a> {
        let Some (term) = self.function.body().get(unsafe { *self.index.get() })
        else { return ControlAction::Pop };
        match term {
            Term::Application (reference) => {
                let function: Function = reference.fetch(&virtual_machine.environment);
                ControlAction::Push (function)
            },
            Term::Combinator (combinator) => match combinator.evaluate(virtual_machine) {
                Ok (_) => {
                    unsafe { *self.index.get() += 1}
                    ControlAction::Continue
                },
                Err (error) => ControlAction::Error (error.to_string()),
            },
            Term::Data (data) => {
                virtual_machine.data_stack.push(data.clone());
                unsafe { *self.index.get() += 1}
                ControlAction::Continue
            },
            Term::Recursion => ControlAction::Push (self.function.clone()),
        }
    }

}