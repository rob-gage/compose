// Copyright Rob Gage 2025

use std::fmt::{
    Display,
    Formatter
};
use super::{
    Combinator,
    Data,
    Integer
};

/// A concatenative programming term that can represent data or operations
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Term {

    /// Application of a named function
    Application (usize),

    /// A combinator that performs an operation on the stack
    Combinator (Combinator),

    /// Data as a term that will be pushed to the `Stack`
    Data (Data),

}

impl Term {

    /// Creates a new `Data (Boolean)` term from a `bool`
    pub fn new_boolean(string: &str) -> Self {
        match string {
            "true" => Self::Data (Data::Boolean (true)),
            "false" => Self::Data (Data::Boolean (false)),
            _ => unreachable!("`new_boolean` should only be called with strings parsable to \
            booleans"),
        }
    }

    /// Creates a new `Data (Integer)` term from a `&str`
    ///
    /// (will fail if the `&str` is not a parsable `Integer`)
    pub fn new_integer(string: &str) -> Self {
        Self::Data (Data::Integer (Integer::from_string(string)
            .expect("String provided to `new_integer` function is a parsable `Integer`")))
    }

}

impl Display for Term {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}