// Copyright Rob Gage 2025

mod interpreter;

use interpreter::Interpreter;


fn main() {
    let mut interpreter: Interpreter = Interpreter::new();
    println!("\n  Compositor\n  Compose Interactive Environment\n");
    interpreter.run();
}