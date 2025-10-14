// Copyright Rob Gage 2025

use std::{
    cell::UnsafeCell,
    collections::{
        HashMap,
        HashSet,
    },
    sync::Arc,
};
use crate::{
    Data,
    Function,
    FunctionStorage,
    Term,
    UnresolvedFunction,
    UnresolvedTerm,
};

/// Allows definition and retrieval of named functions and anonymous functions
pub struct Namespace {
    /// The function storage used to store functions defined in this namespace
    function_storage: Arc<UnsafeCell<FunctionStorage>>,
    /// The indices of defined functions in the function storage mapped by name
    indices_by_name: HashMap<String, usize>,
    /// The names of defined functions in the function storage mapped by index
    names_by_index: HashMap<usize, String>,
}

impl Namespace {

    // /// Resolves and defines a new function in this `Namespace`
    // pub fn define(
    //     &mut self,
    //     unresolved_function: UnresolvedFunction
    // ) -> Result<Function, HashSet<String>> {
    //     let index: usize = self.resolve_body(unresolved_function.body())?;
    //     if let Some (name) = unresolved_function.name() {
    //         self.indices_by_name.insert(name.to_string(), index);
    //         self.names_by_index.insert(index, name.to_string());
    //     }
    //     Ok (Function::new(&self.function_storage, index))
    // }

    /// Displays a term
    pub fn display_term(&self, term: &Term) -> String {
        match term {
            Term::Application (function_index) => self.names_by_index[function_index].clone(),
            Term::Combinator (combinator) => combinator.name().to_string(),
            Term::Data (data) => data.display(&self),
        }
    }

    /// Gets a named `Function` from the `Namespace` if it exists
    pub fn get(&self, name: &str) -> Option<Function> {
        if let Some (index) = self.indices_by_name.get(name) {
            Some (Function::new(&self.function_storage, *index))
        } else { None }
    }

    /// Creates a new `Namespace`
    pub fn new() -> Self {
        Self {
            function_storage: Arc::new(UnsafeCell::new(FunctionStorage::new())),
            indices_by_name: HashMap::new(),
            names_by_index: HashMap::new(),
        }
    }

    /// Attempts to resolve a `&[UnresolvedTerm]` lambda, returning its index
    fn resolve_body(
        &mut self,
        unresolved_body: &[UnresolvedTerm],
    ) -> Result<usize, HashSet<String>> {
        use UnresolvedTerm::*;
        let storage: &mut FunctionStorage = unsafe { &mut *self.function_storage.get() };
        // check for missing dependencies required for resolution
        let missing_dependencies: HashSet<String> = HashSet::from_iter(
            unresolved_body.iter()
                .filter_map(|t| match t {
                    UnresolvedTerm::UnresolvedApplication (name) => if self.indices_by_name
                        .contains_key(name) {
                        None
                    } else {
                        Some (name.clone())
                    },
                    _ => None,
                })
                .collect::<Vec<String>>()
        );
        if !missing_dependencies.is_empty() {
            return Err(missing_dependencies);
        }
        // iterate through function body and resolve terms
        let mut resolved_terms: Vec<Term> = vec![];
        for term in unresolved_body {
            match term {
                // leave already resolved terms alone
                Resolved (term) => resolved_terms.push(term.clone()),
                // resolve function application
                UnresolvedApplication (name)
                => if let Some (applied_function_index) = self.indices_by_name.get(name) {
                    resolved_terms.push(Term::Application (*applied_function_index));
                } else { unreachable!("Unresolvable applications are already filtered out") },
                // resolve lambdas by storing them as functions
                UnresolvedLambda (unresolved_terms) => {
                    let lambda_index: usize = self.resolve_body(unresolved_terms)?;
                    let function: Function = Function::new(&self.function_storage, lambda_index);
                    resolved_terms.push(Term::Data (Data::Lambda (function.body().to_vec())))
                },
                // resolves `Recursion`s to `Application`s with the index this function will have
                UnresolvedRecursion => resolved_terms.push(
                    Term::Application (storage.next_index())
                ),
            }
        }
        Ok (storage.store_function(&resolved_terms))
    }

}

