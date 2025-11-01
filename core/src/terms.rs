// Copyright Rob Gage 2025

use crate::{
    Combinator,
    Data,
    DataStack
};
use std::{
    iter::repeat,
    marker::PhantomData,
};

/// A concatenative programming term that can represent data or operations
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Term
where
    Self: Sized
{

    /// Application of a named function
    Application (usize),

    /// A combinator that performs an operation on the stack
    Combinator (Combinator),

    /// Data as a term that will be pushed to the `Stack`
    Data (Data),

    Recursion,

}