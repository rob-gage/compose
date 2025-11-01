// Copyright Rob Gage 2025

mod control_action;
mod control_frame;
mod control_stack;
pub mod data_stack;
pub mod function_storage;

use control_action::ControlAction;
use control_frame::ControlFrame;
use control_stack::ControlStack;
use data_stack::DataStack;
use function_storage::FunctionStorage;

/// A virtual machine used for evaluation of Compose programs and functions
pub struct VirtualMachine<'a> {
    control_stack: ControlStack<'a>,
    data_stack: DataStack,
    function_storage: FunctionStorage,
}

impl<'a> VirtualMachine<'a> {

    /// Performs one step of the evaluation process
    fn step(&'a mut self) -> Result<(), String> {
        let Some (top) = self.control_stack.top() else {
            return Ok (())
        };
        todo!()
    }

}