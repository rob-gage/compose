// Copyright Rob Gage 2025

mod combinator;
mod data;
mod integer;
mod namespace;
mod stack;
mod syntax;
mod term;
mod functions;

use combinator::Combinator;
use data::Data;
use functions::{
    Function,
    FunctionStorage,
};
use integer::Integer;
use syntax::{
    SyntaxError,
    UnresolvedFunction,
    UnresolvedTerm,
};
use term::Term;

pub use namespace::Namespace;
pub use stack::Stack;