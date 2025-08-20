// Copyright Rob Gage 2025

extern crate core;

mod parser;
mod virtual_machine;

pub use parser::UnresolvedFunction;
pub use virtual_machine::{
    Function,
    Namespace,
    Stack,
};