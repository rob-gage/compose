// Copyright Rob Gage 2025

use std::fmt::{
    self,
    Display,
    Formatter
};
use super::Integer;

/// Data that can be stored on the `Stack`
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Data {

    /// A true or false value
    Boolean (bool),

    /// An unbounded signed integer
    Integer (Integer),

    /// An anonymous function
    Lambda (Vec<usize>),

    /// A list of `Data`
    List (Vec<Data>),

    /// A text string
    String (String),

}

impl Display for Data {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {

            Data::Boolean (boolean) => write!(
                f, "{}",
                if *boolean { "true" } else { "false" }
            ),

            Data::Integer (integer) => write!(f, "{}", integer.to_string()),

            Data::Lambda (_) => write!(f, "λ"),

            Data::List (items) => write!(f, "[ {} ]", items.iter()
                .map(|term| format!("{}", term))
                .collect::<Vec<_>>()
                .join(" ")
            ),

            Data::String (string) => write!(f, "\"{}\"", string),

        }
    }
}