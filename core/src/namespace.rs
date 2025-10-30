// Copyright Rob Gage 2025

use std::collections::{
    HashMap,
    HashSet,
};
use crate::{
    Data,
    Stack,
    Term,
    TermBuffer,
    TermSequence,
    TermSequenceReference,
    UnresolvedFunction,
    UnresolvedTerm
};

/// Allows definition and retrieval of named functions and anonymous functions
pub struct Namespace<'a> {
    /// The `TermBuffer` used to store functions in this namespace
    term_buffer: TermBuffer<'a>,
    /// The indices of defined functions in the function storage mapped by name
    functions_by_name: HashMap<String, TermSequenceReference>,
}

impl<'a> Namespace<'a> {

    /// Defines a new named function in this `Namespace`
    pub fn define(
        &'a mut self,
        name: &str,
        body: &[UnresolvedTerm],
    ) -> Result<TermSequence<'a>, HashSet<String>> {
        self.resolve(name, body)
            .map(move |index| self.term_buffer.get(index))
    }

    /// Evaluates a `TermSequence` on a `Stack`
    pub fn evaluate_terms(
        &self,
        stack: &mut Stack,
        terms: TermSequence
    ) -> Result<(), String> { terms.evaluate(&self.term_buffer, stack) }

    /// Creates a new `Namespace`
    pub fn new() -> Self {
        Self {
            term_buffer: TermBuffer::new(),
            functions_by_name: HashMap::new(),
        }
    }

    /// Resolves a function, stores it in the `Namespace` and returns it's `FunctionIndex`
    pub fn resolve(
        &mut self,
        function_name: &str,
        terms: &[UnresolvedTerm]
    ) -> Result<TermSequenceReference, HashSet<String>> {
        use UnresolvedTerm::*;
        let reserved: TermSequenceReference = self.term_buffer.reserve(terms.len());
        self.functions_by_name.insert(function_name.to_string(), reserved);
        let mut resolved: Vec<Term> = Vec::with_capacity(terms.len());
        let mut undefined: HashSet<String> = HashSet::new();
        for unresolved_term in terms {
            match unresolved_term {
                // nothing needs to be done with already resolved terms
                Resolved (term) => resolved.push(term.clone()),
                // resolve function applications
                UnresolvedApplication (unresolved_name) =>
                    if let Some (function_index) = self.functions_by_name.get(unresolved_name) {
                        resolved.push(Term::Application (*function_index));
                    } else if unresolved_name == function_name {
                        resolved.push(Term::Application (reserved));
                    } else { undefined.insert(function_name.to_string()); },
                // resolve lambdas
                UnresolvedLambda (lambda_body) => {
                    let lambda: Data = Data::Lambda (match self.resolve("", lambda_body) {
                        Ok (lambda_index) => vec![lambda_index],
                        Err (lambda_undefined) => {
                            undefined.extend(lambda_undefined);
                            vec![]
                        }
                    });
                    resolved.push(Term::Data (lambda));
                }
            }
        }
        if undefined.is_empty() {
            self.term_buffer.store(reserved, &resolved);
            Ok (reserved)
        } else {
            self.functions_by_name.remove(function_name);
            Err (undefined)
        }
    }


}

