// Copyright Rob Gage 2025

use crate::{
    LambdaReference,
    Integer,
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