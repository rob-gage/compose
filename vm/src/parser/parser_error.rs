// Copyright Rob Gage 2025

/// An error that can occur while parsing syntax sources
pub struct ParserError {
    /// A list of starting and ending lines and columns for the invalid syntax
    /// making up this `ParserError`
    invalid_syntax_ranges: Vec<((usize, usize), (usize, usize))>,
}