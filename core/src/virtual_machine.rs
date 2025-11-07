// Copyright Rob Gage 2025

pub mod data;
pub mod data_stack;
mod old_function;
pub mod function_storage;
pub mod combinator;
mod control;

use control::{
    ControlAction,
    ControlFrame,
    ControlStack,
};
use data_stack::DataStack;
use crate::{
    Environment,
    Function,
};
use std::sync::{
    Arc,
    RwLock,
    RwLockReadGuard,
};

pub use data::Data;

/// A virtual machine used for evaluation of Compose programs and functions
pub struct VirtualMachine {
    data_stack: DataStack,
    environment: Arc<RwLock<Environment>>,
}

impl VirtualMachine {

    pub fn evaluate<'r>(&mut self, function_reference: FunctionReference) -> Result<(), String> {
        // lock environment so nothing can write to it until evaluation is finished
        let guard: RwLockReadGuard<Environment> = self.environment.read().unwrap();
        let environment: &Environment = &*guard;
        // push first function to the control stack as a `ControlFrame`
        let function: Function = function_reference.fetch(environment);
        self.control_stack.push_frame(ControlFrame::from_function(function));
        // repeatedly pop frame from stack and do as much evaluation as possible
        while let Some (mut frame) = self.control_stack.pop_frame() {
            loop {
                let action = frame.run_step(&mut self.data_stack, environment);
                // match frame.run_step(&mut self.data_stack, environment) {
                //     ControlAction::Continue => continue,
                //     ControlAction::Error(error) => return Err(*error),
                //     ControlAction::Pop => break,
                //     ControlAction::Push(function) => {
                //         self.control_stack.push_frame(frame);
                //         self.control_stack.push_frame(ControlFrame::from_function(function));
                //     },
                // }
            }
        }

        Ok(())
    }

    fn environment<'guard: 'a>(&self, guard: &'guard RwLockReadGuard<Environment>) -> &'a Environment {
        &*guard
    }

    /// Creates a new `VirtualMachine` from a `&Arc<RwLock<Environment>>`
    pub fn from_environment(environment: &Arc<RwLock<Environment<'a>>>) -> Self {
        Self {
            control_stack: ControlStack::new(),
            data_stack: DataStack::new(),
            environment: environment.clone(),
        }
    }

    /// Runs the `VirtualMachine` to perform the evaluation process
    fn run<'b>(&mut self, guard: RwLockReadGuard<'a, Environment<'a>>) -> Result<(), String> {
        // let environment: &Environment = &*guard;
        // loop {
        //     let Some(frame) = self.control_stack.top() else { return Ok(()) };
        //     loop {
        //
        //         let stack: *mut DataStack = &mut self.data_stack as *mut _;
        //         match frame.run_step(unsafe { &mut *stack }, &environment) {
        //             ControlAction::Continue => continue,
        //             ControlAction::Error(error) => return Err(error),
        //             ControlAction::Halt => return Ok(()),
        //             ControlAction::Pop => {
        //                 self.control_stack.pop_frame();
        //                 break;
        //             }
        //             ControlAction::Push(new_frame) => {
        //                 self.control_stack.push_frame(ControlFrame::from_function(new_frame));
        //                 break;
        //             }
        //         }
        //     }
        // }
        todo!()
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