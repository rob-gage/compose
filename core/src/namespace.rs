// Copyright Rob Gage 2025

use std::collections::{
    HashMap,
    HashSet,
};
use crate::{
    Data,
    FunctionIndex,
    FunctionStorage,
    Term,
    UnresolvedTerm,
};

/// Allows definition and retrieval of named functions and anonymous functions
pub struct Namespace {
    /// The function storage used to store functions defined in this namespace
    function_storage: FunctionStorage,
    /// The indices of defined functions in the function storage mapped by name
    functions_by_name: HashMap<String, FunctionIndex>,
}

impl Namespace {

    /// Defines a named function in this `Namespace`
    pub fn define_function(&mut self, name: &str, body: &[Term]) -> Result<(), ()> {
        if self.functions_by_name.contains_key(name) { Err (()) } else {
            let index: FunctionIndex = self.function_storage.store_function(body);
            self.functions_by_name.insert(name.to_string(), index);
            Ok (())
        }
    }

    /// Creates a new `Namespace`
    pub fn new() -> Self {
        Self {
            function_storage: FunctionStorage::new(),
            functions_by_name: HashMap::new(),
        }
    }

    /// Attempts to resolve a sequence of unresolved terms
    pub fn resolve_function(
        &mut self,
        name: &str,
        body: &[UnresolvedTerm]
    ) -> Result<FunctionIndex, HashSet<String>> {
        use UnresolvedTerm::*;
        let mut resolved: Vec<Term> = Vec::with_capacity(body.len());
        let mut undefined: HashSet<String> = HashSet::new();
        for unresolved_term in body {
            match unresolved_term {
                // nothing needs to be done with already resolved terms
                Resolved (term) => resolved.push(term.clone()),
                // resolve function applications
                UnresolvedApplication (unresolved_name) =>
                    if let Some (function_index) = self.functions_by_name.get(unresolved_name) {
                        resolved.push(Term::Application (*function_index));
                    } else if unresolved_name == name  {
                        resolved.push(Term::Application (self.function_storage.reserve()))
                    } else { undefined.insert(name.to_string()); },
                // resolve lambdas
                UnresolvedLambda (terms) => {
                    let lambda: Data = Data::Lambda (match self.resolve_function(terms) {
                        Ok (lambda_terms) => lambda_terms,
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
            Ok (self.function_storage.store_function(&resolved))
        } else { Err (undefined) }
    }

}

