mod arg_parser;

use std::env;
use std::process::Command;
use std::{thread, time};
use crate::arg_parser::parser;
use crate::arg_parser::parser::Query;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = parser::parse_args(args).unwrap();

    match command.query {
        Query::RNDebugger => {
            Command::new("killall")
                .arg("React Native Debugger")
                .spawn()
                .expect("failed to execute process");

            thread::sleep(time::Duration::from_millis(1000));

            Command::new("open")
                .arg("-a")
                .arg("React Native Debugger")
                .spawn()
                .expect("failed to execute process");

            return;
        },
        _ => {
            println!("Command not found");
            return;
        }
    }
}

