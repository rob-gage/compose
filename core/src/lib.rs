// Copyright Rob Gage 2025

mod combinator;
mod integer;
mod namespace;
mod syntax;
mod virtual_machine;

use combinator::Combinator;
use integer::Integer;
use syntax::UnresolvedTerm;
use virtual_machine::terms::Term;

pub use virtual_machine::{
    Data,
    function_storage::{
        FunctionStorage,
        FunctionReference
    },
    VirtualMachine,
};
pub use namespace::Namespace;
pub use virtual_machine::data_stack::DataStack;
pub use syntax::UnresolvedFunction;