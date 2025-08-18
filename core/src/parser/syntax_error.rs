// Copyright Rob Gage 2025

use std::{
    fmt::{
        Display,
        Formatter,
        self,
    },
    ops::Range
};

/// An error with syntax being parsed
pub struct SyntaxError {
    /// The ranges of invalid syntax within the source
    range: Range<usize>,
    /// The source index in the parser this `SyntaxError` is in
    source_index: usize,
    /// The `SyntaxErrorVariant`
    variant: SyntaxErrorVariant,
}
impl SyntaxError {

    /// Creates a new `SyntaxError`
    pub fn new(source_index: usize, variant: SyntaxErrorVariant, range: Range<usize>) -> Self {
        Self {
            range,
            source_index,
            variant
        }
    }

}

/// Variant type for `SyntaxError`
pub enum SyntaxErrorVariant {

    /// Expected a colon after the identifier in a definition
    ExpectedColon,

    /// Expected a top level definition
    ExpectedDefinition,

    /// Expected a semicolon at the end of a definition
    ExpectedSemicolon,

    /// Encountered an invalid term in a function body
    ExpectedTerm,

}