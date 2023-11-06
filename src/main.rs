mod arg_parser;
mod rn_debugger;

use std::env;
use crate::arg_parser::parser;
use crate::arg_parser::parser::Query;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = parser::parse_args(args).unwrap();

    match command.query {
        Query::RNDebugger => {
            rn_debugger::restart();
        },
        _ => {
            println!("Command not found");
            return;
        }
    }
}

