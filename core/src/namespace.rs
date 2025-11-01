// Copyright Rob Gage 2025

use std::{
    collections::{
        HashMap,
        HashSet,
    },
    fmt::{
        Formatter,
        Result as FormatResult,
    },
};
use std::fmt::Debug;
use crate::{
    Data,
    Function,
    FunctionStorage,
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
    /// The names of functions defined in the `Namespace` mapped by their function index
    names_by_function: HashMap<usize, String>
}

impl Namespace {

    /// Defines a new `Function` in this `Namespace` from an `UnresolvedFunction`
    pub fn define(
        &mut self,
        unresolved_function: &UnresolvedFunction,
    ) -> Result<Function, HashSet<String>> {
        let function_index: usize = self.function_storage.reserve();
        self.functions_by_name.insert(unresolved_function.name().to_string(), function_index);
        self.names_by_function.insert(function_index, unresolved_function.name().to_string());
        self.resolve(function_index, unresolved_function.body())?;
        Ok (Function::from_function_index (function_index))
    }

    /// Displays a term within the context of this `Namespace`
    pub fn format_term(&self, f: &mut Formatter, term: &Term) -> FormatResult {
       match term {
           Term::Application (function_index) => f.write_str(
               self.names_by_function.get(&function_index)
                   .expect("`Namespace::format_term` will only be called on terms that exist \
                   in this `Namespace`")
           ),
           Term::Combinator (combinator) => f.write_str(combinator.name()),
           Term::Data (data) => data.format(f, self),
           Term::Recursion => f.write_str("@"),
       }
    }

    /// Returns the `FunctionStorage` used by this `Namespace`
    pub const fn function_storage(&self) -> &FunctionStorage { &self.function_storage }

    /// Creates a new `Namespace`
    pub fn new() -> Self {
        Self {
            function_storage: FunctionStorage::new(),
            functions_by_name: HashMap::new(),
            names_by_function: HashMap::new()
        }
    }

    /// Resolves an unresolved function body, and stores it in this `Namespace` at a specified
    /// index in the `FunctionStorage`
    fn resolve(
        &mut self,
        function_index: usize,
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
                    if let Some (function_index) = self.functions_by_name.get(unresolved_name) {
                        resolved.push(Term::Application (*function_index));
                    } else { undefined.insert(unresolved_name.to_string()); },
                // resolve lambdas
                UnresolvedLambda (lambda_body) => {
                    let lambda_index: usize = self.function_storage.reserve();
                    let lambda: Data = Data::Lambda (match self.resolve(lambda_index, lambda_body) {
                        Ok (_) => vec![lambda_index],
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
            self.function_storage.store(function_index, &resolved);
            Ok (())
        } else { Err (undefined) }
    }


}

