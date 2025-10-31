// Copyright Rob Gage 2025

use std::collections::{
    HashMap,
    HashSet,
};
use crate::{
    Data,
    Function,
    FunctionStorage,
    Stack,
    Term,
    UnresolvedFunction,
    UnresolvedTerm
};

/// Allows definition and retrieval of named functions and anonymous functions
pub struct Namespace {
    /// The `TermBuffer` used to store functions in this namespace
    function_storage: FunctionStorage,
    /// The indices of defined functions in the function storage mapped by name
    functions_by_name: HashMap<String, usize>,
}

impl Namespace {

    /// Defines a new `Function` in this `Namespace` from an `UnresolvedFunction`
    pub fn define(
        &mut self,
        unresolved_function: &UnresolvedFunction,
    ) -> Result<Function, HashSet<String>> {
        let function_index: usize = self.resolve(unresolved_function.body())?;
        self.functions_by_name.insert(unresolved_function.name().to_string(), function_index);
        Ok (Function::from_function_index (function_index))
    }

    /// Returns the `FunctionStorage` used by this `Namespace`
    pub const fn function_storage(&self) -> &FunctionStorage { &self.function_storage }

    /// Creates a new `Namespace`
    pub fn new() -> Self {
        Self {
            function_storage: FunctionStorage::new(),
            functions_by_name: HashMap::new(),
        }
    }

    /// Resolves an unresolved function body, storing it in this `Namespace`, and returning its
    /// index in this `Namespace`'s `FunctionStorage`
    fn resolve(
        &mut self,
        unresolved_body: &[UnresolvedTerm],
    ) -> Result<usize, HashSet<String>> {
        use UnresolvedTerm::*;
        let mut resolved: Vec<Term> = Vec::with_capacity(unresolved_body.len());
        let mut undefined: HashSet<String> = HashSet::new();
        for unresolved_term in unresolved_body {
            match unresolved_term {
                // nothing needs to be done with already resolved terms
                Resolved (term) => resolved.push(term.clone()),
                // resolve function applications
                UnresolvedApplication (unresolved_name) =>
                    if let Some (function_index) = self.functions_by_name.get(unresolved_name) {
                        resolved.push(Term::Application (*function_index));
                    } else { undefined.insert(unresolved_name.to_string()); },
                // resolve lambdas
                UnresolvedLambda (lambda_body) => {
                    let lambda: Data = Data::Lambda (match self.resolve(lambda_body) {
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
            Ok (self.function_storage.store(&resolved))
        } else { Err (undefined) }
    }


}

