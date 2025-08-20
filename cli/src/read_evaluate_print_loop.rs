// Copyright Rob Gage 2025

use compose_vm::{
    Function,
    Namespace,
    Stack,
    UnresolvedFunction,
};
use rustyline::{
    Editor,
    error::ReadlineError,
    history::DefaultHistory,
    Config
};
use std::process::exit;

/// Runs the read-evaluate-print loop
pub fn read_evaluate_print_loop() -> ! {
    // initialize rustyline `Config` and `Editor`
    let config: Config = Config::builder()
        .auto_add_history(true)
        .build();
    let mut editor: Editor<(), DefaultHistory> = Editor::<(), DefaultHistory>::with_config(config)
        .expect("Can initialize REPL");
    // initialize namespace and stack
    let mut namespace: Namespace = Namespace::new();
    let mut stack: Stack = Stack::new();
    // run REPL
    loop {
        match editor.readline("  » ") {
            Ok (input) => {
                // check if input is a REPL command
                if repl_command(&input, &mut namespace, &mut stack) { continue }
                // check if input is free terms
                if free_terms(&input, &mut namespace, &mut stack) { continue }
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
}

/// Handle REPL input if it is a REPL command
fn repl_command(
    input: &str,
    namespace: &mut Namespace,
    stack: &mut Stack
) -> bool {
    let input: &str = input.trim();
    if input.starts_with('!') {
        let input: &str = &input[1..];
        match input {
            // Exits the REPL
            "exit" => {
                println!("Exiting");
                exit(0)
            },
            // Clears the REPL stack
            "clear" => {
                *stack = Stack::new();
                println!("Cleared stack");
            },
            _ => return false,
        }
        true
    } else { false }
}

/// Handle REPL input if it is free terms
fn free_terms(
    input: &str,
    namespace: &mut Namespace,
    stack: &mut Stack
) -> bool {
    match UnresolvedFunction::anonymous_function(input) {
        Ok (unresolved_function) => {
            match namespace.define(unresolved_function) {
                Ok(function) => {
                    if let Err (error) = function.evaluate(stack) {
                        eprintln!("Evaluation error: {}", error)
                    } else {
                        println!("\n    {}\n", stack.display_stack());
                    }
                },
                Err(error) => eprintln!("Required functions are not defined: {:?}", error)
            }
            true
        },
        Err (_) => false,
    }
}