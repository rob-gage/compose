// Copyright Rob Gage 2025

use clap::{
    Parser,
    Subcommand,
};
use crate::read_evaluate_print_loop;

/// Command line interface for the concatenative language
#[derive(Parser, Debug)]
pub struct Interface {
    #[command(subcommand)]
    pub command: Option<Command>,
}

impl Interface {

    /// Handles the execution of the command represented by this `Interface`
    pub fn handle(&self) {
        match &self.command {
            // start REPL if no subcommand was provided
            None => read_evaluate_print_loop(),
            // build files into library if subcommand `build` was used
            Some (Command::Build { files }) => {

            },
            // build and run files if subcommand `run` was used
            Some (Command::Run { files }) => {

            },
        }
    }

}

/// Top-level commands for the interface
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Run a list of source files
    Run {
        #[arg(required = true)]
        files: Vec<String>,
    },
    /// Build a list of source files into an executable/library
    Build {
        #[arg(required = true)]
        files: Vec<String>,
    },
}