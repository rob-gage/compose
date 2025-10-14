// Copyright Rob Gage 2025

use std::{
    cell::UnsafeCell,
    sync::Arc,
};
use super::{
    FunctionStorage,
    Stack,
    Term,
};

/// A function that can be evaluated
pub struct Function {
    /// The `FunctionStorage` containing this `Function`
    function_storage: Arc<UnsafeCell<FunctionStorage>>,
    /// The index of this `Function` in its `FunctionStorage`
    index: usize,
}

impl Function {

    /// Returns the body of `Term`s composing this function
    pub fn body(&self) -> &[Term] {
        let storage: &mut FunctionStorage = unsafe { &mut *self.function_storage.get() };
        storage.get_body(self.index)
    }

    /// Evaluate the `Function`
    pub fn evaluate(&self, stack: &mut Stack) -> Result<(), String> {
        let storage: &mut FunctionStorage = unsafe { &mut *self.function_storage.get() };
        stack.evaluate_function_body(storage, self.body())
            .map_err(|s| s.to_string())
    }

    /// Create a new `Function` from a `FunctionStorage` and its index within the storage
    pub fn new(
        function_storage: &Arc<UnsafeCell<FunctionStorage>>,
        index: usize
    ) -> Self { Self {
        function_storage: function_storage.clone(),
        index,
    } }

}