// Copyright Rob Gage 2025

use crate::{
    Data,
    FunctionReference,
    Integer,
    Term,
    VirtualMachine
};
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


    /// ## Modulo
    ///
    /// `a b -> (a % b)`
    ///
    /// Evaluates to the remainder of the second number on top of the stack divided by the first
    /// number on top of the stack
    Modulo
    ; "%",

    /// ## Multiply
    ///
    /// `a b -> (a * b)`
    ///
    /// Multiplies the two numbers on top of the stack
    Multiply
    ; "*",

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
    ; "<"
    
,

    /// # Functional Combinators

    /// ## Apply
    ///
    /// `a |f| -> a ...`
    ///
    /// Applies the function on top of the stack
    Apply
    ; "apply",

    /// ## If
    ///
    /// `a |f| |g| b -> a ...`
    ///
    /// Applies function `|f|` (third from top of stack) to term `a` (fourth from top of stack) if
    /// boolean `b` (top of stack) is a true, otherwise applies function `|g|` (second from tbe top
    /// of stack) to term `a` and the stack below
    If
    ; "if",

    /// ## Compose
    ///
    /// `|f| |g| -> |f g|`
    ///
    /// Composes a function from two functions on top of the stack
    Compose
    ; "compose",

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
    Construct
    ; "construct",

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
    /// Pushes a duplicate of the second value from top of the stack to top of the stack
    Hop
    ; "hop",

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

            Modulo => arithmetic_operation(stack, |a, b| a % b),

            Multiply => arithmetic_operation(stack, |a, b| a * b),

            Subtract => arithmetic_operation(stack, |a, b| a - b),

            // boolean combinators

            And => boolean_logic_operation(stack, |a, b| a && b),

            ExclusiveOr => boolean_logic_operation(stack, |a, b| a ^ b),

            Not => if let Some(top) = stack.pop() {
                if let Data::Boolean(boolean) = top {
                    stack.push(Data::Boolean(!boolean));
                    Continue
                } else {
                    Error ("Can only perform boolean \"not\" operation on boolean data".to_string())
                }
            } else {
                Error ("Cannot perform boolean \"not\" operation on empty stack".to_string())
            },

            Or => boolean_logic_operation(stack, |a, b| a || b),

            // comparison combinators

            // Equality => comparison_operation(stack, |a, b| Ok(a == b)),

            // GreaterThan => comparison_operation(stack, |a, b| match (a, b) {
            //     (Data::Integer(a), Data::Integer(b)) => Ok(a > b),
            //     _ => Err("Can only perform \"greater than\" operation on integers")
            // }),

            // LessThan => comparison_operation(stack, |a, b| match (a, b) {
            //     (Data::Integer(a), Data::Integer(b)) => Ok(a < b),
            //     _ => Err("Can only perform \"less than\" operation on integers")
            // }),

            // functional combinators

            Apply => {
                let top: Option<Data> = stack.pop();
                if let Some (Data::Lambda (reference)) = top {
                    let lambda: Function = reference.get(environment);
                    Push (lambda)
                } else { Error ("Stack must have a lambda on top to be applied".to_string()) }
            },

            Compose => match (stack.pop(), stack.pop()) {
                (Some(Data::Lambda (a_reference)), Some(Data::Lambda(b_reference))) => {
                    stack.push(Data::Lambda (a_reference.compose(b_reference)));
                    Continue
                }
                _ => Error ("Cannot perform `compose` operation unless there are two lambdas on \
                top of the stack".to_string()),
            }

            If => match (stack.pop(), stack.pop()) {
                (Some(Data::Lambda (false_reference)), Some(Data::Lambda(true_reference))) =>
                    match stack.pop() {
                        Some(Data::Boolean (boolean)) => if boolean {
                            let true_lambda: Function = true_reference.get(environment);
                            Push (true_lambda)
                        } else {
                            let false_lambda: Function = false_reference.get(environment);
                            Push (false_lambda)
                        }
                    _ => Error ("Cannot perform `if` operation unless there is a boolean below \
                        the two lambdas on top of the stack".to_string())
                },
                _ => Error ("Cannot perform `if` operation unless there are two lambdas on top of \
                the stack".to_string()),
            }

            Under => match (stack.pop(), stack.pop()) {
                (Some(Data::Lambda (reference)), Some(top)) => {
                    let lambda: Function = reference.get(environment)
                        .extended(&[Term::Data (top)]);
                    Push (lambda)
                }
                _ => Error ("Cannot perform `under` operation unless there is a lambda under \
                another item on top of the stack".to_string()),
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
            (Data::Integer(b), Data::Integer(a)) => {
                (b, a)
            }
            _ => return Error("Can only perform arithmetic operation on integers"
                .to_string())
        };
        stack.push(Data::Integer(operation(a, b)));
        Continue
    }
}



/// Evaluates a Boolean logic operation on
fn boolean_logic_operation<'a>(
    stack: &mut DataStack,
    operation: fn(bool, bool) -> bool,
) -> ControlAction<'a> {
    if stack.size() < 2 {
        ControlAction::Error ("Not enough items in the stack to perform boolean logic operation"
            .to_string())
    } else {
        let (b, a): (bool, bool) = match (
            stack.pop().unwrap(),
            stack.pop().unwrap()
        ) {
            (Data::Boolean(b), Data::Boolean(a)) => {
                (b, a)
            }
            _ => return ControlAction::Error("Can only perform boolean logic operation on booleans"
                .to_string())
        };
        stack.push(Data::Boolean(operation(a, b)));
        ControlAction::Continue
    }
}



fn comparison_operation<'a>(
    stack: &mut DataStack,
    operation: fn(Data, Data) -> Result<bool, &'static str>,
) -> ControlAction {
    if stack.size() < 2 {
        Error("Not enough items in the stack to perform comparison operation".to_string())
    } else {
        let (b, a): (Data, Data) = (stack.pop().unwrap(), stack.pop().unwrap());
        stack.push(Data::Boolean(match operation(a, b) {
            Ok (output) => output,
            Err (error) => return Error (error.to_string())
        }));
        Continue
    }
}