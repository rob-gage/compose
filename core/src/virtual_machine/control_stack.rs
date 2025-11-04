// Copyright Rob Gage 2025

use smallvec::SmallVec;
use std::cell::UnsafeCell;
use super::{
    ControlFrame,
    DataStack,
    FunctionStorage,
    Term,
};

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