extern crate progressive;

use std::process::Command;
use std::{thread, time::Duration};

use progressive::progress;

fn main() {
    println!("Scanning...");
    for n in progress(14_000..14_301) {
        freq(n * 1000);
        thread::sleep(Duration::from_millis(500));
    }
}

fn freq(f: u32) {
    Command::new("rigctl")
        .args(["-m", "360"])
        .args(["-r", "/dev/ttyUSB0"])
        .args(["F", &f.to_string()])
        .spawn()
        .expect("failed to execute process");
}
