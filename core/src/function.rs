// Copyright Rob Gage 2025

use crate::{
    Environment,
    Term
};

/// A `Function` that can be evaluated on a `VirtualMachine`
pub enum Function {
    Contiguous {
        pointer: *const Term,
        size: usize,
    },
    Composed (Vec<usize>)
}

impl Function {

    /// Returns a slice containing the `Term`s in this `Function`
    pub

}