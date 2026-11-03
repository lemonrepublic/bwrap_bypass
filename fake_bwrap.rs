use std::env;
use std::os::unix::process::CommandExt;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let real_bwrap = /usr/bin/bwrap;

    let err = Command::new(real_bwrap)
        .args(&args)
        .exec();

    eprintln!(Failed to execute real bwrap: {}, err);
} 
