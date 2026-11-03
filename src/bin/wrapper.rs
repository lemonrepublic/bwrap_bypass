use std::env;
use std::os::unix::process::CommandExt;
use std::process::Command;

fn main() {
    let lib_path = "/usr/local/lib/libbwrap_bypass.so";
    let args: Vec<String> = env::args().skip(1).collect();

    let err = Command::new("flatpak")
        .args(&args)
        .env("LD_PRELOAD", lib_path)
        .exec();

    eprintln!("Failed to launch flatpak wrapper: {}", err);
}
