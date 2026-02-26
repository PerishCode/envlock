use std::process::Command;

use tempfile::TempDir;

fn write_profile(dir: &TempDir) -> String {
    let profile = dir.path().join("cmd-profile.json");
    std::fs::write(
        &profile,
        r#"{
  "injections": [
    {
      "type": "env",
      "vars": {
        "ENVLOCK_PROFILE": "from-command-mode"
      }
    }
  ]
}"#,
    )
    .expect("profile should be written");
    profile
        .to_str()
        .expect("profile path should be UTF-8")
        .to_string()
}

#[test]
fn command_mode_runs_child_with_exported_envs() {
    let temp = TempDir::new().expect("temp dir should be created");
    let profile_path = write_profile(&temp);

    let output = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args([
            "-p",
            &profile_path,
            "--log-level",
            "error",
            "--",
            "bash",
            "-lc",
            "printf '%s' \"$ENVLOCK_PROFILE\"",
        ])
        .output()
        .expect("envlock command should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout should be UTF-8");
    assert_eq!(stdout, "from-command-mode");
}

#[test]
fn command_mode_propagates_child_exit_code() {
    let temp = TempDir::new().expect("temp dir should be created");
    let profile_path = write_profile(&temp);

    let output = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args([
            "-p",
            &profile_path,
            "--log-level",
            "error",
            "--",
            "bash",
            "-lc",
            "exit 17",
        ])
        .output()
        .expect("envlock command should run");

    assert_eq!(output.status.code(), Some(17));
}
