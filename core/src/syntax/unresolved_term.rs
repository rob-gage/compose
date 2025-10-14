// Copyright Rob Gage 2025

use crate::{
    Combinator,
    Data,
    Term
};
use pups::*;

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

impl UnresolvedTerm {

    pub fn parse(input: &Text) -> ParseResult<Self> {
        todo!()
    }

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

/// Parses a boolean term
fn boolean(input: &Text) -> ParseResult<UnresolvedTerm> {
    token("true").map(|_| Data::Boolean (true))
        .or(token("false").map(|_| Data::Boolean (false)))
        .map(|boolean| UnresolvedTerm::Resolved (Term::Data (boolean)))
        .map_error(|_| ())
        .parse(input)

}
