// Copyright Rob Gage 2025

use crate::{
    LambdaReference,
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
    Lambda (LambdaReference),

    /// A list of `Data`
    List (Vec<Data>),

}

impl Data {

    /// Formats the data using a `Namespace`
    pub fn write<'e, W: Write>(&self, w: &mut W, namespace: &'e Namespace<'e>) -> FormatResult {
        match self {

            Data::Boolean (boolean) => w.write_str(if *boolean { "true" } else { "false" }),

            Data::Integer (integer) => w.write_str(&integer.to_string()),

            Data::Lambda (reference) => {
                w.write_str("( ")?;
                for term in reference.fetch(&*namespace.environment().read().unwrap()).body() {
                    namespace.write_term(w, term)?;
                    w.write_char(' ')?;
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
