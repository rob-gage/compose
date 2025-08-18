// Copyright Rob Gage 2025

use smallvec::{
    SmallVec,
    smallvec,
};
use std::{
    cell::UnsafeCell,
    fmt::{
        Display,
        Formatter
    },
    mem::swap,
};
use crate::virtual_machine::data::Data::Boolean;
use super::{
    Combinator,
    Data,
    FunctionStorage,
    Integer,
    Term,
};

/// How many terms on the stack are stored on the actual stack
const STACK_STACK_SIZE: usize = 1024;

/// A last-in-first-out stack that can store `Value`s and is used to evaluate programs
pub struct Stack {
    /// The buffer containing the data on the stack
    buffer: UnsafeCell<SmallVec<[Data; STACK_STACK_SIZE]>>,
    /// The size of the stack
    top: usize
}

impl Stack {


    /// Displays the top of the stack as a string
    pub fn display_stack(&self) -> String {
        const DISPLAY_COUNT: usize = 5;
        let mut collected: Vec<String> = Vec::new();
        for i in (0..DISPLAY_COUNT).rev() {
            if let Some(item) = self.get_from_top(i) {
                collected.push(format!("{}", item));
            }
        }
        let string: String = collected.join(" ");
        if self.size() > DISPLAY_COUNT {
            format!("... {}", string)
        } else {
            string
        }
    }


    /// Evaluates a `Combinator`
    pub fn evaluate_combinator(
        &mut self,
        storage: &FunctionStorage,
        combinator: Combinator
    ) -> Result<(), &'static str> {
        use crate::virtual_machine::combinator::Combinator::*;

