use std::env;
use std::os::unix::process::CommandExt;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut exec_idx = None;

    for (i, arg) in args.iter().enumerate().skip(1) {
        if arg == "--" {
            exec_idx = Some(i + 1);
            break;
        }
    }

    let target_args = if let Some(idx) = exec_idx {
        if idx < args.len() {
            &args[idx..]
        } else {
            &args[args.len() - 1..]
        }
    } else {
        &args[args.len() - 1..]
    };

    if !target_args.is_empty() {
        let err = Command::new(&target_args[0])
            .args(&target_args[1..])
            .exec();
        eprintln!("Failed to execute: {}", err);
    }
}
