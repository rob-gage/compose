// Copyright Rob Gage 2025

use nom::{
    bytes::complete::{
        take_while,
        take_while1,
    },
    combinator::map,
    IResult,
    Parser,
};

/// Parses an identifier made of numbers, letters, and underscores
pub fn parse_identifier(input: &str) -> IResult<&str, String> {
    let first_predicate = |c: char| c.is_ascii_alphabetic() || c == '_';
    let rest_predicate = |c: char| c.is_ascii_alphanumeric() || c == '_';
    map(
        (
            take_while1(first_predicate),
            take_while(rest_predicate),
        ),
        |(first, rest): (&str, &str)| format!("{}{}", first, rest),
    ).parse(input)
}