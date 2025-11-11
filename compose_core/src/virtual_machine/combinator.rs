// Copyright Rob Gage 2025

use crate::{
    Value,
    Integer,
};
use std::mem::swap;
use crate::virtual_machine::Combinator::Branch;
use super::{
    ControlAction::{
        self,
        *
    },
    DataStack,
    Environment,
    Function,
};

/// Defines `Combinator` enum
macro_rules! combinators {
    (
        $(
            $(#[$meta:meta])*
            $variant:ident ; $token:expr
        ),* $(,)?
    ) => {

        /// A concatenative combinator that modifies the stack
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum Combinator {
            $(
                $variant,
            )*
        }

        impl Combinator {

            /// Returns the name of the `Combinator`
            pub const fn name(&self) -> &'static str {
                match self {
                    $(
                        Combinator::$variant => $token,
                    )*
                }
            }

        }
    }
}



macro_rules! implementation {
    ($body:expr) => {{
        fn f(vm: &'a mut VirtualMachine<'a>) -> Result<(), String> {
            $body(vm)
        }
        f
    }};
}


combinators! {

    /// # Arithmetic Combinators

    /// ## Add
    ///
    /// `a b -> (a + b)`
    ///
    /// Adds the two numbers on top of the stack
    Add
    ; "+",

    /// ## Divide
    ///
    /// `a b -> (a / b)`
    ///
    /// Divides the second number on top of the stack by the number on top of the stack
    Divide
    ; "/",

    /// ## Multiply
    ///
    /// `a b -> (a * b)`
    ///
    /// Multiplies the two numbers on top of the stack
    Multiply
    ; "*",

    /// ## Remainder
    ///
    /// `a b -> (a % b)`
    ///
    /// Evaluates to the remainder of the second number on top of the stack divided by the first
    /// number on top of the stack
    Remainder
    ; "%",

    /// ## Subtract
    ///
    /// `a b -> (a - b)`
    ///
    /// Subtracts the number on top of the stack from the second number on top of the stack
    Subtract
    ; "-",

    /// # Boolean Logic Combinators

    /// ## And
    ///
    /// `a b -> (a & b)`
    ///
    /// Transforms the two booleans on top of the stack to one with a true value if they are both
    /// true, otherwise transforms them into a boolean with a false value.
    And
    ; "&",

    ///    /// ## Exclusive Or
    /// `a b -> (a ^ b)`
    ///
    /// Transforms the two booleans on top of the stack to one with a true value if only one is
    /// true, otherwise transforms them into a boolean with a false value.
    ExclusiveOr
    ; "^",

    /// ## Not
    ///
    /// `a -> !a`
    ///
    /// Transforms a boolean on top of the stack to true if it is false, and false if it is true.
    Not
    ; "!",

    /// ## Or
    ///
    /// `a b -> (a | b)`
    ///
    /// Transforms the two booleans on top of the stack to one with a true value if either one is
    /// true, otherwise transforms them into a boolean with a false value.
    Or
    ; "|",

    /// ## Comparison Combinators

    /// ## Equality
    ///
    /// `a b -> (a = b)`
    ///
    /// Evaluates to a true boolean value if the top two items on the stack are equal, otherwise
    /// evaluates to a false boolean value.
    Equality
    ; "=",

    /// ## Greater Than
    ///
    /// `a b -> (a > b)`
    ///
    /// Evaluates to a true boolean value if the integer on top of the stack is less than the
    /// one below it.
    GreaterThan
    ; ">",

    /// ## Less Than
    ///
    /// `a b -> (a < b)`
    ///
    /// Evaluates to a true boolean value if the integer on top of the stack is greater than the
    /// one below it.
    LessThan
    ; "<",

    /// # Functional Combinators

    /// ## Apply
    ///
    /// `a |f| -> a ...`
    ///
    /// Applies the function on top of the stack
    Apply
    ; "apply",

    /// ## Branch
    ///
    /// `a |f| |g| b -> a ...`
    ///
    /// Applies function `|f|` (third from top of stack) to term `a` (fourth from top of stack) if
    /// boolean `b` (top of stack) is a true, otherwise applies function `|g|` (second from tbe top
    /// of stack) to term `a` and the stack below
    Branch
    ; "?",

    /// ## Compose
    ///
    /// `|f| |g| -> |f g|`
    ///
    /// Composes a function from two functions on top of the stack
    Compose
    ; "compose",

    /// ## Under
    ///
    /// `a ... |f| # -> a ...`
    ///
    /// Consumes an index, from the top of the stack, and a lambda below it, and applies the lambda
    /// at the index (after consuming the lambda)
    Deep
    ; "deep",

    /// ## Under
    ///
    /// `a b |f| -> a ... b`
    ///
    /// Applies the function on top of the stack to the second value from the top of the stack
    Under
    ; "under",

    /// # List Processing Combinators

    /// ## Construct
    ///
    /// `a [x] -> [y]`
    ///
    /// Prepends an element `a` (second from top of the stack) to the list `[x]` (top of the
    /// stack)
    Append
    ; "append",

    /// ## Count
    ///
    /// `[x] -> a`
    ///
    /// Turns a list on top of the stack into its size
    Count
    ; "count",

    /// ## Filter
    ///
    /// `[x] |f| -> [y]`
    ///
    /// Filters the list `[x]` (second from top of the stack), keeping only the items
    /// that match a predicate function `|f|` (top of the stack)
    Filter
    ; "filter",

    /// ## Fold
    ///
    /// `[x] a |f| -> a`
    ///
    /// Reduces the list `[x]` (third from top of the stack) into a single accumulated
    /// value by applying the function `|f|` (on top of the stack) to the accumulator `a` (second
    /// from top of the stack) with each element of the list.
    Fold
    ; "fold",

    /// ## Head
    ///
    /// `[x] -> a`
    ///
    /// Returns the first element in the list on top of the stack
    Head
    ; "head",

    /// ## Join
    ///
    /// `[x] [y] -> [x y]`
    ///
    /// Joins the two lists on top of the stack into one
    Join
    ; "join",

    /// ## Map
    ///
    /// `[x] |f| -> [y]`
    ///
    /// Applies the function `|f|` (top of the stack) to every item in the list `[x]` (second
    /// from top of the stack) creating a new list
    Map
    ; "map",

    /// ## Tail
    ///
    /// `[x] -> a`
    ///
    /// Returns everything but the first element in the list on top of the stack
    Tail
    ; "tail",

    /// # Stack Manipulation Combinators

    /// ## Copy
    ///
    /// `a -> a a`
    ///
    /// Duplicates the item on top of the stack
    Copy
    ; "copy",

    /// ## Drop
    ///
    /// `a b -> a`
    ///
    /// Removes the item on top of the stack
    Drop
    ; "drop",

    /// ## Hop
    ///
    /// `a b -> a b a`
    ///
    /// Pushes a duplicate of the second value from top of the stack
    Hop
    ; "hop",

    /// ## Pick
    ///
    /// `c ... # -> ... c`
    ///
    /// Consumes an index from the top of the stack, and then pushes a duplicate of the item at that
    /// index
    Pick
    ; "pick",

    /// ## Rotate
    ///
    /// `a b c -> c a b`
    ///
    /// Moves the top item on the stack to the position below the next top two items
    Rotate
    ; "rotate",

    /// ## Swap
    ///
    /// `a b -> b a`
    ///
    /// Swaps the two items on top of the stack
    Swap
    ; "swap",

}
impl Combinator {

