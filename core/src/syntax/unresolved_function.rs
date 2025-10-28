// Copyright Rob Gage 2025

use super::{
    SyntaxError,
    UnresolvedTerm
};
use pups::*;

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
            .map(|(name, body)| Self { body, name: name.to_string() })
            .parse(input)
    }

}

//
// impl UnresolvedFunction {
//
//     /// Creates a new anonymous function from syntax made of the body terms
//     pub fn anonymous_function(body_syntax: &str) -> Result<Self, Vec<SyntaxError>> {
//         Ok (Self {
//             body: Self::parse_free_terms(0, body_syntax, 0)
//                 .map_err(|(errors, _)| errors)?.0,
//             name: "".to_string(),
//         })
//     }
//
//     /// Returns the body of this `UnresolvedFunction`
//     pub fn body(&self) -> &[UnresolvedTerm] {
//         &self.body
//     }
//
//     /// Attempts to parse an `UnresolvedFunction` from syntax
//     pub fn parse(
//         source_index: usize,
//         syntax: &str,
//         start: usize,
//     ) -> Result<(UnresolvedFunction, usize), (Vec<SyntaxError>, usize)> {
//         let mut errors: Vec<SyntaxError> = Vec::new();
//         let mut offset = start;
//         let mut remaining = &syntax[offset..];
//         // parse the function name
//         let (unparsed, name) = match parse_identifier(remaining) {
//             Ok(ok) => ok,
//             Err(_) => {
//                 let err_span = offset
//                     ..offset + preceding_non_whitespace(remaining);
//                 return Err((
//                     vec![SyntaxError::new(
//                         source_index,
//                         SyntaxErrorVariant::ExpectedDefinition,
//                         err_span,
//                     )],
//                     offset,
//                 ));
//             }
//         };
//         offset += remaining.len() - unparsed.len();
//         remaining = unparsed;
//         // skip whitespace before colon
//         let whitespace_length: usize = preceding_whitespace(remaining);
//         offset += whitespace_length;
//         remaining = &remaining[whitespace_length..];
//         // expect colon
//         if !remaining.starts_with(":") {
//             return Err((
//                 vec![SyntaxError::new(
//                     source_index,
//                     SyntaxErrorVariant::ExpectedColon,
//                     offset..offset,
//                 )],
//                 // consume until `;` or end of file
//                 remaining.find(';').map(|i| offset + i).unwrap_or(syntax.len()),
//             ));
//         }
//         // consume colon
//         offset += 1;
//         remaining = &remaining[1..];
//         // skip whitespace before terms
//         let whitespace_length: usize = preceding_whitespace(remaining);
//         offset += whitespace_length;
//         remaining = &remaining[whitespace_length..];
//         // parse function body terms
//         let (terms, body_consumed) =
//             match Self::parse_free_terms(source_index, syntax, offset) {
//                 Ok(ok) => ok,
//                 Err(err) => return Err(err),
//             };
//         offset += body_consumed;
//         remaining = &syntax[offset..];
//         // expect semicolon
//         if !remaining.starts_with(";") {
//             return Err((
//                 vec![SyntaxError::new(
//                     source_index,
//                     SyntaxErrorVariant::ExpectedSemicolon,
//                     offset..offset,
//                 )],
//                 offset,
//             ));
//         }
//         // consume semicolon
//         offset += 1;
//         Ok((Self { body: terms, name }, offset - start))
//     }
//
//     /// Attempts to parse a function body (`Vec<UnresolvedTerm>`)
//     fn parse_free_terms(
//         source_index: usize,
//         syntax: &str,
//         start: usize,
//     ) -> Result<(Vec<UnresolvedTerm>, usize), (Vec<SyntaxError>, usize)> {
//         let mut terms = Vec::new();
//         let mut errors = Vec::new();
//         let mut offset = start;
//         let mut remaining = &syntax[offset..];
//         // loop until  ';' or end of syntax
//         while !remaining.is_empty() && !remaining.starts_with(";") {
//             match UnresolvedTerm::parse(remaining) {
//                 Ok((after_term, term)) => {
//                     let consumed = remaining.len() - after_term.len(); // track consumed length
//                     offset += consumed;
//                     remaining = after_term;
//                     terms.push(term);
//                 }
//                 Err(_) => {
//                     let invalid_length: usize = preceding_non_whitespace(remaining);
//                     errors.push(SyntaxError::new(
//                         source_index,
//                         SyntaxErrorVariant::ExpectedTerm,
//                         offset..offset + invalid_length,
//                     ));
//                     // skip past the invalid syntax
//                     offset += invalid_length;
//                     remaining = &syntax[offset..];
//                 }
//             }
//             // skip whitespace after term or invalid syntax
//             let whitespace_length: usize = preceding_whitespace(remaining);
//             offset += whitespace_length;
//             remaining = &syntax[offset..];
//         }
//         if errors.is_empty() {
//             Ok((terms, offset - start))
//         } else {
//             Err((errors, offset - start))
//         }
//     }
//
//     /// Returns the name of this `UnresolvedFunction`
//     pub fn name(&self) -> Option<&str> {
//         if self.name.is_empty() { None } else { Some(self.name.as_str()) }
//     }
//
// }