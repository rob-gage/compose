// Copyright Rob Gage 2025

mod control_action;
mod control_stack;
mod control_frame;
pub mod data;
pub mod data_stack;
mod function;
pub mod function_storage;
pub mod terms;
pub mod combinator;


use control_action::ControlAction;
use control_frame::ControlFrame;
use control_stack::ControlStack;

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
    environment: Environment,
}

impl<'a> VirtualMachine<'a> {

    /// Evaluates a function using this `VirtualMachine`
    pub fn evaluate(&'a mut self, function: Function<'a>) -> Result<(), String> {
        self.control_stack.push_frame(ControlFrame::from_function(function));
        self.run()
    }

    /// Runs the `VirtualMachine` to perform the evaluation process
    fn run(&mut self) -> Result<(), String> {
        loop {
            loop {
                let frame: &ControlFrame<'a> = self.control_stack.top().unwrap();
                match frame.run_step(&mut self.data_stack, &self.environment) {
                    ControlAction::Continue => continue,
                    ControlAction::Error(error) => return Err(error),
                    ControlAction::Halt => return Ok(()),
                    ControlAction::Pop => {
                        self.control_stack.pop_frame();
                        break;
                    }
                    ControlAction::Push(new_frame) => {
                        self.control_stack.push_frame(ControlFrame::from_function(new_frame));
                        break;
                    }
                }
            }
        }
    }

    /// Returns the data on this `VirtualMachine`s stack as an iterator, starting at the bottom
    /// of the stack
    pub fn data(&self) -> impl IntoIterator<Item = Data> {
        self.data_stack.items()
    }

    /// Adds data to the stack of this `VirtualMachine`
    pub fn with_data(mut self, data: &[Data]) -> Self {
        for item in data {
            self.data_stack.push(item.clone());
        }
        self
    }

}