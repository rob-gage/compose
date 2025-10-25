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

    /// Creates a new `Namespace`
    pub fn new() -> Self {
        Self {
            function_storage: FunctionStorage::new(),
            functions_by_name: HashMap::new(),
        }
    }

    /// Resolves a function, stores it in the `Namespace` and returns it's `FunctionIndex`
    pub fn resolve_function(
        &mut self,
        name: &str,
        body: &[UnresolvedTerm]
    ) -> Result<FunctionIndex, HashSet<String>> {
        use UnresolvedTerm::*;
        let reserved_index: FunctionIndex = self.function_storage.reserve(body.len());
        self.functions_by_name.insert(name.to_string(), reserved_index);
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
                        resolved.push(Term::Application (reserved_index));
                    } else { undefined.insert(name.to_string()); },
                // resolve lambdas
                UnresolvedLambda (lambda_body) => {
                    let lambda: Data = Data::Lambda (match self.resolve_function("", lambda_body) {
                        Ok (lambda_index) =>
                            self.function_storage.get(lambda_index).to_vec(),
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
            self.function_storage.store(reserved_index, &resolved);
            Ok (reserved_index)
        } else {
            self.functions_by_name.remove(name);
            Err (undefined)
        }
    }

}

