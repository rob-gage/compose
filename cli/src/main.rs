// Copyright Rob Gage 2025

mod interface;
mod read_evaluate_print_loop;

use clap::Parser;
use interface::Interface;
use read_evaluate_print_loop::read_evaluate_print_loop;

fn main() {
    let interface: Interface = Interface::parse();
    interface.handle()
}