// Copyright Rob Gage 2025

use compose_core::{
    Function,
    FunctionStorage,
    Namespace,
    Stack,
    UnresolvedFunction,
};
use pups::*;
use rustyline::{
    Editor,
    error::ReadlineError,
    history::DefaultHistory,
    Config
};
use std::process::exit;

/// An interpreter for the `Compose` language
pub struct Interpreter {
    /// The `Namespace` used by this `Interpreter`
    namespace: Namespace,
    /// The `Stack` used by this `Interpreter`
    stack: Stack,
}

impl Interpreter {

    /// Creates a new `Interpreter`
    pub fn new() -> Self { Self { namespace: Namespace::new(), stack: Stack::new() } }

    /// Runs one iteration of the main `Interpreter` loop
    fn read_evaluate_print(&mut self, editor: &mut Editor<(), DefaultHistory>) {
        match editor.readline("  » ") {
            Ok (input) => {
                let input: &str = input.trim();
                // execute interpreter commands
                match input {
                    // exits the `Interpeter`
                    "!exit" => {
                        println!("Exiting.");
                        exit(0)
                    }
                    // resets the `Interpreter`
                    "!reset" => {
                        println!("Resetting.");
                        self.namespace = Namespace::new();
                        self.stack = Stack::new();
                        return;
                    }
                    _ => {}
                };
                let input: Text = Text::from_string(input);
                // let function_result: ParseResult<UnresolvedFunction>
                //     = UnresolvedFunction::parse.then_ignore(end()).parse(&input);
                // if let ParseResult::Success (unresolved_function, _) = function_result {
                //     // define named functions
                //     match self.namespace.define(&unresolved_function) {
                //         Ok (_) => println!("Defined function: {}", unresolved_function.name()),
                //         Err (missing) => {
                //             println!("Function not defined. Missing required functions:\n");
                //             for name in missing {
                //                 println!("  {}", name);
                //             }
                //         }
                //     };
                //     return;
                // } else
                let function_result: ParseResult<UnresolvedFunction>
                    = UnresolvedFunction::parse_free_terms.trace("Free term parser")
                    .then_ignore(end().trace("End of input parser")).parse(&input);
                if let ParseResult::Success (unresolved_function, _) = function_result {
                    // define free terms as temporary function
                    let function: Function = match self.namespace.define(&unresolved_function) {
                        Ok (terms) => terms,
                        Err (missing) => {
                            println!("Function not defined. Missing required functions:\n");
                            for name in missing {
                                println!("  {}", name);
                            }
                            return;
                        }
                    };
                    // evaluate free terms
                    match function.evaluate(self.namespace.function_storage(), &mut self.stack) {
                        Ok (_) => println!("Print stack"),
                        Err (error) => eprintln!("Error: {}", error)
                    }
                } else {
                    println!("Unrecognized input. This Compose interpreter only accepts functions \
                    to be defined, and free terms to be evaluated");
                }
            }
            Err (ReadlineError::Interrupted) => {
                println!("Use !exit to quit");
            }
            Err (error) => {
                eprintln!("Fatal REPL error: {}", error);
                exit(1);
            }
        }
    }

    /// Runs the `Interpreter`
    pub fn run(&mut self) -> ! {
        // initialize rustyline `Config` and `Editor`
        let config: Config = Config::builder()
            .auto_add_history(true)
            .build();
        let mut editor: Editor<(), DefaultHistory> = Editor::<(), DefaultHistory>::with_config(config)
            .expect("Can initialize REPL");
        // run REPL
        loop {
            self.read_evaluate_print(&mut editor);
        }
    }


}