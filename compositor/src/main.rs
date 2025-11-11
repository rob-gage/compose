// Copyright Rob Gage 2025

mod interface;
mod interpreter;

use clap::Parser;
use interface::Interface;
use interpreter::Interpreter;


fn main() {
    let interface: Interface = Interface::parse();
    interface.handle()
}