// Copyright Rob Gage 2025

use crate::{
    Combinator,
    Term
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UnresolvedTerm {

    /// A term that has already been resolved
    Resolved (Term),

    /// Application of an unresolved named function
    UnresolvedApplication (String),

    /// An unresolved lambda term
    UnresolvedLambda (Vec<UnresolvedTerm>),

    /// Unresolved recursive application
    UnresolvedRecursion,

}

// /// Parse an `UnresolvedTerm`
// fn parse_unresolved_term(input: &str) -> IResult<&str, UnresolvedTerm> {
//     alt((
//         parse_lambda,
//         parse_boolean,
//         parse_combinator,
//         parse_integer,
//         map(parse_identifier, UnresolvedTerm::UnresolvedApplication),
//     )).parse(input)
// }

