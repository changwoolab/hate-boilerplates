use std::process::Command;
use std::{thread, time};

/**
 * Restarts React Native Debugger
 */
pub fn restart() {
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
}
