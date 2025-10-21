// Copyright Rob Gage 2025

use crate::{
    Combinator,
    Data,
    Integer,
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
        choice([
            lambda,
            boolean,
            combinator,
            integer,
            application,
        ])
            .parse(input)
    }

    pub fn parse_many(input: &Text) -> ParseResult<Vec<Self>> {
        separated(
            Self::parse,
            whitespace()
        )
            .parse(input)
    }

}


/// Parses a function application term
fn application(input: &Text) -> ParseResult<UnresolvedTerm> {
    unicode_identifier()
        .map(|identifier: &str| UnresolvedTerm::UnresolvedApplication (identifier.to_string()))
        .parse(input)
}


/// Parses a boolean term
fn boolean(input: &Text) -> ParseResult<UnresolvedTerm> {
    choice([
        token("true").emit(UnresolvedTerm::Resolved (Term::Data (Data::Boolean (true)))),
        token("false").emit(UnresolvedTerm::Resolved (Term::Data (Data::Boolean (false)))),
    ])
        .parse(input)
}


/// Parses a combinator term
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


/// Parses a lambda term
fn lambda(input: &Text) -> ParseResult<UnresolvedTerm> {
    delimited(
        token("(").then(whitespace().or_not()),
        UnresolvedTerm::parse_many,
        whitespace().or_not().then(token(")"))
    )
        .map(|terms| UnresolvedTerm::UnresolvedLambda (terms))
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
