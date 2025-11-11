// Copyright Rob Gage 2025

mod functions;
mod integer;
mod namespace;
mod syntax;
mod term;
mod virtual_machine;

use functions::{
    Environment,
    Function,
    LambdaReference,
};
use virtual_machine::Combinator;
use integer::Integer;
use syntax::UnresolvedTerm;
use term::Term;

pub use virtual_machine::{
    Value,
    VirtualMachine,
};

pub use functions::FunctionReference;
pub use namespace::Namespace;
pub use syntax::UnresolvedFunction;