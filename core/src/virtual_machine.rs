// Copyright Rob Gage 2025

use crate::{
    Function,
    FunctionStorage,
    Stack,
    TermSequence,
};
use std::sync::Arc;

/// A virtual machine that can handle evaluation of functions
#[derive(Clone)]
pub struct VirtualMachine<'a> {
    /// The `FunctionStorage` used to store functions used by this `VirtualMachine`
    function_storage: Arc<FunctionStorage<'a>>,
    /// The `Stack` used by this `VirtualMachine` for evaluation
    stack: Stack
}

impl<'a> VirtualMachine<'a> {

    /// Evaluates a `Function` using this `VirtualMachine`
    pub fn evaluate<TS>(&'a mut self, function: Function<'a, TS>) -> Result<(), String>
    where
        TS: TermSequence<'a>
    {
        function.evaluate(&self.function_storage, &mut self.stack)
    }

    /// Creates a new `VirtualMachine` from an `FunctionStorage`
    pub fn from_function_storage(function_storage: FunctionStorage<'a>) -> Self {
        Self {
            function_storage: Arc::new(function_storage),
            stack: Stack::new(),
        }
    }

    /// Clears the stack on a `VirtualMachine`
    pub fn reset(&mut self) { self.stack = Stack::new(); }

}