// Copyright Rob Gage 2025

mod combinator;
mod data;
mod integer;
mod namespace;
mod syntax;
mod terms;
mod virtual_machine;

use combinator::Combinator;
use data::Data;
use integer::Integer;
use syntax::UnresolvedTerm;
use terms::Term;

pub use virtual_machine::function_storage::{
    Function,
    FunctionStorage,
};
pub use namespace::Namespace;
pub use virtual_machine::data_stack::DataStack;
pub use syntax::UnresolvedFunction;