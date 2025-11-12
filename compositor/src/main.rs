// Copyright Rob Gage 2025

mod interpreter;

use interpreter::Interpreter;

use colored::Colorize;

fn main() {
    let mut interpreter: Interpreter = Interpreter::new();
    println!("\n    {}\n    Compose Interactive Environment\n\n", "Compositor".green().bold());
    interpreter.run();
}