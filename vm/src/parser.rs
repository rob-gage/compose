// Copyright Rob Gage 2025

mod parse_identifier;
mod unresolved_function;
mod unresolved_term;
mod parser_error;
mod syntax_error;


use crate::Namespace;
use parse_identifier::parse_identifier;
use std::ops::Range;

pub use syntax_error::{
    SyntaxError,
    SyntaxErrorVariant,
};
pub use unresolved_function::UnresolvedFunction;
pub use unresolved_term::UnresolvedTerm;

use codespan_reporting::{
    diagnostic::{
        Diagnostic,
        Label,
        Severity
    },
    files::SimpleFiles,
    term::{
        termcolor::{
            ColorChoice,
            StandardStream
        },
        Config,
    },
};

/// Parser that builds a `Namespace` from source files
pub struct Parser {
    /// The `Namespace` that the resolvable parsed functions are stored in
    namespace: Namespace,
    /// The syntax sources being used by this `Parser`
    sources: SimpleFiles<String, String>,
    /// The functions that have been parsed but not yet resolved
    unresolved_functions: Vec<(UnresolvedFunction, usize, Range<usize>)>,
}

impl Parser {

    /// Adds a new syntax source to this `Parser`
    pub fn add_source(
        &mut self,
        name: &str,
        syntax: &str
    ) -> Result<(), Vec<SyntaxError>> {
        let normalized: String = normalize(syntax); // normalize syntax
        let syntax: &str = &normalized;
        // add source to sources and get its index
        let source_index = self.sources.add(name.to_string(), syntax.to_string());
        let mut cursor: usize = 0; // set cursor at start of file
        // accumulate parsed functions
        let mut functions: Vec<(UnresolvedFunction, usize, Range<usize>)> = Vec::new();
        let mut syntax_errors: Vec<SyntaxError> = Vec::new(); // accumulate syntax errors
        // parse declarations until syntax is all parsed or an error is encountered
        while cursor < syntax.len() {
            let remaining: &str = &normalized[cursor..normalized.len()];
            if remaining.starts_with(char::is_whitespace) {
                cursor += preceding_whitespace(remaining)
            } else { match UnresolvedFunction::parse(
                source_index,
                syntax,
                cursor,
            ) {
                Ok ((unresolved_function, length)) => {
                    let start: usize = cursor;
                    cursor += length;
                    functions.push((unresolved_function, source_index, start..cursor));
                },
                Err ((errors, length)) => {
                    syntax_errors.extend(errors);
                    if length == 0 {
                        return Err (syntax_errors)
                    }
                    cursor += length;
                }
            } }
        }
        if syntax_errors.is_empty() {
            self.unresolved_functions.extend(functions);
            Ok (())
        } else {
            Err (syntax_errors)
        }
    }

    /// Creates a new `Parser`
    pub fn new() -> Self {
        Self {
            namespace: Namespace::new(),
            sources:  SimpleFiles::new(),
            unresolved_functions: vec![],
        }
    }

}

/// Normalizes irregularities with the syntax that may cause issues during parsing
fn normalize(syntax: &str) -> String {
    // normalize cross-platform newlines
    let mut normalized: String = syntax
        .replace("\r\n", "\n")
        .replace('\r', "\n");
    // Strip UTF-8 byte order mark if present
    if normalized.starts_with('\u{FEFF}') {
        normalized = normalized.trim_start_matches('\u{FEFF}').to_string();
    }
    // normalize special unicode line separators with \n
    normalized = normalized
        .replace('\u{2028}', "\n")
        .replace('\u{2029}', "\n");
    normalized
}

/// Returns the number of bytes of non-whitespace in the string
fn preceding_non_whitespace(string: &str) -> usize {
    string
        .char_indices()
        .position(|(_, c)| c.is_whitespace())
        .unwrap_or(string.len())
}

/// Returns the number of bytes of preceding whitespace are in the string
fn preceding_whitespace(string: &str) -> usize {
    string
        .char_indices()
        .position(|(_, c)| !c.is_whitespace())
        .unwrap_or(string.len())
}