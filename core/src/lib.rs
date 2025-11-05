// Copyright Rob Gage 2025

mod combinator;

mod environment;
mod integer;
mod namespace;
mod syntax;
mod virtual_machine;
use combinator::Combinator;
use integer::Integer;
use syntax::UnresolvedTerm;
use virtual_machine::terms::Term;

pub use environment::{
    Environment,
    FunctionReference,
    LambdaReference,
};
pub use virtual_machine::{
    Data,
    Function,
    VirtualMachine,
};
pub use namespace::Namespace;
pub use virtual_machine::data_stack::DataStack;
pub use syntax::UnresolvedFunction;