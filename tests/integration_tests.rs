use std::fs;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_toml_config_parsing() {
    let toml_content = r#"
        [bypass]
        target_bwrap = "/usr/bin/bwrap"
        enable_sanitization = true

        [binds]
        read_only = ["/usr/share/fonts"]
        read_write = ["/home/user/documents"]
    "#;

    let config: Result<bwrap_bypass::config::Config, _> = toml::from_str(toml_content);
    assert!(config.is_ok(), "Failed to parse valid bwrap_bypass.toml configuration");

    let cfg = config.unwrap();
    assert_eq!(cfg.bypass.target_bwrap, "/usr/bin/bwrap");
    assert_eq!(cfg.bypass.enable_sanitization, true);
    assert_eq!(cfg.binds.read_only, vec!["/usr/share/fonts".to_string()]);
    assert_eq!(cfg.binds.read_write, vec!["/home/user/documents".to_string()]);
}

#[test]
fn test_config_file_missing_handling() {
    let non_existent_path = "non_existent_bwrap_bypass.toml";
    let result = bwrap_bypass::config::Config::load(non_existent_path);
    assert!(result.is_err(), "Loading a non-existent configuration file should return an error");
}

#[test]
fn test_fake_bwrap_mock_execution() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("bwrap_bypass.toml");
    let mock_bwrap_path = dir.path().join("mock_bwrap");

    // Write a sample TOML configuration in the temp directory
    let toml_content = r#"
        [bypass]
        target_bwrap = "mock_bwrap"
        enable_sanitization = true

        [binds]
        read_only = ["/test/ro"]
        read_write = ["/test/rw"]
    "#;
    fs::write(&config_path, toml_content).unwrap();

    // Create a mock target executable (e.g., a simple shell script or binary that outputs its args)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        // A mock script that writes its arguments to a file or stdout for verification
        let script_content = "#!/bin/sh\necho \"args: $@\" > /tmp/mock_bwrap_output.txt\n";
        fs::write(&mock_bwrap_path, script_content).unwrap();
        let mut perm = fs::metadata(&mock_bwrap_path).unwrap().permissions();
        perm.set_mode(0o755);
        fs::set_permissions(&mock_bwrap_path, perm).unwrap();
    }
}
