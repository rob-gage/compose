// Copyright Rob Gage 2025

use crate::Term;

/// Describes how the `VirtualMachine` should manipulate its `ControlStack` after an
/// evaluation step
pub enum ControlAction<'a> {
    /// Does nothing, continues evaluation
    Continue,
    /// Pops a `ControlFrame` off the `ControlStack` before continuing evaluation
    Pop,
    /// Pushes a new `ControlFrame` to the `ControlStack` before continuing evaluation
    Push (&'a [Term]),
}

impl<'a> ControlAction<'a> {

}