// Copyright Rob Gage 2025

use super::{
    ControlFrame,
    Function
};

/// Describes how the `VirtualMachine` should manipulate its `ControlStack` after an
/// evaluation step
pub enum ControlAction<'e> {
    /// Does nothing, continues evaluation
    Continue,
    /// Halts evaluation, and returns an error
    Error (String),
    /// Halts evaluation
    Halt,
    /// Pops a `ControlFrame` off the `ControlStack` before continuing evaluation
    Pop,
    /// Pushes a new `ControlFrame` to the `ControlStack` before continuing evaluation
    Push (Function<'e>),
}