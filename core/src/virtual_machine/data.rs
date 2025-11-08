// Copyright Rob Gage 2025

use crate::{
    LambdaReference,
    Integer,
    Namespace,
};
use smallvec::{
    SmallVec,
    smallvec,
};
use std::{
    cell::UnsafeCell,
    fmt::{
        Formatter,
        Result as FormatResult,
        Write,
    }
};

/// Data that can be stored on the `Stack`
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {

    /// A true or false value
    Boolean (bool),

    /// An unbounded signed integer
    Integer (Integer),

    /// An anonymous function
    Lambda (LambdaReference),

    /// A list of `Data`
    List (Vec<Value>),

}



/// How many terms on the stack are stored on the actual stack
const STACK_STACK_SIZE: usize = 1024;

/// A last-in-first-out stack that can store `Data` and is used to evaluate programs
pub struct DataStack {
    /// The buffer containing the data on the stack
    buffer: UnsafeCell<SmallVec<[Value; STACK_STACK_SIZE]>>,
    /// The size of the stack
    top: usize
}

impl DataStack {

    /// Returns the items in this stack as a `&[Data]`
    pub fn items(&self) -> SmallVec<[Value; STACK_STACK_SIZE]> {
        unsafe { (*self.buffer.get()).clone() }
    }

    /// Creates a new `Stack` with no data in it
    pub fn new() -> Self {
        Self {
            buffer: UnsafeCell::new(smallvec![]),
            top: 0,
        }
    }

    /// Gets a reference to the `Stack` item with a specified index from the top of the stack
    /// (`0` is the index for the top)
    pub fn get_from_top(&self, index: usize) -> Option<&Value> {
        let stack_index: usize = self.top.checked_sub(1 + index)?;
        unsafe {
            let buffer: &SmallVec<[Value; STACK_STACK_SIZE]> = &*self.buffer.get();
            buffer.get(stack_index)
        }
    }
    
    /// Gets a reference to the `Stack` item with a specified index from the top of the stack
    /// (`0` is the index for the top)
    pub fn get_mutable_from_top(&self, index: usize) -> Option<&mut Value> {
        let stack_index: usize = self.top.checked_sub(1 + index)?;
        unsafe {
            let buffer: &mut SmallVec<[Value; STACK_STACK_SIZE]> = &mut *self.buffer.get();
            buffer.get_mut(stack_index)
        }
    }
    
    /// Pops a `Value` off the array
    pub fn pop(&mut self) -> Option<Value> {
        if self.top == 0 {
            return None;
        }
        self.top -= 1;
        unsafe {
            Some((*self.buffer.get()).pop().unwrap())
        }
    }
    
    /// Pushes a `Value` onto the top of the stack
    pub fn push(&mut self, data: Value) {
        unsafe {
            (*self.buffer.get()).push(data);
        }
        self.top += 1;
    }
    
    /// Returns the size of the `Stack`
    pub const fn size(&self) -> usize {
        self.top
    }

}

impl Clone for DataStack {
    fn clone(&self) -> Self {
        let cloned: SmallVec<[Value; STACK_STACK_SIZE]> = unsafe { (&*self.buffer.get()).clone() };
        Self {
            buffer: UnsafeCell::new(cloned),
            top: self.top,
        }
    }

}