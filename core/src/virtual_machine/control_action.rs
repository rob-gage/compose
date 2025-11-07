// Copyright Rob Gage 2025

use super::{
    ControlStack,
    Function
};

/// Describes how the `VirtualMachine` should manipulate its `ControlStack` after an
/// evaluation step
pub enum ControlAction<'vm> {
    /// Does nothing, continues evaluation
    Continue,
    /// Halts evaluation, and returns an error
    Error (String),
    /// Pops a `ControlFrame` off the `ControlStack` before continuing evaluation
    Pop,
    /// Pushes a new `ControlFrame` to the `ControlStack` before continuing evaluation
    Push (Function<'vm>),
}