        // helper function to apply a lambda, taking its term sequence indices
        fn apply_lambda(
            storage: &FunctionStorage,
            stack: &mut Stack,
            term_sequence_indices: &[usize]
        ) -> Result<(), &'static str> {
            for &index in term_sequence_indices {
                let sequence: &[Term] = storage.get_body(index);
                stack.evaluate_term_sequence(storage, sequence)?;
            }
            Ok (())
        }

        // helper function to perform an arithmetic operation on the stack
        fn arithmetic_operation(
            stack: &mut Stack,
            operation: fn(Integer, Integer) -> Integer
        ) -> Result<(), &'static str> {
            if stack.size() < 2 {
                Err ("Not enough items in the stack to perform arithmetic operation")
            } else {
                let (b, a): (Integer, Integer) = match (
                    stack.pop().unwrap(),
                    stack.pop().unwrap()
                ) {
                    (Data::Integer (b), Data::Integer (a)) => {
                        (b, a)
                    }
                    _ => return Err ("Can only perform arithmetic operation on integers")
                };
                stack.push(Data::Integer (operation(a, b)));
                Ok (())
            }
        }

        // helper function to perform a boolean logic operation on the stack
        fn boolean_logic_operation(
            stack: &mut Stack,
            operation: fn(bool, bool) -> bool,
        ) -> Result<(), &'static str> {
            if stack.size() < 2 {
                Err ("Not enough items in the stack to perform boolean logic operation")
            } else {
                let (b, a): (bool, bool) = match (
                    stack.pop().unwrap(),
                    stack.pop().unwrap()
                ) {
                    (Data::Boolean (b), Data::Boolean (a)) => {
                        (b, a)
                    }
                    _ => return Err ("Can only perform boolean logic operation on booleans")
                };
                stack.push(Data::Boolean (operation(a, b)));
                Ok (())
            }
        }

        // helper function to perform a comparison operation on the stack
        fn comparison_operation(
            stack: &mut Stack,
            operation: fn(Data, Data) -> Result<bool, &'static str>,
        ) -> Result<(), &'static str> {
            if stack.size() < 2 {
                Err ("Not enough items in the stack to perform comparison operation")
            } else {
                let (b, a): (Data, Data) = (stack.pop().unwrap(), stack.pop().unwrap());
                stack.push(Data::Boolean (operation(a, b)?));
                Ok (())
            }
        }

        match combinator {

            // arithmetic combinators

            Add => arithmetic_operation(self, |a, b| a + b),

            Divide => arithmetic_operation(self, |a, b| a / b),

            Modulo => arithmetic_operation(self, |a, b| a % b),

            Multiply => arithmetic_operation(self, |a, b| a * b),

            Subtract => arithmetic_operation(self, |a, b| a - b),

            // boolean logic combinators

            And => boolean_logic_operation(self, |a, b| a && b),

            ExclusiveOr => boolean_logic_operation(self, |a, b| a ^ b),

            Not => if let Some (top) = self.pop() {
                if let Data::Boolean (boolean) = top {
                    self.push(Data::Boolean (!boolean));
                    Ok (())
                } else {
                    Err ("Can only perform boolean \"not\" operation on boolean data")
                }
            } else { Err ("Cannot perform boolean \"not\" operation on empty stack") }

            Or => boolean_logic_operation(self, |a, b| a || b),

            // comparison combinators

            Equality => comparison_operation(self, |a, b| Ok(a == b)),

            GreaterThan => comparison_operation(self, |a, b| match (a, b) {
                (Data::Integer (a), Data::Integer (b)) => Ok (a > b),
                _ => Err ("Can only perform \"greater than\" operation on integers")
            }),

            LessThan => comparison_operation(self, |a, b| match (a, b) {
                (Data::Integer (a), Data::Integer (b)) => Ok (a < b),
                _ => Err ("Can only perform \"less than\" operation on integers")
            }),

            // functional combinators

            Apply => {
                let top: Option<Data> = self.pop();
                if let Some (Data::Lambda (sequence_indices)) = top {
                    apply_lambda(storage, self, &sequence_indices)?;
                    Ok (())
                } else { Err ("Stack must have a lambda on top to be applied") }
            },

            Compose => match self.pop() {
                Some (Data::Lambda (b_indices)) =>
                    match self.get_mutable_from_top(0) {
                    Some (Data::Lambda (a_indices)) => {
                        a_indices.extend(b_indices);
                        Ok (())
                    }
                    _ => Err (
                        "Cannot perform `compose` operation; second from top of stack is \
                                not a lambda"
                    )
                }
                _ => Err (
                    "Cannot perform `compose` operation; top of stack is not a lambda"
                )
            }

            If => match (self.pop(), self.pop()) {
                (
                    Some (Data::Lambda (false_indices)), Some (Data::Lambda (true_indices))
                ) => match self.pop() {
                    Some (Data::Boolean (boolean)) => if boolean {
                        apply_lambda(storage, self, &true_indices)?; Ok (())
                    } else {
                        apply_lambda(storage, self, &false_indices)?; Ok (())
                    }
                    _ => Err ("Cannot perform `if` operation unless there is a boolean below \
                        the two lambdas on top of the stack")
                },
                _ => Err ("Cannot perform `if` operation unless there are two lambdas on top of \
                the stack"),
            }

            Under => match (self.pop(), self.pop()) {
                (Some (Data::Lambda (b_indices)), Some (a)) => {
                    apply_lambda(storage, self, &b_indices)?;
                    self.push(a);
                    Ok (())
                }
                _ => Err ("Cannot perform `under` operation unless there is a lambda under another \
                item on top of the stack"),
            }

            // stack manipulation combinators

            Copy => if let Some (top) = self.get_from_top(0) {
                self.push(top.clone());
                Ok (())
            } else { Err ("No items the in stack to be copied") },

            Drop => {
                if let Some (top) = self.pop() {
                    Ok (())
                } else { Err ("No items in the stack to be dropped") }
            },

            Hop => {
                if let Some (top) = self.get_from_top(1) {
                    self.push(top.clone());
                    Ok (())
                } else { Err ("Not enough items in the stack to be hopped") }
            },

            Rotate => {
                if self.size() < 3 {
                    Err ("Not enough items in the stack to rotate")
                } else {
                    let a: &mut Data = self.get_mutable_from_top(2).unwrap();
                    let b: &mut Data = self.get_mutable_from_top(1).unwrap();
                    let c: &mut Data = self.get_mutable_from_top(0).unwrap();
                    swap(a, c);
                    swap(b, c);
                    Ok (())
                }
            },

            Swap => {
                if self.size() < 2 {
                    Err ("Not enough items in the stack to swap")
                } else {
                    swap(
                        self.get_mutable_from_top(0).unwrap(),
                        self.get_mutable_from_top(1).unwrap(),
                    );
                    Ok (())
                }
            },

            _ => Err ("Combinator is not yet implemented"),

        }
    }


    /// Evaluates a single `Term` on this `Runtime`
    fn evaluate_term(
        &mut self,
        storage: &FunctionStorage,
        term: &Term
    ) -> Result<(), &'static str> {
        // println!("evaluated term: {:?}", term);
        match term {

            Term::Application (identifier) => {
                let sequence: &[Term] = storage.get_body(*identifier);
                self.evaluate_term_sequence(storage, sequence)
            },

            Term::Combinator (combinator) => self.evaluate_combinator(storage, combinator.clone()),

            Term::Data (data) => {
                self.push(data.clone());
                Ok (())
            }

        }
    }


    /// Evaluates a sequence of `Term`s
    pub fn evaluate_term_sequence(
        &mut self,
        storage: &FunctionStorage,
        sequence: &[Term],
    ) -> Result<(), &'static str> {
        for term in sequence {
            self.evaluate_term(storage, term)?
        }
        Ok (())
    }


    /// Creates a new `Stack` with no data in it
    pub fn new() -> Self {
        Self {
            buffer: UnsafeCell::new(smallvec![]),
            top: 0,
        }
    }


    /// Gets a reference to the `Stack` item with a specified index from the top of the stack
    /// (`0` is the index for the top)
    pub fn get_from_top(&self, index: usize) -> Option<&Data> {
        let stack_index: usize = self.top.checked_sub(1 + index)?;
        unsafe {
            (*self.buffer.get()).get(stack_index)
        }
    }


    /// Gets a reference to the `Stack` item with a specified index from the top of the stack
    /// (`0` is the index for the top)
    pub fn get_mutable_from_top(&self, index: usize) -> Option<&mut Data> {
        let stack_index: usize = self.top.checked_sub(1 + index)?;
        unsafe {
            (*self.buffer.get()).get_mut(stack_index)
        }
    }


    /// Pops a `Value` off the array
    pub fn pop(&mut self) -> Option<Data> {
        if self.top == 0 {
            return None;
        }
        self.top -= 1;
        unsafe {
            Some((*self.buffer.get()).pop().unwrap())
        }
    }


    /// Pushes a `Value` onto the top of the stack
    pub fn push(&mut self, data: Data) {
        unsafe {
            (*self.buffer.get()).push(data);
        }
        self.top += 1;
    }


    /// Returns the size of the `Stack`
    pub const fn size(&self) -> usize {
        self.top
    }


}


impl Display for Stack {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for term in unsafe { &*self.buffer.get() } {
            write!(f, "  {}", term)?;
        }
        Ok(())
    }
}