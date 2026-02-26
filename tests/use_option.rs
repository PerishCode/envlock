use std::process::Command;

use tempfile::TempDir;

#[test]
fn use_reads_config_from_envlock_config_home() {
    let temp = TempDir::new().expect("temp dir should be created");
    let config_home = temp.path().join("cfg-home");
    let configs_dir = config_home.join("configs");
    std::fs::create_dir_all(&configs_dir).expect("configs dir should be created");

    let profile_config = configs_dir.join("dev.json");
    std::fs::write(
        &profile_config,
        r#"{"injections":[{"type":"env","vars":{"ENVLOCK_PROFILE":"from-use"}}]}"#,
    )
    .expect("profile config should be written");

    let output = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args(["--use", "dev", "--output", "json", "--log-level", "error"])
        .env("ENVLOCK_CONFIG_HOME", &config_home)
        .output()
        .expect("envlock command should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be UTF-8");
    assert!(stdout.contains("\"ENVLOCK_PROFILE\": \"from-use\""));
}

#[test]
fn config_has_priority_over_use() {
    let temp = TempDir::new().expect("temp dir should be created");
    let config_home = temp.path().join("cfg-home");
    let configs_dir = config_home.join("configs");
    std::fs::create_dir_all(&configs_dir).expect("configs dir should be created");

    let use_config = configs_dir.join("dev.json");
    std::fs::write(
        &use_config,
        r#"{"injections":[{"type":"env","vars":{"ENVLOCK_PROFILE":"from-use"}}]}"#,
    )
    .expect("use config should be written");

    let explicit_config = temp.path().join("explicit.json");
    std::fs::write(
        &explicit_config,
        r#"{"injections":[{"type":"env","vars":{"ENVLOCK_PROFILE":"from-config"}}]}"#,
    )
    .expect("explicit config should be written");

    let output = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args([
            "-c",
            explicit_config
                .to_str()
                .expect("explicit path should be UTF-8"),
            "--use",
            "dev",
            "--output",
            "json",
            "--log-level",
            "error",
        ])
        .env("ENVLOCK_CONFIG_HOME", &config_home)
        .output()
        .expect("envlock command should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be UTF-8");
    assert!(stdout.contains("\"ENVLOCK_PROFILE\": \"from-config\""));
    assert!(!stdout.contains("\"ENVLOCK_PROFILE\": \"from-use\""));
}

#[test]
fn use_defaults_to_home_dot_envlock_when_env_not_set() {
    let temp = TempDir::new().expect("temp dir should be created");
    let home_dir = temp.path().join("home");
    let configs_dir = home_dir.join(".envlock/configs");
    std::fs::create_dir_all(&configs_dir).expect("default configs dir should be created");

    let profile_config = configs_dir.join("default.json");
    std::fs::write(
        &profile_config,
        r#"{"injections":[{"type":"env","vars":{"ENVLOCK_PROFILE":"from-default-home"}}]}"#,
    )
    .expect("profile config should be written");

    let output = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args([
            "--use",
            "default",
            "--output",
            "json",
            "--log-level",
            "error",
        ])
        .env("HOME", &home_dir)
        .env_remove("ENVLOCK_CONFIG_HOME")
        .output()
        .expect("envlock command should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be UTF-8");
    assert!(stdout.contains("\"ENVLOCK_PROFILE\": \"from-default-home\""));
}
