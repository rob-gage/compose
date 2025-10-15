// Copyright Rob Gage 2025

use pups::*;

macro_rules! combinators {
    (
        $(
            $(#[$meta:meta])*
            $variant:ident :: $token:expr
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
    };
}

combinators! {

    /// # Arithmetic Combinators

    /// ## Add
    ///
    /// `a b -> (a + b)`
    ///
    /// Adds the two numbers on top of the stack
    Add :: "+",

    /// ## Divide
    ///
    /// `a b -> (a / b)`
    ///
    /// Divides the second number on top of the stack by the number on top of the stack
    Divide :: "/",

    /// ## Modulo
    ///
    /// `a b -> (a % b)`
    ///
    /// Evaluates to the remainder of the second number on top of the stack divided by the first
    /// number on top of the stack
    Modulo :: "%",

    /// ## Multiply
    ///
    /// `a b -> (a * b)`
    ///
    /// Multiplies the two numbers on top of the stack
    Multiply :: "*",

    /// ## Subtract
    ///
    /// `a b -> (a - b)`
    ///
    /// Subtracts the number on top of the stack from the second number on top of the stack
    Subtract :: "-",

    /// # Boolean Logic Combinators

    /// ## And
    ///
    /// `a b -> (a & b)`
    ///
    /// Transforms the two booleans on top of the stack to one with a true value if they are both
    /// true, otherwise transforms them into a boolean with a false value.
    And :: "&",

    /// ## Exclusive Or
    ///
    /// `a b -> (a ^ b)`
    ///
    /// Transforms the two booleans on top of the stack to one with a true value if only one is
    /// true, otherwise transforms them into a boolean with a false value.
    ExclusiveOr :: "^",

    /// ## Not
    ///
    /// `a -> !a`
    ///
    /// Transforms a boolean on top of the stack to true if it is false, and false if it is true.
    Not :: "!",

    /// ## Or
    ///
    /// `a b -> (a | b)`
    ///
    /// Transforms the two booleans on top of the stack to one with a true value if either one is
    /// true, otherwise transforms them into a boolean with a false value.
    Or :: "|",

    /// ## Comparison Combinators

    /// ## Equality
    ///
    /// `a b -> (a = b)`
    ///
    /// Evaluates to a true boolean value if the top two items on the stack are equal, otherwise
    /// evaluates to a false boolean value.
    Equality :: "=",

    /// ## Greater Than
    ///
    /// `a b -> (a > b)`
    ///
    /// Evaluates to a true boolean value if the integer on top of the stack is less than the
    /// one below it.
    GreaterThan :: ">",

    /// ## Less Than
    ///
    /// `a b -> (a < b)`
    ///
    /// Evaluates to a true boolean value if the integer on top of the stack is greater than the
    /// one below it.
    LessThan :: "<",

    /// # Functional Combinators

    /// ## Apply
    ///
    /// `a |f| -> a ...`
    ///
    /// Applies the function on top of the stack
    Apply :: "apply",

    /// ## If
    ///
    /// `a |f| |g| b -> a ...`
    ///
    /// Applies function `|f|` (third from top of stack) to term `a` (fourth from top of stack) if
    /// boolean `b` (top of stack) is a true, otherwise applies function `|g|` (second from tbe top
    /// of stack) to term `a` and the stack below
    If :: "if",

    /// ## Compose
    ///
    /// `|f| |g| -> |f g|`
    ///
    /// Composes a function from two functions on top of the stack
    Compose :: "compose",

    /// ## Under
    ///
    /// `a b |f| -> a ... b`
    ///
    /// Applies the function on top of the stack to the second value from the top of the stack
    Under :: "under",

    /// # List Processing Combinators

    /// ## Construct
    ///
    /// `a [x] -> [y]`
    ///
    /// Prepends an element `a` (second from top of the stack) to the list `[x]` (top of the
    /// stack)
    Construct :: "construct",

    /// ## Count
    ///
    /// `[x] -> a`
    ///
    /// Turns a list on top of the stack into its size
    Count :: "count",

    /// ## Filter
    ///
    /// `[x] |f| -> [y]`
    ///
    /// Filters the list `[x]` (second from top of the stack), keeping only the items
    /// that match a predicate function `|f|` (top of the stack)
    Filter :: "filter",

    /// ## Fold
    ///
    /// `[x] a |f| -> a`
    ///
    /// Reduces the list `[x]` (third from top of the stack) into a single accumulated
    /// value by applying the function `|f|` (on top of the stack) to the accumulator `a` (second
    /// from top of the stack) with each element of the list.
    Fold :: "fold",

    /// ## Head
    ///
    /// `[x] -> a`
    ///
    /// Returns the first element in the list on top of the stack
    Head :: "head",

    /// ## Join
    ///
    /// `[x] [y] -> [x y]`
    ///
    /// Joins the two lists on top of the stack into one
    Join :: "join",

    /// ## Map
    ///
    /// `[x] |f| -> [y]`
    ///
    /// Applies the function `|f|` (top of the stack) to every item in the list `[x]` (second
    /// from top of the stack) creating a new list
    Map :: "map",

    /// ## Tail
    ///
    /// `[x] -> a`
    ///
    /// Returns everything but the first element in the list on top of the stack
    Tail :: "tail",

    /// # Stack Manipulation Combinators

    /// ## Copy
    ///
    /// `a -> a a`
    ///
    /// Duplicates the item on top of the stack
    Copy :: "copy",

    /// ## Drop
    ///
    /// `a b -> a`
    ///
    /// Removes the item on top of the stack
    Drop :: "drop",

    /// ## Hop
    ///
    /// `a b -> a b a`
    ///
    /// Pushes a duplicate of the second value from top of the stack to top of the stack
    Hop :: "hop",

    /// ## Rotate
    ///
    /// `a b c -> c a b`
    ///
    /// Moves the top item on the stack to the position below the next top two items
    Rotate :: "rotate",

    /// ## Swap
    ///
    /// `a b -> b a`
    ///
    /// Swaps the two items on top of the stack
    Swap :: "swap"

}