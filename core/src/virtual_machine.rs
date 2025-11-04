// Copyright Rob Gage 2025

mod control_action;
mod control_frame;
mod control_stack;
pub mod data;
pub mod data_stack;
mod function;
pub mod function_storage;
pub mod terms;

use control_action::ControlAction;
use control_frame::ControlFrame;
use control_stack::ControlStack;
use data::Data;
use data_stack::DataStack;
use function::Function;
use function_storage::FunctionStorage;
use terms::Term;

/// A virtual machine used for evaluation of Compose programs and functions
pub struct VirtualMachine<'a> {
    control_stack: ControlStack<'a>,
    data_stack: DataStack,
    function_storage: &'a FunctionStorage<'a>,
}

impl<'a> VirtualMachine<'a> {

    /// Evaluates a function using this `VirtualMachine`
    pub fn evaluate(&'a mut self, function: Function<'a>) -> Result<(), String> {
        self.control_stack.push_frame(ControlFrame::from_function(function));
        self.run()
    }

    /// Creates a new `VirtualMachine`
    pub fn from_function_storage(function_storage: &'a FunctionStorage) -> Self {
        Self {
            control_stack: ControlStack::new(),
            data_stack: DataStack::new(),
            function_storage,
        }
    }

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