    /// Evaluates this `Combinator` on a `VirtualMachine`
    pub fn evaluate<'a>(
        &self,
        stack: &mut DataStack,
        environment: &'a Environment
    ) -> ControlAction<'a> {
        use Combinator::*;
        match self {

            // arithmetic combinators

            Add => arithmetic_operation(stack, |a, b| a + b),

            Divide => arithmetic_operation(stack, |a, b| a / b),

            Remainder => arithmetic_operation(stack, |a, b| a % b),

            Multiply => arithmetic_operation(stack, |a, b| a * b),

            Subtract => arithmetic_operation(stack, |a, b| a - b),

            // boolean combinators

            And => boolean_logic_operation(stack, |a, b| a && b),

            ExclusiveOr => boolean_logic_operation(stack, |a, b| a ^ b),

            Not => if let Some(top) = stack.pop() {
                if let Value::Boolean(boolean) = top {
                    stack.push(Value::Boolean(!boolean));
                    Continue
                } else {
                    Error ("Can only perform boolean \"not\" operation on boolean data".to_string())
                }
            } else {
                Error ("Cannot perform boolean \"not\" operation on empty stack".to_string())
            },

            Or => boolean_logic_operation(stack, |a, b| a || b),

            // comparison combinators

            Equality => comparison_operation(stack, |a, b| Ok(a == b)),

            GreaterThan => comparison_operation(stack, |a, b| match (a, b) {
                (Value::Integer(a), Value::Integer(b)) => Ok(a > b),
                _ => Err("Can only perform \"greater than\" operation on integers")
            }),

            LessThan => comparison_operation(stack, |a, b| match (a, b) {
                (Value::Integer(a), Value::Integer(b)) => Ok(a < b),
                _ => Err("Can only perform \"less than\" operation on integers")
            }),

            // functional combinators

            Apply => {
                let top: Option<Value> = stack.pop();
                if let Some (Value::Lambda (reference)) = top {
                    let lambda: Function = reference.get(environment);
                    Push (lambda)
                } else { Error ("Stack must have a lambda on top to be applied".to_string()) }
            },

            Compose => match (stack.pop(), stack.pop()) {
                (Some (Value::Lambda (b_reference)), Some (Value::Lambda (a_reference))) => {
                    stack.push(Value::Lambda (a_reference.compose(b_reference)));
                    Continue
                }
                _ => Error ("Cannot perform `compose` operation unless there are two lambdas on \
                top of the stack".to_string()),
            }

            Branch => match (stack.pop(), stack.pop()) {
                (Some(Value::Lambda (false_reference)), Some(Value::Lambda(true_reference))) =>
                    match stack.pop() {
                        Some(Value::Boolean (boolean)) => if boolean {
                            let true_lambda: Function = true_reference.get(environment);
                            Push (true_lambda)
                        } else {
                            let false_lambda: Function = false_reference.get(environment);
                            Push (false_lambda)
                        }
                        _ => Error ("Cannot perform `?` operation unless there is a boolean below \
                        the two lambdas on top of the stack".to_string())
                    },
                _ => Error ("Cannot perform `?` operation unless there are two lambdas on top of \
                the stack".to_string()),
            }

            Deep => match (stack.pop(), stack.pop()) {
                (Some (Value::Integer (integer)), Some (Value::Lambda (reference))) => {
                    if let Some (terms) = stack.pop_slice(integer.as_stack_index(stack.size())) {
                        let lambda: Function = reference.get(environment)
                            .extended_with_data(&terms);
                        Push (lambda)
                    } else { Error ("Cannot perform `deep` operation unless there is an index on \
                    top of the stack, and a lambda below it".to_string()) }
                }
                _ => Error ("Cannot perform `deep` operation unless there is an index on \
                    top of the stack, and a lambda below it".to_string())
            }

            Under => match (stack.pop(), stack.pop()) {
                (Some(Value::Lambda (reference)), Some(top)) => {
                    let lambda: Function = reference.get(environment)
                        .extended_with_data(&[top]);
                    Push (lambda)
                }
                _ => Error ("Cannot perform `under` operation unless there is a lambda under \
                another item on top of the stack".to_string()),
            }

            // list combinators

            Append => match (stack.pop(), stack.pop()) {
                (Some (value), Some (Value::List (mut list_items))) => {
                    list_items.push(value);
                    stack.push(Value::List (list_items));
                    Continue
                }
                _ => Error ("Cannot perform `append` unless there is a list below the value to \
                be appended to it on the stack".to_string()),
            }

            Join => match (stack.pop(), stack.pop()) {
                (Some (Value::List (list_b)), Some (Value::List (mut list_a))) => {
                    list_a.extend(list_b);
                    stack.push(Value::List (list_a));
                    Continue
                }
                _ => Error ("Cannot perform `join` unless there are two lists on top of the stack\
                ".to_string()),
            }
            
            // stack manipulation combinators

            Copy => if let Some(top) = stack.get_from_top(0) {
                stack.push(top.clone());
                Continue
            } else { Error ("No items the in stack to be copied".to_string()) },
    
            Drop => if let Some(_) = stack.pop() {
                Continue
            } else { Error ("No items in the stack to be dropped".to_string()) },
    
            Hop => if let Some(top) = stack.get_from_top(1) {
                stack.push(top.clone());
                Continue
            } else { Error ("Not enough items in the stack to be hopped".to_string()) },

            Pick => if let Some (Value::Integer (integer)) = stack.pop() {
                let Some (indexed) = stack.get_from_top(integer.as_stack_index(stack.size()))
                else { return Error ("Cannot perform `pick` on an empty stack.".to_string()); } ;
                stack.push(indexed.clone());
                Continue
            } else { Error ("Missing integer on top of the stack to represent index for `pick` \
            operation".to_string()) },
    
            Rotate => if stack.size() < 3 {
                Error ("Not enough items in the stack to rotate".to_string())
            } else {
                let a: &mut Value = stack.get_mutable_from_top(2).unwrap();
                let b: &mut Value = stack.get_mutable_from_top(1).unwrap();
                let c: &mut Value = stack.get_mutable_from_top(0).unwrap();
                swap(a, c);
                swap(b, c);
                Continue
            },
    
            Swap => if stack.size() < 2 {
                Error ("Not enough items in the stack to swap".to_string())
            } else {
                swap(
                    stack.get_mutable_from_top(0).unwrap(),
                    stack.get_mutable_from_top(1).unwrap(),
                );
                Continue
            }

            // --------------------------------------------------------------------------------

            _ => Error (String::from("Combinator not implemented yet."))

        }
    }

}



