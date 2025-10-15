// Copyright Rob Gage 2025

use crate::{
    Combinator,
    Data,
    Integer,
    Term
};
use pups::*;
use crate::data::Data::Lambda;

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

// fn combinator(input: &Text) -> ParseResult<UnresolvedTerm> {
//     use Combinator::*;
//     fn combinator_parser<'a>(
//         combinator: Combinator
//     ) -> impl Parser<'a, Text<'a>, Output = Combinator, Error = (), Message = ()> {
//         token(combinator.name()).map(move |_| combinator)
//     }
//     // arithmetic
//     combinator_parser(Add)
//         .or(combinator_parser(Divide))
//         .or(combinator_parser(Modulo))
//         .or(combinator_parser(Multiply))
//         .or(combinator_parser(Subtract))
//     // boolean logic
//         .or(combinator_parser(And))
//         .or(combinator_parser(ExclusiveOr))
//         .or(combinator_parser(Not))
//         .or(combinator_parser(Or))
//     // comparison
//         .or(combinator_parser(Equality))
//         .or(combinator_parser(GreaterThan))
//         .or(combinator_parser(LessThan))
//     // functional
//         .or(combinator_parser(Apply))
//         .or(combinator_parser(If))
//         .or(combinator_parser(Compose))
//         .or(combinator_parser(Under))
//     // stack manipulation
//         .or(combinator_parser(Copy))
//         .or(combinator_parser(Drop))
//         .or(combinator_parser(Hop))
//         .or(combinator_parser(Rotate))
//         .or(combinator_parser(Swap))
//     // ---------------------------------------------------
//         .map(|combinator| UnresolvedTerm::Resolved (Term::Combinator (combinator)))
//         .map_error(|_| ())
//         .parse(input)
// }

/// Parses an integer term
fn integer(input: &Text) -> ParseResult<UnresolvedTerm> {
    number()
        .map(|number| UnresolvedTerm::Resolved (Term::Data (Data::Integer (
            Integer::from_string(number).expect("Parser will never parse an invalid integer")
        ))))
        .parse(input)
}
