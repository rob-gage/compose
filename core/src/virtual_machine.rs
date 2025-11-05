// Copyright Rob Gage 2025

pub mod data;
pub mod data_stack;
mod function;
pub mod function_storage;
pub mod terms;

use smallvec::SmallVec;
use std::cell::UnsafeCell;
use crate::Environment;
use data_stack::DataStack;
use terms::Term;

pub use data::Data;
pub use function::Function;

/// A virtual machine used for evaluation of Compose programs and functions
pub struct VirtualMachine<'a> {
    control_stack: ControlStack<'a>,
    data_stack: DataStack,
    function_storage: Environment,
}

impl<'a> VirtualMachine<'a> {
    
    /// Adds data to the stack of this `VirtualMachine`
    pub fn add_data(&mut self, data: &[Data]) {
        for item in data {
            self.data_stack.push(item.clone());
        }
    }

    /// Evaluates a function using this `VirtualMachine`
    pub fn evaluate(&'a mut self, function: Function<'a>) -> Result<(), String> {
        self.control_stack.push_frame(ControlFrame::from_function(function));
        self.run()
    }

    // /// Creates a new `VirtualMachine`
    // pub fn from_function_storage(function_storage: &'a FunctionStorage) -> Self {
    //     Self {
    //         control_stack: ControlStack::new(),
    //         data_stack: DataStack::new(),
    //         function_storage,
    //     }
    // }

    /// Runs the `VirtualMachine` to perform the evaluation process
    fn run(&'a mut self) -> Result<(), String> {
        loop {
            let Some(frame) = self.control_stack.top() else { return Ok(()) };
            match frame.run_step(&mut self.data_stack, &self.function_storage) {
                ControlAction::Continue => continue,
                ControlAction::Error(error) => return Err(error),
                ControlAction::Halt => return Ok(()),
                ControlAction::Pop => self.control_stack.pop_frame(),
                ControlAction::Push(new_frame) =>
                    self.control_stack.push_frame(ControlFrame::from_function(new_frame)),
            }
        }
    }

    /// Returns the data stack items as a `&[Data]`, with the top item at the end
    pub fn items(&self) -> impl IntoIterator<Item = Data> {
        self.data_stack.items()
    }

}



/// Describes how the `VirtualMachine` should manipulate its `ControlStack` after an
/// evaluation step
pub enum ControlAction<'a> {
    /// Does nothing, continues evaluation
    Continue,
    /// Halts evaluation, and returns an error
    Error (String),
    /// Halts evaluation
    Halt,
    /// Pops a `ControlFrame` off the `ControlStack` before continuing evaluation
    Pop,
    /// Pushes a new `ControlFrame` to the `ControlStack` before continuing evaluation
    Push (Function<'a>),
}



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
        data_stack: &mut DataStack,
        environment: &'a Environment,
    ) -> ControlAction<'a> {
        let Some (term) = self.function.body().get(unsafe { *self.index.get() })
        else { return ControlAction::Pop };
        match term {
            Term::Application (reference) => {
                let function: Function = reference.fetch(environment);
                ControlAction::Push (function)
            },
            Term::Combinator (combinator) => match data_stack.evaluate_combinator(
                environment,
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
                ControlAction::Push (self.function.clone()),
        }
    }

}



/// The stack that stores the `ControlFrame`s used to represent function calls
pub struct ControlStack<'a> (UnsafeCell<SmallVec<[ControlFrame<'a>; 1024]>>);

impl<'a> ControlStack<'a> {

    /// Create a new `ControlStack`
    pub fn new() -> Self { Self (UnsafeCell::new(SmallVec::new())) }

    /// Removes the `ControlFrame` from the top of this `ControlStack`
    pub fn pop_frame(&self) {
        unsafe { (*self.0.get()).pop(); }
    }

    /// Adds a new `ControlFrame` to this `ControlStack`
    pub fn push_frame(&'_ self, frame: ControlFrame<'a>) {
        unsafe { (*self.0.get()).push(frame); }
    }

    /// Returns a reference to the `ControlFrame` at the top of this `ControlStack`
    pub fn top(&'a self) -> Option<&ControlFrame<'a>> {
        unsafe { (*self.0.get()).last() }
    }
}