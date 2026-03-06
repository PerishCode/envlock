use std::process::Command;

use tempfile::TempDir;

#[test]
fn alias_append_and_list_work() {
    let temp = TempDir::new().expect("temp dir should be created");
    let envlock_home = temp.path().join("envlock-home");
    std::fs::create_dir_all(envlock_home.join("profiles"))
        .expect("profiles directory should be created");

    let profile = envlock_home.join("profiles/work.json");
    std::fs::write(
        &profile,
        r#"{"injections":[{"type":"env","vars":{"ENVLOCK_PROFILE":"work"}}]}"#,
    )
    .expect("profile should be written");

    let append = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args([
            "alias",
            "append",
            "work",
            "--profile",
            profile.to_str().expect("path should be UTF-8"),
        ])
        .env("ENVLOCK_HOME", &envlock_home)
        .output()
        .expect("envlock command should run");
    assert!(append.status.success());

    let list = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args(["alias", "list"])
        .env("ENVLOCK_HOME", &envlock_home)
        .output()
        .expect("envlock command should run");
    assert!(list.status.success());

    let stdout = String::from_utf8(list.stdout).expect("stdout should be UTF-8");
    assert!(stdout.contains("work ->"));
}

#[test]
fn unknown_command_falls_back_to_alias() {
    let temp = TempDir::new().expect("temp dir should be created");
    let envlock_home = temp.path().join("envlock-home");
    std::fs::create_dir_all(envlock_home.join("profiles"))
        .expect("profiles directory should be created");

    let profile = envlock_home.join("profiles/work.json");
    std::fs::write(
        &profile,
        r#"{"injections":[{"type":"env","vars":{"ENVLOCK_PROFILE":"from-alias"}}]}"#,
    )
    .expect("profile should be written");

    let append = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args([
            "alias",
            "append",
            "work",
            "--profile",
            profile.to_str().expect("path should be UTF-8"),
        ])
        .env("ENVLOCK_HOME", &envlock_home)
        .output()
        .expect("envlock command should run");
    assert!(append.status.success());

    let run = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args(["work"])
        .env("ENVLOCK_HOME", &envlock_home)
        .output()
        .expect("envlock command should run");

    assert!(run.status.success());
    let stdout = String::from_utf8(run.stdout).expect("stdout should be UTF-8");
    assert!(stdout.contains("export ENVLOCK_PROFILE='from-alias'"));
}
