use std::process::Command;

#[test]
fn logs_go_to_stderr_and_exports_stay_on_stdout() {
    let output = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args(["-p", "examples/envlock.sample.json", "--log-level", "info"])
        .env_remove("RUST_LOG")
        .output()
        .expect("envlock command should run");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be valid UTF-8");
    let stderr = String::from_utf8(output.stderr).expect("stderr should be valid UTF-8");

    assert!(stdout.contains("export ENVLOCK_PROFILE='dev'"));
    assert!(stderr.contains("envlock run started"));
    assert!(!stdout.contains("envlock run started"));
}
