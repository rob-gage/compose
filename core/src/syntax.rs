// Copyright Rob Gage 2025

mod syntax_error;
mod unresolved_function;
mod unresolved_term;

use syntax_error::SyntaxErrorVariant;

pub use syntax_error::SyntaxError;
pub use unresolved_function::UnresolvedFunction;
pub use unresolved_term::UnresolvedTerm;