use std::process::Command;

use tempfile::TempDir;

#[test]
fn profiles_init_creates_default_profile() {
    let temp = TempDir::new().expect("temp dir should be created");
    let envlock_home = temp.path().join("envlock-home");

    let output = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args(["profiles", "init", "--type", "minimal"])
        .env("ENVLOCK_HOME", &envlock_home)
        .output()
        .expect("envlock command should run");

    assert!(output.status.success());
    assert!(envlock_home.join("profiles/default.json").is_file());
}

#[test]
fn profiles_status_reports_existing_profiles() {
    let temp = TempDir::new().expect("temp dir should be created");
    let envlock_home = temp.path().join("envlock-home");
    let profiles = envlock_home.join("profiles");
    std::fs::create_dir_all(&profiles).expect("profiles directory should be created");
    std::fs::write(
        profiles.join("default.json"),
        r#"{"injections":[{"type":"env","vars":{"ENVLOCK_PROFILE":"default"}}]}"#,
    )
    .expect("profile should be written");

    let output = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args(["profiles", "status"])
        .env("ENVLOCK_HOME", &envlock_home)
        .output()
        .expect("envlock command should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be UTF-8");
    assert!(stdout.contains("profiles_count: 1"));
    assert!(stdout.contains("default.json [ok]"));
}
