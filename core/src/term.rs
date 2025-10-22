// Copyright Rob Gage 2025

use crate::{
    Combinator,
    Data,
};

/// A concatenative programming term that can represent data or operations
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Term {

    /// Application of a named function
    ///
    /// The `usize` represents the index in the function storage of the function, or it represents
    /// recursion of the named function the application is in if it is `usize::MAX`
    Application (usize),

    /// A combinator that performs an operation on the stack
    Combinator (Combinator),

    /// Data as a term that will be pushed to the `Stack`
    Data (Data),

}