// Copyright Rob Gage 2025

mod combinator;
mod data;
mod integer;
mod namespace;
mod stack;
mod syntax;
mod terms;

use combinator::Combinator;
use data::Data;
use integer::Integer;
use syntax::{
    SyntaxError,
    UnresolvedFunction,
    UnresolvedTerm,
};
use terms::{
    Term,
    TermBuffer,
    TermSequence,
    TermSequenceReference,
};

pub use namespace::Namespace;
pub use stack::Stack;