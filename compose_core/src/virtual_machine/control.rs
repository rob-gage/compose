// Copyright Rob Gage 2025

use crate::{
    Environment,
    Function,
    Term,
    Value,
};
use smallvec::SmallVec;
use super::DataStack;


/// Describes how the `VirtualMachine` should manipulate its `ControlStack` after an
/// evaluation step
pub enum ControlAction<'vm> {
    /// Does nothing, continues evaluation
    Continue,
    /// Halts evaluation, and returns an error
    Error (String),
    // /// Folds a list into one value
    // FoldList {
    //     start: Value,
    //     list: Vec<Value>,
    //     function: Function<'vm>,
    // },
    // FilterList {
    //     list: Vec<Value>,
    //     function: Function<'vm>,
    // },
    // MapList {
    //     list: Vec<Value>,
    //     function: Function<'vm>,
    // },
    /// Pops a `ControlFrame` off the `ControlStack` before continuing evaluation
    Pop,
    /// Pushes a new `ControlFrame` to the `ControlStack` before continuing evaluation
    Push (Function<'vm>),
}



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
    pub fn execute_step(
        &mut self,
        data_stack: &mut DataStack,
        environment: &'a Environment,
    ) -> ControlAction<'a> {
        let Some (term) = self.function.body().get(self.index) else { return ControlAction::Pop };
        let action: ControlAction = match term {
            Term::Application (reference) => {
                let function: Function = reference.get(environment);
                ControlAction::Push (function)
            },
            Term::Combinator (combinator) => combinator.evaluate(data_stack, environment),
            Term::Data (data) => {
                data_stack.push(data.clone());
                ControlAction::Continue
            },
            Term::Recursion => ControlAction::Push (self.function.clone()),
        };
        self.index += 1;
        action
    }

}



/// The stack that stores the `ControlFrame`s used to represent function calls
pub struct ControlStack<'a> (SmallVec<[ControlFrame<'a>; 1024]>);

impl<'a> ControlStack<'a> {

    /// Create a new `ControlStack`
    pub fn new() -> Self { Self (SmallVec::new()) }

    /// Removes the `ControlFrame` from the top of this `ControlStack`
    pub fn pop_frame(&mut self) -> Option<ControlFrame<'a>> { self.0.pop() }

    /// Adds a new `ControlFrame` to this `ControlStack`
    pub fn push_frame(&mut self, frame: ControlFrame<'a>) { self.0.push(frame) }

}