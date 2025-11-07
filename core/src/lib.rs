// Copyright Rob Gage 2025

mod environment;
mod function;
mod integer;
mod namespace;
mod syntax;
mod virtual_machine;

use virtual_machine::combinator::Combinator;
use integer::Integer;
use syntax::UnresolvedTerm;
use virtual_machine::terms::Term;

use environment::Environment;
pub use virtual_machine::{
    Data,
    VirtualMachine,
};

pub use function::Function;
pub use namespace::Namespace;
pub use virtual_machine::data_stack::DataStack;
pub use syntax::UnresolvedFunction;