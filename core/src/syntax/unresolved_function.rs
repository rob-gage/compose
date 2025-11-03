// Copyright Rob Gage 2025

use super::{
    SyntaxError,
    UnresolvedTerm
};
use pups::*;
use crate::virtual_machine::terms::Term;
use std::fmt::Debug;

/// An unresolved function that is stored in a `Namespace` until resolution
#[derive(Clone)]
pub struct UnresolvedFunction {
    /// The `UnresolvedTerm`s composing the function body
    body: Vec<UnresolvedTerm>,
    /// The name of the function
    name: String,
}

impl UnresolvedFunction {

    /// Returns the `UnresolvedTerm`s making up the body of this `UnresolvedFunction`
    pub fn body(&self) -> &[UnresolvedTerm] { &self.body }

    /// Returns the name of this `UnresolvedFunction`
    pub fn name(&self) -> &str { &self.name }

    /// Parses an `UnresolvedFunction` from text
    pub fn parse(input: &Text) -> ParseResult<Self> {
        terminated(unicode_identifier(), whitespace().or_not())
            .then(delimited(
                token(":").then(whitespace().or_not()),
                UnresolvedTerm::parse_many,
                whitespace().or_not().then(token(";"))
            ))
            .map(|(name, body)| Self {
                body: body.into_iter().map(|term| match term {
                    UnresolvedTerm::UnresolvedApplication (function_name) if function_name == name
                    => UnresolvedTerm::Resolved (Term::Recursion),
                    other => other
                }).collect(),
                name: name.to_string()
            })
            .parse(input)
    }

    /// Parses an `UnresolvedFunction` from text containing free terms
    pub fn parse_free_terms(input: &Text) -> ParseResult<Self> {
        delimited(
            whitespace().or_not(),
            UnresolvedTerm::parse_many,
            whitespace().or_not(),
        )
            .map(|body| Self { body, name: String::new() })
            .parse(input)
    }

}

impl Debug for UnresolvedFunction {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.name.is_empty() { f.write_str("ꟛ") } else { write!(f, "`{}`", self.name) }
    }

}