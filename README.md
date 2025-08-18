# The Compose Language

Compose is a concatenative, stack-based programming language. Programs in Compose are built by writing sequenced functions that modify the stack. Compose is functional programming inspired and makes use of functions as data. Capturing the execution context by treating the stack as data is also possible.

In concatenative languages, the flow of data is entirely determined by the stack. Each function operates on the stack: it takes some number of values from the top, performs a computation, and pushes results back onto the stack. Calling two functions side by side means “apply the first function, then the second, passing along the stack as the argument.”

For example, in a simple concatenative style:

`2 3 +`

This pushes `2` and `3` onto the stack, then applies the `+` word, adding the two numbers and leaving `5` on the stack.

Compose follows this model with a minimal set of primitives, allowing programs to be constructed by composing small, reusable functions. Its design emphasizes consistent stack behavior and predictable composition, keeping the core language simple and easily comprehensible.
