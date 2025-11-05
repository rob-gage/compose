// Copyright Rob Gage 2025

use crate::{
    FunctionReference,
    Integer,
    Namespace,
};
use std::fmt::{
    Result as FormatResult,
    Write
};

/// Data that can be stored on the `Stack`
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Data {

    /// A true or false value
    Boolean (bool),

    /// An unbounded signed integer
    Integer (Integer),

    /// An anonymous function
    Lambda (FunctionReference<Vec<usize>>),

    /// A list of `Data`
    List (Vec<Data>),

}

impl Data {

    /// Formats the data using a `Namespace`
    pub fn write<W: Write>(&self, w: &mut W, namespace: &Namespace) -> FormatResult {
        match self {

            Data::Boolean (boolean) => w.write_str(if *boolean { "true" } else { "false" }),

            Data::Integer (integer) => w.write_str(&integer.to_string()),

            Data::Lambda (function_indices) => {
                w.write_str("( ")?;
                for function_index in function_indices {
                    for term in namespace.function_storage().get(*function_index).body() {
                        namespace.write_term(w, term)?;
                        w.write_char(' ')?;
                    }
                }
                w.write_char(')')
            },

            Data::List (items) => {
                w.write_str("[ ")?;
                for item in items {
                    item.write(w, namespace)?;
                    w.write_char(' ')?;
                }
                w.write_char(']')
            }

        }
    }

}
