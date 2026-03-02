use std::process::Command;

use tempfile::TempDir;

#[test]
fn uses_default_profile_from_envlock_home() {
    let temp = TempDir::new().expect("temp dir should be created");
    let envlock_home = temp.path().join("envlock-home");
    let profiles_dir = envlock_home.join("profiles");
    std::fs::create_dir_all(&profiles_dir).expect("profiles dir should be created");

    std::fs::write(
        profiles_dir.join("default.json"),
        r#"{"injections":[{"type":"env","vars":{"ENVLOCK_PROFILE":"from-default"}}]}"#,
    )
    .expect("default profile should be written");

    let output = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args(["--output", "json", "--log-level", "error"])
        .env("ENVLOCK_HOME", &envlock_home)
        .output()
        .expect("envlock command should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be UTF-8");
    assert!(stdout.contains("\"ENVLOCK_PROFILE\": \"from-default\""));
}

#[test]
fn profile_flag_overrides_default_profile() {
    let temp = TempDir::new().expect("temp dir should be created");
    let envlock_home = temp.path().join("envlock-home");
    let profiles_dir = envlock_home.join("profiles");
    std::fs::create_dir_all(&profiles_dir).expect("profiles dir should be created");

    std::fs::write(
        profiles_dir.join("default.json"),
        r#"{"injections":[{"type":"env","vars":{"ENVLOCK_PROFILE":"from-default"}}]}"#,
    )
    .expect("default profile should be written");

    let explicit_profile = temp.path().join("explicit.json");
    std::fs::write(
        &explicit_profile,
        r#"{"injections":[{"type":"env","vars":{"ENVLOCK_PROFILE":"from-profile"}}]}"#,
    )
    .expect("explicit profile should be written");

    let output = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args([
            "-p",
            explicit_profile
                .to_str()
                .expect("explicit path should be UTF-8"),
            "--output",
            "json",
            "--log-level",
            "error",
        ])
        .env("ENVLOCK_HOME", &envlock_home)
        .output()
        .expect("envlock command should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be UTF-8");
    assert!(stdout.contains("\"ENVLOCK_PROFILE\": \"from-profile\""));
    assert!(!stdout.contains("\"ENVLOCK_PROFILE\": \"from-default\""));
}

#[test]
fn fails_when_default_profile_missing() {
    let temp = TempDir::new().expect("temp dir should be created");
    let envlock_home = temp.path().join("envlock-home");

    let output = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args(["--output", "json", "--log-level", "error"])
        .env("ENVLOCK_HOME", &envlock_home)
        .output()
        .expect("envlock command should run");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("stderr should be UTF-8");
    assert!(stderr.contains("profiles/default.json"));
}
