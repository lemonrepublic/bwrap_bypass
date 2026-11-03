use bwrap_bypass::config::Config;
use std::env;
use std::os::unix::process::CommandExt;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut program_args = if args.len() > 1 {
        args[1..].to_vec()
    } else {
        Vec::new()
    };

    // Load configuration from bwrap_bypass.toml if present
    let config_path = env::var("BWRAP_BYPASS_CONFIG").unwrap_or_else(|_| "bwrap_bypass.toml".to_string());
    if let Ok(config) = Config::load(&config_path) {
        // Automatically inject read-only bind mounts
        for bind_path in &config.binds.read_only {
            program_args.push("--ro-bind".to_string());
            program_args.push(bind_path.clone());
            program_args.push(bind_path.clone());
        }

        // Automatically inject read-write bind mounts
        for bind_path in &config.binds.read_write {
            program_args.push("--bind".to_string());
            program_args.push(bind_path.clone());
            program_args.push(bind_path.clone());
        }
    }

    // Sanitize the environment: filter out potentially dangerous or conflicting variables
    let sanitized_env: Vec<(String, String)> = env::vars()
        .filter(|(key, _)| {
            if key.starts_with("LD_") && key != "LD_LIBRARY_PATH" {
                false
            } else {
                true
            }
        })
        .collect();

    // Fallback to default bwrap binary if none specified
    let target_bwrap = env::var("REAL_BWRAP_PATH").unwrap_or_else(|_| "/usr/bin/bwrap".to_string());

    let mut cmd = Command::new(&target_bwrap);
    cmd.args(&program_args);
    cmd.env_clear();
    cmd.envs(sanitized_env);

    let err = cmd.exec();
    eprintln!("Error: failed to execute real bwrap at {}: {}", target_bwrap, err);
    std::process::exit(1);
}
