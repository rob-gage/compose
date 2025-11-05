// Copyright Rob Gage 2025

use std::{
    collections::{
        HashMap,
        HashSet,
    },
    fmt::{
        Result as FormatResult,
        Write,
    },
};
use crate::{
    Data,
    Environment,
    FunctionReference,
    FunctionStorage,
    Term,
    UnresolvedFunction,
    UnresolvedTerm
};

/// Allows definition and retrieval of named functions and anonymous functions
pub struct Namespace {
    /// The `TermBuffer` used to store functions in this namespace
    environment: Environment,
    /// The indices of defined functions in the function storage mapped by name
    functions_by_name: HashMap<String, FunctionReference>,
    /// The names of functions defined in the `Namespace` mapped by their function index
    names_by_function: HashMap<FunctionReference, String>
}

impl Namespace {

    /// Defines a new `Function` in this `Namespace` from an `UnresolvedFunction`
    pub fn define(
        &mut self,
        unresolved_function: &UnresolvedFunction,
    ) -> Result<FunctionReference, HashSet<String>> {
        let reference: FunctionReference = FunctionReference::reserve(&mut self.environment);
        self.functions_by_name.insert(unresolved_function.name().to_string(), reference);
        self.names_by_function.insert(reference, unresolved_function.name().to_string());
        self.resolve(reference, unresolved_function.body())?;
        Ok (reference)
    }

    /// Creates a new `Namespace`
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
            functions_by_name: HashMap::new(),
            names_by_function: HashMap::new()
        }
    }

    /// Resolves an unresolved function body, and stores it in this `Namespace` at a specified
    /// index in the `FunctionStorage`
    fn resolve(
        &mut self,
        reference: FunctionReference,
        unresolved_body: &[UnresolvedTerm],
    ) -> Result<(), HashSet<String>> {
        use UnresolvedTerm::*;
        let mut resolved: Vec<Term> = Vec::with_capacity(unresolved_body.len());
        let mut undefined: HashSet<String> = HashSet::new();
        for unresolved_term in unresolved_body {
            match unresolved_term {
                // nothing needs to be done with already resolved terms
                Resolved (term) => resolved.push(term.clone()),
                // resolve function applications
                UnresolvedApplication (unresolved_name) =>
                    if let Some (application_reference)
                        = self.functions_by_name.get(unresolved_name) {
                        resolved.push(Term::Application (*application_reference));
                    } else { undefined.insert(unresolved_name.to_string()); },
                // resolve lambdas
                UnresolvedLambda (lambda_body) => {
                    let lambda_reference: FunctionReference
                        = FunctionReference::reserve(&mut self.environment);
                    let lambda: Data = Data::Lambda (
                        match self.resolve(lambda_reference, lambda_body) {
                            Ok (_) => vec![lambda_index],
                            Err (lambda_undefined) => {
                                undefined.extend(lambda_undefined);
                                vec![]
                            } 
                        }
                    );
                    resolved.push(Term::Data (lambda));
                }
            }
        }
        if undefined.is_empty() {
            self.function_storage.store(function_index, &resolved);
            Ok (())
        } else { Err (undefined) }
    }

    /// Displays a term within the context of this `Namespace`
    pub fn write_term<W: Write>(&self, w: &mut W, term: &Term) -> FormatResult {
        match term {
            Term::Application (function_index) => w.write_str(
                self.names_by_function.get(&function_index)
                    .expect("`Namespace::format_term` will only be called on terms that exist \
                   in this `Namespace`")
            ),
            Term::Combinator (combinator) => w.write_str(combinator.name()),
            Term::Data (data) => data.write(w, self),
            Term::Recursion => w.write_str("@"),
        }
    }

}

