// Copyright Rob Gage 2025

use crate::{
    Integer,
    Namespace,
};
use std::fmt::{Formatter, Result as FormatResult, Write};

/// Data that can be stored on the `Stack`
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Data {

    /// A true or false value
    Boolean (bool),

    /// An unbounded signed integer
    Integer (Integer),

    /// An anonymous function
    Lambda (Vec<usize>),

    /// A list of `Data`
    List (Vec<Data>),

}

impl Data {

    /// Formats the data using a `Namespace`
    pub fn format(&self, f: &mut Formatter, namespace: &Namespace) -> FormatResult {
        match self {

            Data::Boolean (boolean) => f.write_str(if *boolean { "true" } else { "false" }),

            Data::Integer (integer) => f.write_str(&integer.to_string()),

            Data::Lambda (function_indices) => {
                f.write_str("( ")?;
                for function_index in function_indices {
                    for term in namespace.function_storage().get_body(*function_index) {
                        namespace.format_term(f, term)?;
                        f.write_char(' ')?;
                    }
                }
                f.write_char(')')
            },

            Data::List (items) => {
                f.write_str("[ ")?;
                for item in items {
                    item.format(f, namespace)?;
                    f.write_char(' ')?;
                }
                f.write_char(']')
            }

        }
    }

}
