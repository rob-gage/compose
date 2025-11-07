// Copyright Rob Gage 2025

use smallvec::SmallVec;
use super::ControlFrame;

/// The stack that stores the `ControlFrame`s used to represent function calls
pub struct ControlStack<'a> (SmallVec<[ControlFrame<'a>; 1024]>);

impl<'a> ControlStack<'a> {

    /// Create a new `ControlStack`
    pub fn new() -> Self { Self (SmallVec::new()) }

    /// Removes the `ControlFrame` from the top of this `ControlStack`
    pub fn pop_frame(&mut self) -> Option<ControlFrame<'a>> { self.0.pop() }

    /// Adds a new `ControlFrame` to this `ControlStack`
    pub fn push_frame(&mut self, frame: ControlFrame<'a>) { self.0.push(frame) }
}