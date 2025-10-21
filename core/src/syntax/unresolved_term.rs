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
    choice([
        token("true").emit(UnresolvedTerm::Resolved (Term::Data (Data::Boolean (true)))),
        token("false").emit(UnresolvedTerm::Resolved (Term::Data (Data::Boolean (false)))),
    ])
        .parse(input)
}

fn combinator(input: &Text) -> ParseResult<UnresolvedTerm> {
    use Combinator::*;
    fn combinator_parser<'a>(
        combinator: Combinator
    ) -> impl Parser<'a, UnresolvedTerm, (), (), Text<'a>> {
        token(combinator.name()).emit(UnresolvedTerm::Resolved (Term::Combinator (combinator)))
    }
    choice([
        // arithmetic
        combinator_parser(Add),
        combinator_parser(Divide),
        combinator_parser(Modulo),
        combinator_parser(Multiply),
        combinator_parser(Subtract),
        // boolean logic
        combinator_parser(And),
        combinator_parser(ExclusiveOr),
        combinator_parser(Not),
        combinator_parser(Or),
        // comparison
        combinator_parser(Equality),
        combinator_parser(GreaterThan),
        combinator_parser(LessThan),
        // functional
        combinator_parser(Apply),
        combinator_parser(Compose),
        combinator_parser(If),
        combinator_parser(Under),
        // stack manipulation
        combinator_parser(Copy),
        combinator_parser(Drop),
        combinator_parser(Hop),
        combinator_parser(Rotate),
        combinator_parser(Swap),
    ])
        .parse(input)
}


/// Parses an integer term
fn integer(input: &Text) -> ParseResult<UnresolvedTerm> {
    number()
        .map(|number| UnresolvedTerm::Resolved (Term::Data (Data::Integer (
            Integer::from_string(number).expect("Parser will never parse an invalid integer")
        ))))
        .parse(input)
}
