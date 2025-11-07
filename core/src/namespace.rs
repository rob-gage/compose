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
    sync::{
        Arc,
        RwLock,
    }
};
use crate::{
    Value,
    Environment,
    FunctionReference,
    LambdaReference,
    Term,
    VirtualMachine,
    UnresolvedFunction,
    UnresolvedTerm
};

/// Allows definition and retrieval of named functions and anonymous functions
pub struct Namespace {
    /// The `TermBuffer` used to store functions in this namespace
    environment: Arc<RwLock<Environment>>,
    /// The indices of defined functions in the function storage mapped by name
    functions_by_name: HashMap<String, FunctionReference>,
    /// The names of functions defined in the `Namespace` mapped by their function index
    names_by_function: HashMap<FunctionReference, String>
}

impl Namespace {

    /// Creates a new `VirtualMachine` from this `Namespace`
    pub fn create_virtual_machine(&self) -> VirtualMachine {
        VirtualMachine::from_environment(&self.environment)
    }

    /// Defines a new `Function` in this `Namespace` from an `UnresolvedFunction`
    pub fn define(
        &mut self,
        unresolved_function: &UnresolvedFunction,
    ) -> Result<FunctionReference, HashSet<String>> {
        let environment: &mut Environment = &mut *self.environment.write().unwrap();
        let reference: FunctionReference = FunctionReference::reserve(environment);
        self.functions_by_name.insert(unresolved_function.name().to_string(), reference);
        self.names_by_function.insert(reference, unresolved_function.name().to_string());
        resolve(environment, &self.functions_by_name, reference, unresolved_function.body())?;
        Ok (reference)
    }

    /// Creates a new `Namespace`
    pub fn new() -> Self {
        Self {
            environment: Arc::new(RwLock::new(Environment::new())),
            functions_by_name: HashMap::new(),
            names_by_function: HashMap::new()
        }
    }

    /// Displays a term within the context of this `Namespace`
    pub fn write_term<W: Write>(&self, w: &mut W, term: &Term) -> FormatResult {
        match term {
            Term::Application (reference) => w.write_str(
                self.names_by_function.get(&reference)
                    .expect("`Namespace::format_term` will only be called on terms that exist \
                   in this `Namespace`")
            ),
            Term::Combinator (combinator) => w.write_str(combinator.name()),
            Term::Data (data) => match data {
                Value::Boolean (boolean) => w.write_str(if *boolean { "true" } else { "false" }),
                Value::Integer (integer) => w.write_str(&integer.to_string()),
                Value::Lambda (reference) => {
                    w.write_str("( ")?;
                    for term in reference.get(&*self.environment.read().unwrap()).body() {
                        self.write_term(w, term)?;
                        w.write_char(' ')?;
                    }
                    w.write_char(')')
                },
                Value::List (items) => {
                    w.write_str("[ ")?;
                    for item in items {
                        self.write_term(w, &Term::Data (data.clone()))?;
                        w.write_char(' ')?;
                    }
                    w.write_char(']')
                }
            },
            Term::Recursion => w.write_str("@"),
        }
    }

}

/// Resolves an unresolved function body, and stores it in this `Namespace` at a specified
/// index in the `FunctionStorage`
fn resolve(
    environment: &mut Environment,
    functions_by_name: &HashMap<String, FunctionReference>,
    function_reference: FunctionReference,
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
                if let Some (reference) = functions_by_name.get(unresolved_name) {
                    resolved.push(Term::Application (*reference));
                } else { undefined.insert(unresolved_name.to_string()); },
            // resolve lambdas
            UnresolvedLambda (lambda_body) => {
                let reference: FunctionReference
                    = FunctionReference::reserve(environment);
                let lambda: Value = Value::Lambda (
                    match resolve(environment ,functions_by_name, function_reference, lambda_body) {
                        Ok (_) => LambdaReference::from_function(reference),
                        Err (lambda_undefined) => {
                            undefined.extend(lambda_undefined);
                            LambdaReference::from_function(reference)
                        }
                    }
                );
                resolved.push(Term::Data (lambda));
            }
        }
    }
    if undefined.is_empty() {
        function_reference.set_body(environment, &resolved);
        Ok (())
    } else { Err (undefined) }
}