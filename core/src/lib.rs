// Copyright Rob Gage 2025

mod functions;
mod integer;
mod namespace;
mod syntax;
mod virtual_machine;

use functions::{
    Environment,
    Function,
    LambdaReference,
};
use virtual_machine::combinator::Combinator;
use integer::Integer;
use syntax::UnresolvedTerm;
use virtual_machine::terms::Term;

pub use virtual_machine::{
    Data,
    VirtualMachine,
};

pub use functions::FunctionReference;
pub use namespace::Namespace;
pub use syntax::UnresolvedFunction;