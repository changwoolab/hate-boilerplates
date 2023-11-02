use std::env;
use std::process::Command;
use std::{thread, time};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a command");
        return;
    }

    if args.len() > 2 {
        println!("Please provide only one command");
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "rn-debugger" => {
            Command::new("killall")
                .arg("React Native Debugger")
                .spawn()
                .expect("failed to execute process");

            thread::sleep(time::Duration::from_millis(1500));

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
