use std::process::Command;

use tempfile::TempDir;

#[test]
fn use_reads_profile_from_envlock_profile_home() {
    let temp = TempDir::new().expect("temp dir should be created");
    let profile_home = temp.path().join("profile-home");
    let profiles_dir = profile_home.join("profiles");
    std::fs::create_dir_all(&profiles_dir).expect("profiles dir should be created");

    let profile_file = profiles_dir.join("dev.json");
    std::fs::write(
        &profile_file,
        r#"{"injections":[{"type":"env","vars":{"ENVLOCK_PROFILE":"from-use"}}]}"#,
    )
    .expect("profile file should be written");

    let output = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args(["--use", "dev", "--output", "json", "--log-level", "error"])
        .env("ENVLOCK_PROFILE_HOME", &profile_home)
        .output()
        .expect("envlock command should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be UTF-8");
    assert!(stdout.contains("\"ENVLOCK_PROFILE\": \"from-use\""));
}

#[test]
fn profile_has_priority_over_use() {
    let temp = TempDir::new().expect("temp dir should be created");
    let profile_home = temp.path().join("profile-home");
    let profiles_dir = profile_home.join("profiles");
    std::fs::create_dir_all(&profiles_dir).expect("profiles dir should be created");

    let use_profile = profiles_dir.join("dev.json");
    std::fs::write(
        &use_profile,
        r#"{"injections":[{"type":"env","vars":{"ENVLOCK_PROFILE":"from-use"}}]}"#,
    )
    .expect("use profile should be written");

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
            "--use",
            "dev",
            "--output",
            "json",
            "--log-level",
            "error",
        ])
        .env("ENVLOCK_PROFILE_HOME", &profile_home)
        .output()
        .expect("envlock command should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be UTF-8");
    assert!(stdout.contains("\"ENVLOCK_PROFILE\": \"from-profile\""));
    assert!(!stdout.contains("\"ENVLOCK_PROFILE\": \"from-use\""));
}

#[test]
fn use_defaults_to_home_dot_envlock_profiles_when_env_not_set() {
    let temp = TempDir::new().expect("temp dir should be created");
    let home_dir = temp.path().join("home");
    let profiles_dir = home_dir.join(".envlock/profiles");
    std::fs::create_dir_all(&profiles_dir).expect("default profiles dir should be created");

    let profile_file = profiles_dir.join("default.json");
    std::fs::write(
        &profile_file,
        r#"{"injections":[{"type":"env","vars":{"ENVLOCK_PROFILE":"from-default-home"}}]}"#,
    )
    .expect("profile file should be written");

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
        .env_remove("ENVLOCK_PROFILE_HOME")
        .output()
        .expect("envlock command should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be UTF-8");
    assert!(stdout.contains("\"ENVLOCK_PROFILE\": \"from-default-home\""));
}
