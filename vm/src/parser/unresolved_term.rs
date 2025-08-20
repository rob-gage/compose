// Copyright Rob Gage 2025

use crate::virtual_machine::{
    Combinator,
    Term
};
use nom::{
    branch::alt,
    bytes::complete::{
        tag,
        tag_no_case
    },
    character::complete::{
        char,
        digit1,
        multispace0,
    },
    combinator::{
        map,
        opt,
        recognize,
        value,
    },
    multi::many0,
    IResult,
    Parser,
    sequence::{
        delimited,
        pair,
    },
};
use super::parse_identifier;

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

    /// Attempts to parse an `UnresolvedTerm` from parser
    pub fn parse(input: &str) -> IResult<&str, Self> {
        parse_unresolved_term(input)
    }

}

/// Parse an `UnresolvedTerm`
fn parse_unresolved_term(input: &str) -> IResult<&str, UnresolvedTerm> {
    alt((
        parse_lambda,
        parse_boolean,
        parse_combinator,
        parse_integer,
        map(parse_identifier, UnresolvedTerm::UnresolvedApplication),
    )).parse(input)
}

/// Parses a boolean `UnresolvedTerm`
fn parse_boolean(input: &str) -> IResult<&str, UnresolvedTerm> {
    map(
        alt((
            tag_no_case("true"),
            tag_no_case("false"),
        )),
        |string| UnresolvedTerm::Resolved(Term::new_boolean(string)),
    ).parse(input)
}

/// Parses a combinator `UnresolvedTerm`
fn parse_combinator(input: &str) -> IResult<&str, UnresolvedTerm> {
    use Combinator::*;
    map(
        alt((
            alt((
                // arithmetic combinators
                value(Add, tag("+")),
                value(Divide, tag("/")),
                value(Modulo, tag("%")),
                value(Multiply, tag("*")),
                value(Subtract, tag("-")),
                // boolean logic combinators
                value(And, tag("&")),
                value(ExclusiveOr, tag("^")),
                value(Not, tag("!")),
                value(Or, tag("|")),
                // comparison combinators
                value(Equality, tag("=")),
                value(GreaterThan, tag(">")),
                value(LessThan, tag("<")),
            )),
            alt((
                // functional combinators
                value(Apply, tag("apply")),
                value(Compose, tag("compose")),
                value(If, tag("if")),
                value(Under, tag("under")),
                // list processing combinators
                value(Count, tag("count")),
                value(Filter, tag("filter")),
                value(Fold, tag("fold")),
                value(Head, tag("head")),
                value(Join, tag("join")),
                value(Map, tag("map")),
                value(Tail, tag("tail")),
                // stack manipulation combinators
                value(Copy, tag("copy")),
                value(Drop, tag("drop")),
                value(Hop, tag("hop")),
                value(Rotate, tag("rotate")),
                value(Swap, tag("swap")),
            )),
        )),
        |combinator| UnresolvedTerm::Resolved (Term::Combinator (combinator))
    ).parse(input)
}

/// Parses an integer `UnresolvedTerm`
fn parse_integer(input: &str) -> IResult<&str, UnresolvedTerm> {
    map(
        recognize(
            pair(
                opt(char('-')),
                digit1,
            ),
        ),
        |string| UnresolvedTerm::Resolved (Term::new_integer(string))
    ).parse(input)
}

/// Parse a lambda from a sequence of terms inside parentheses
fn parse_lambda(input: &str) -> IResult<&str, UnresolvedTerm> {
    map(
        delimited(
            delimited(multispace0, char('('), multispace0),
            many0(delimited(multispace0, UnresolvedTerm::parse, multispace0)),
            delimited(multispace0, char(')'), multispace0),
        ),
        UnresolvedTerm::UnresolvedLambda,
    ).parse(input)
}