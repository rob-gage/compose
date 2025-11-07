// Copyright Rob Gage 2025

pub mod data;
pub mod combinator;
mod control;

use control::{
    ControlAction,
    ControlFrame,
    ControlStack,
};
use data::DataStack;
use crate::{
    Environment,
    Function,
    FunctionReference,
};
use std::sync::{
    Arc,
    RwLock,
    RwLockReadGuard,
};

pub use combinator::Combinator;
pub use data::Value;

/// A virtual machine used for evaluation of Compose programs and functions
pub struct VirtualMachine {
    data_stack: DataStack,
    environment: Arc<RwLock<Environment>>,
}

impl VirtualMachine {

    pub fn evaluate(&mut self, function_reference: FunctionReference) -> Result<(), String> {
        // lock environment so nothing can write to it until evaluation is finished
        let guard: RwLockReadGuard<Environment> = self.environment.read().unwrap();
        let environment: &Environment = &*guard;
        // create control stack, then push first function to the control stack as a `ControlFrame`
        let mut control_stack: ControlStack = ControlStack::new();
        let function: Function = function_reference.get(environment);
        control_stack.push_frame(ControlFrame::from_function(function));
        // repeatedly pop frame from stack and do as much evaluation as possible
        while let Some (mut frame) = control_stack.pop_frame() {
            loop {
                let action = frame.execute_step(&mut self.data_stack, environment);
                match action {
                    ControlAction::Continue => continue,
                    ControlAction::Error(error) => return Err(error),
                    ControlAction::Pop => break,
                    ControlAction::Push(function) => {
                        control_stack.push_frame(frame);
                        control_stack.push_frame(ControlFrame::from_function(function));
                        break;
                    },
                }
            }
        }
        Ok (())
    }

    /// Creates a new `VirtualMachine` from a `&Arc<RwLock<Environment>>`
    pub fn from_environment(environment: &Arc<RwLock<Environment>>) -> Self {
        Self {
            data_stack: DataStack::new(),
            environment: environment.clone(),
        }
    }

    /// Returns the data on this `VirtualMachine`s stack as an iterator, starting at the bottom
    /// of the stack
    pub fn data(&self) -> impl IntoIterator<Item =Value> {
        self.data_stack.items()
    }

    /// Adds data to the stack of this `VirtualMachine`
    pub fn with_data(mut self, data: &[Value]) -> Self {
        for item in data {
            self.data_stack.push(item.clone());
        }
        self
    }

}