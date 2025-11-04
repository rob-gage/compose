// Copyright Rob Gage 2025

mod control_action;
mod control_frame;
mod control_stack;
pub mod data_stack;
mod function;
pub mod function_storage;
pub mod terms;

use control_action::ControlAction;
use control_frame::ControlFrame;
use control_stack::ControlStack;
use data_stack::DataStack;
use function::Function;
use function_storage::FunctionStorage;
use terms::Term;

/// A virtual machine used for evaluation of Compose programs and functions
pub struct VirtualMachine<'a> {
    control_stack: ControlStack<'a>,
    data_stack: DataStack,
    function_storage: &'a FunctionStorage,
}

impl<'a> VirtualMachine<'a> {

    /// Runs the `VirtualMachine` to perform the evaluation process
    fn run(&'a mut self) -> Result<(), String> {
        loop {
            let action: ControlAction = {
                let Some (frame) = self.control_stack.top() else { return Ok (()) };
                frame.run_step(&mut self.data_stack, self.function_storage)
            };
            loop {
                match action {
                    ControlAction::Continue => continue,
                    ControlAction::Error (error) => return Err (error),
                    ControlAction::Halt  => return Ok (()),
                    ControlAction::Pop => {
                        self.control_stack.pop_frame();
                        break
                    },
                    ControlAction::Push (new_frame) => self.control_stack.push_frame(
                        ControlFrame::from_function(new_frame.clone())
                    ),
                }
            };
        }
    }

}