/// Evaluates an arithmetic operation on a `VirtualMachine`
fn arithmetic_operation<'a>(
    stack: &mut DataStack,
    operation: fn(Integer, Integer) -> Integer
) -> ControlAction<'a> {
    if stack.size() < 2 {
        Error("Not enough items in the stack to perform arithmetic operation"
            .to_string())
    } else {
        let (b, a): (Integer, Integer) = match (
            stack.pop().unwrap(),
            stack.pop().unwrap()
        ) {
            (Value::Integer(b), Value::Integer(a)) => {
                (b, a)
            }
            _ => return Error("Can only perform arithmetic operation on integers"
                .to_string())
        };
        stack.push(Value::Integer(operation(a, b)));
        Continue
    }
}



/// Evaluates a Boolean logic operation on
fn boolean_logic_operation<'a>(
    stack: &mut DataStack,
    operation: fn(bool, bool) -> bool,
) -> ControlAction<'a> {
    if stack.size() < 2 {
        Error ("Not enough items in the stack to perform boolean logic operation"
            .to_string())
    } else {
        let (b, a): (bool, bool) = match (
            stack.pop().unwrap(),
            stack.pop().unwrap()
        ) {
            (Value::Boolean(b), Value::Boolean(a)) => {
                (b, a)
            }
            _ => return Error("Can only perform boolean logic operation on booleans"
                .to_string())
        };
        stack.push(Value::Boolean(operation(a, b)));
        Continue
    }
}



fn comparison_operation<'a>(
    stack: &mut DataStack,
    operation: fn(Value, Value) -> Result<bool, &'static str>,
) -> ControlAction<'a> {
    if stack.size() < 2 {
        Error("Not enough items in the stack to perform comparison operation".to_string())
    } else {
        let (b, a): (Value, Value) = (stack.pop().unwrap(), stack.pop().unwrap());
        stack.push(Value::Boolean (match operation(a, b) {
            Ok (output) => output,
            Err (error) => return Error (error.to_string())
        }));
        Continue
    }
}