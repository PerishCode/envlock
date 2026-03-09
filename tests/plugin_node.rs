use std::process::Command;

use tempfile::TempDir;

fn write_fake_tool(path: &std::path::Path, version: &str) {
    std::fs::write(path, format!("#!/usr/bin/env bash\necho {}\n", version))
        .expect("fake tool script should be written");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = std::fs::metadata(path)
            .expect("fake tool metadata should exist")
            .permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(path, permissions).expect("fake tool should be executable");
    }
}

fn write_slow_fake_tool(path: &std::path::Path, version: &str, delay_seconds: u64) {
    std::fs::write(
        path,
        format!(
            "#!/usr/bin/env bash\nsleep {}\necho {}\n",
            delay_seconds, version
        ),
    )
    .expect("slow fake tool script should be written");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = std::fs::metadata(path)
            .expect("slow fake tool metadata should exist")
            .permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(path, permissions).expect("slow fake tool should be executable");
    }
}

fn write_path_bound_manager(path: &std::path::Path, version: &str) {
    std::fs::write(
        path,
        format!(
            "#!/usr/bin/env sh\nnode >/dev/null 2>&1\nprintf '{}\\n'\n",
            version
        ),
    )
    .expect("path-bound manager should be written");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = std::fs::metadata(path)
            .expect("path-bound manager metadata should exist")
            .permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(path, permissions)
            .expect("path-bound manager should be executable");
    }
}

#[test]
fn plugin_node_init_creates_embedded_script() {
    let temp = TempDir::new().expect("temp dir should be created");
    let envlock_home = temp.path().join("envlock-home");

    let output = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args(["plugin", "node", "init"])
        .env("ENVLOCK_HOME", &envlock_home)
        .output()
        .expect("envlock command should run");

    assert!(output.status.success());
    assert!(envlock_home.join("plugins/node.sh").is_file());
}

#[test]
fn plugin_node_preview_and_apply_emit_patch() {
    let temp = TempDir::new().expect("temp dir should be created");
    let envlock_home = temp.path().join("envlock-home");
    let state_dir = temp.path().join("node-state");
    let node_bin = temp.path().join("fake-node.sh");
    let npm_bin = temp.path().join("fake-npm.sh");
    let pnpm_bin = temp.path().join("fake-pnpm.sh");
    let yarn_bin = temp.path().join("fake-yarn.sh");

    write_fake_tool(&node_bin, "v24.12.0");
    write_fake_tool(&npm_bin, "10.9.2");
    write_fake_tool(&pnpm_bin, "10.30.3");
    write_fake_tool(&yarn_bin, "1.22.22");

    let init = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args([
            "plugin",
            "node",
            "init",
            "--state-dir",
            state_dir.to_str().expect("state dir should be UTF-8"),
            "--node-bin",
            node_bin.to_str().expect("node bin path should be UTF-8"),
            "--npm-bin",
            npm_bin.to_str().expect("npm bin path should be UTF-8"),
            "--pnpm-bin",
            pnpm_bin.to_str().expect("pnpm bin path should be UTF-8"),
            "--yarn-bin",
            yarn_bin.to_str().expect("yarn bin path should be UTF-8"),
        ])
        .env("ENVLOCK_HOME", &envlock_home)
        .output()
        .expect("envlock command should run");
    assert!(init.status.success());

    let preview = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args([
            "plugin",
            "node",
            "preview",
            "--state-dir",
            state_dir.to_str().expect("state dir should be UTF-8"),
            "--node-bin",
            node_bin.to_str().expect("node bin path should be UTF-8"),
            "--npm-bin",
            npm_bin.to_str().expect("npm bin path should be UTF-8"),
            "--pnpm-bin",
            pnpm_bin.to_str().expect("pnpm bin path should be UTF-8"),
            "--yarn-bin",
            yarn_bin.to_str().expect("yarn bin path should be UTF-8"),
        ])
        .env("ENVLOCK_HOME", &envlock_home)
        .output()
        .expect("envlock command should run");
    assert!(preview.status.success());

    let stdout = String::from_utf8(preview.stdout).expect("stdout should be UTF-8");
    assert!(stdout.contains("\"schema\": \"envlock.patch.v1\""));
    assert!(stdout.contains("\"ENVLOCK_NODE_BIN\""));
    assert!(stdout.contains("\"YARN_CACHE_FOLDER\""));
    assert!(stdout.contains("global/bin"));

    let apply = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args([
            "plugin",
            "node",
            "apply",
            "--state-dir",
            state_dir.to_str().expect("state dir should be UTF-8"),
            "--node-bin",
            node_bin.to_str().expect("node bin path should be UTF-8"),
            "--npm-bin",
            npm_bin.to_str().expect("npm bin path should be UTF-8"),
            "--pnpm-bin",
            pnpm_bin.to_str().expect("pnpm bin path should be UTF-8"),
            "--yarn-bin",
            yarn_bin.to_str().expect("yarn bin path should be UTF-8"),
        ])
        .env("ENVLOCK_HOME", &envlock_home)
        .output()
        .expect("envlock command should run");
    assert!(apply.status.success());

    let link = state_dir.join("current/bin/node");
    let metadata = std::fs::symlink_metadata(&link).expect("symlink metadata should exist");
    assert!(metadata.file_type().is_symlink());

    assert!(state_dir.join("versions/node/v24.12.0/bin/node").exists());
    assert!(state_dir.join("versions/npm/v10.9.2/bin/npm").exists());
    assert!(state_dir.join("versions/npm/v10.9.2/global/bin").is_dir());
    assert!(state_dir
        .join("versions/npm/v10.9.2/global/lib/node_modules")
        .is_dir());
    assert!(state_dir.join("versions/pnpm/v10.30.3/bin/pnpm").exists());
    assert!(state_dir.join("versions/yarn/v1.22.22/bin/yarn").exists());
    assert!(state_dir.join("versions/yarn/v1.22.22/global").is_dir());
    assert!(state_dir.join("state.v2.json").is_file());
}

#[test]
fn plugin_node_apply_uses_selected_node_for_manager_version_resolution() {
    let temp = TempDir::new().expect("temp dir should be created");
    let envlock_home = temp.path().join("envlock-home");
    let state_dir = temp.path().join("node-state");
    let node_dir = temp.path().join("node-bin-dir");
    let node_bin = node_dir.join("node");
    let npm_bin = temp.path().join("fake-npm.sh");
    let pnpm_bin = temp.path().join("fake-pnpm.sh");
    let yarn_bin = temp.path().join("fake-yarn.sh");

    std::fs::create_dir_all(&node_dir).expect("node dir should be created");
    write_fake_tool(&node_bin, "v18.20.8");
    write_path_bound_manager(&npm_bin, "10.8.2");
    write_path_bound_manager(&pnpm_bin, "10.30.3");
    write_path_bound_manager(&yarn_bin, "1.22.22");

    let init = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args([
            "plugin",
            "node",
            "init",
            "--state-dir",
            state_dir.to_str().expect("state dir should be UTF-8"),
        ])
        .env("ENVLOCK_HOME", &envlock_home)
        .output()
        .expect("init command should run");
    assert!(init.status.success());

    let apply = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args([
            "plugin",
            "node",
            "apply",
            "--state-dir",
            state_dir.to_str().expect("state dir should be UTF-8"),
            "--node-bin",
            node_bin.to_str().expect("node bin path should be UTF-8"),
            "--npm-bin",
            npm_bin.to_str().expect("npm bin path should be UTF-8"),
            "--pnpm-bin",
            pnpm_bin.to_str().expect("pnpm bin path should be UTF-8"),
            "--yarn-bin",
            yarn_bin.to_str().expect("yarn bin path should be UTF-8"),
        ])
        .env("ENVLOCK_HOME", &envlock_home)
        .env("PATH", "/usr/bin:/bin")
        .output()
        .expect("apply command should run");

    assert!(
        apply.status.success(),
        "apply should succeed with explicit node bin, stderr: {}",
        String::from_utf8_lossy(&apply.stderr)
    );
    let state = std::fs::read_to_string(state_dir.join("state.v2.json"))
        .expect("state file should be readable");
    assert!(state.contains("\"version\": \"18.20.8\""));
    assert!(state.contains("\"version\": \"10.8.2\""));
    assert!(state.contains("\"version\": \"10.30.3\""));
    assert!(state.contains("\"version\": \"1.22.22\""));
}

#[test]
fn plugin_node_preview_reports_missing_override_binary() {
    let temp = TempDir::new().expect("temp dir should be created");
    let envlock_home = temp.path().join("envlock-home");
    let state_dir = temp.path().join("node-state");
    let npm_bin = temp.path().join("fake-npm.sh");
    let pnpm_bin = temp.path().join("fake-pnpm.sh");
    let yarn_bin = temp.path().join("fake-yarn.sh");

    write_fake_tool(&npm_bin, "10.9.2");
    write_fake_tool(&pnpm_bin, "10.30.3");
    write_fake_tool(&yarn_bin, "1.22.22");

    let init = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args([
            "plugin",
            "node",
            "init",
            "--state-dir",
            state_dir.to_str().unwrap(),
        ])
        .env("ENVLOCK_HOME", &envlock_home)
        .output()
        .expect("init command should run");
    assert!(init.status.success());

    let missing = temp.path().join("does-not-exist");
    let preview = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args([
            "plugin",
            "node",
            "preview",
            "--state-dir",
            state_dir.to_str().expect("state dir should be UTF-8"),
            "--node-bin",
            missing.to_str().expect("missing path should be UTF-8"),
            "--npm-bin",
            npm_bin.to_str().expect("npm bin path should be UTF-8"),
            "--pnpm-bin",
            pnpm_bin.to_str().expect("pnpm bin path should be UTF-8"),
            "--yarn-bin",
            yarn_bin.to_str().expect("yarn bin path should be UTF-8"),
        ])
        .env("ENVLOCK_HOME", &envlock_home)
        .output()
        .expect("preview command should run");

    assert!(!preview.status.success());
    let stderr = String::from_utf8(preview.stderr).expect("stderr should be UTF-8");
    assert!(
        stderr.contains("configured node binary does not exist or cannot be read"),
        "stderr should contain actionable missing binary error, got: {stderr}"
    );
    assert!(
        !stderr.contains("node binary not found"),
        "stderr should not contain generic PATH fallback error when override is set, got: {stderr}"
    );
}

#[test]
fn plugin_node_preview_rejects_invalid_plugin_json() {
    let temp = TempDir::new().expect("temp dir should be created");
    let envlock_home = temp.path().join("envlock-home");
    let state_dir = temp.path().join("node-state");

    let init = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args([
            "plugin",
            "node",
            "init",
            "--state-dir",
            state_dir.to_str().unwrap(),
        ])
        .env("ENVLOCK_HOME", &envlock_home)
        .output()
        .expect("init command should run");
    assert!(init.status.success());

    let script_path = envlock_home.join("plugins/node.sh");
    std::fs::write(&script_path, "#!/usr/bin/env bash\necho not-json\n")
        .expect("script should be rewritten");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = std::fs::metadata(&script_path)
            .expect("script metadata should exist")
            .permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(&script_path, permissions).expect("script should be executable");
    }

    let preview = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args([
            "plugin",
            "node",
            "preview",
            "--state-dir",
            state_dir.to_str().unwrap(),
        ])
        .env("ENVLOCK_HOME", &envlock_home)
        .output()
        .expect("preview command should run");

    assert!(!preview.status.success());
    let stderr = String::from_utf8(preview.stderr).expect("stderr should be UTF-8");
    assert!(stderr.contains("invalid plugin patch JSON output"));
}

#[test]
#[cfg(unix)]
fn plugin_node_preview_rejects_looped_symlink_override() {
    let temp = TempDir::new().expect("temp dir should be created");
    let envlock_home = temp.path().join("envlock-home");
    let state_dir = temp.path().join("node-state");
    let loop_bin = temp.path().join("node-loop");
    let npm_bin = temp.path().join("fake-npm.sh");
    let pnpm_bin = temp.path().join("fake-pnpm.sh");
    let yarn_bin = temp.path().join("fake-yarn.sh");

    write_fake_tool(&npm_bin, "10.9.2");
    write_fake_tool(&pnpm_bin, "10.30.3");
    write_fake_tool(&yarn_bin, "1.22.22");
    std::os::unix::fs::symlink(&loop_bin, &loop_bin).expect("looped symlink should be created");

    let init = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args([
            "plugin",
            "node",
            "init",
            "--state-dir",
            state_dir.to_str().unwrap(),
        ])
        .env("ENVLOCK_HOME", &envlock_home)
        .output()
        .expect("init command should run");
    assert!(init.status.success());

    let preview = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args([
            "plugin",
            "node",
            "preview",
            "--state-dir",
            state_dir.to_str().expect("state dir should be UTF-8"),
            "--node-bin",
            loop_bin.to_str().expect("loop path should be UTF-8"),
            "--npm-bin",
            npm_bin.to_str().expect("npm bin path should be UTF-8"),
            "--pnpm-bin",
            pnpm_bin.to_str().expect("pnpm bin path should be UTF-8"),
            "--yarn-bin",
            yarn_bin.to_str().expect("yarn bin path should be UTF-8"),
        ])
        .env("ENVLOCK_HOME", &envlock_home)
        .output()
        .expect("preview command should run");

    assert!(!preview.status.success());
    let stderr = String::from_utf8(preview.stderr).expect("stderr should be UTF-8");
    assert!(
        stderr.contains("configured node binary symlink has invalid or looped target"),
        "stderr should contain looped symlink error, got: {stderr}"
    );
    assert!(
        !stderr.contains("node binary not found"),
        "stderr should not contain generic PATH fallback error when override is set, got: {stderr}"
    );
}

#[test]
fn plugin_node_preview_rejects_non_executable_override_without_fallback_message() {
    let temp = TempDir::new().expect("temp dir should be created");
    let envlock_home = temp.path().join("envlock-home");
    let state_dir = temp.path().join("node-state");
    let node_bin = temp.path().join("fake-node.sh");
    let npm_bin = temp.path().join("fake-npm.sh");
    let pnpm_bin = temp.path().join("fake-pnpm.sh");
    let yarn_bin = temp.path().join("fake-yarn.sh");

    std::fs::write(&node_bin, "#!/usr/bin/env bash\necho v24.12.0\n")
        .expect("fake node script should be written");
    write_fake_tool(&npm_bin, "10.9.2");
    write_fake_tool(&pnpm_bin, "10.30.3");
    write_fake_tool(&yarn_bin, "1.22.22");

    let init = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args([
            "plugin",
            "node",
            "init",
            "--state-dir",
            state_dir.to_str().unwrap(),
        ])
        .env("ENVLOCK_HOME", &envlock_home)
        .output()
        .expect("init command should run");
    assert!(init.status.success());

    let preview = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args([
            "plugin",
            "node",
            "preview",
            "--state-dir",
            state_dir.to_str().expect("state dir should be UTF-8"),
            "--node-bin",
            node_bin.to_str().expect("node bin path should be UTF-8"),
            "--npm-bin",
            npm_bin.to_str().expect("npm bin path should be UTF-8"),
            "--pnpm-bin",
            pnpm_bin.to_str().expect("pnpm bin path should be UTF-8"),
            "--yarn-bin",
            yarn_bin.to_str().expect("yarn bin path should be UTF-8"),
        ])
        .env("ENVLOCK_HOME", &envlock_home)
        .output()
        .expect("preview command should run");

    assert!(!preview.status.success());
    let stderr = String::from_utf8(preview.stderr).expect("stderr should be UTF-8");
    assert!(
        stderr.contains("configured node binary is not executable"),
        "stderr should contain non-executable override error, got: {stderr}"
    );
    assert!(
        !stderr.contains("node binary not found"),
        "stderr should not contain generic PATH fallback error when override is set, got: {stderr}"
    );
}

#[test]
fn plugin_node_failure_propagates_plugin_exit_code() {
    let temp = TempDir::new().expect("temp dir should be created");
    let envlock_home = temp.path().join("envlock-home");
    let state_dir = temp.path().join("node-state");

    let init = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args([
            "plugin",
            "node",
            "init",
            "--state-dir",
            state_dir.to_str().unwrap(),
        ])
        .env("ENVLOCK_HOME", &envlock_home)
        .output()
        .expect("init command should run");
    assert!(init.status.success());

    let script_path = envlock_home.join("plugins/node.sh");
    std::fs::write(
        &script_path,
        "#!/usr/bin/env bash\necho node plugin apply is locked by another process >&2\nexit 73\n",
    )
    .expect("script should be rewritten");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = std::fs::metadata(&script_path)
            .expect("script metadata should exist")
            .permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(&script_path, permissions).expect("script should be executable");
    }

    let apply = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args([
            "plugin",
            "node",
            "apply",
            "--state-dir",
            state_dir.to_str().unwrap(),
        ])
        .env("ENVLOCK_HOME", &envlock_home)
        .output()
        .expect("apply command should run");

    assert_eq!(apply.status.code(), Some(73));
    let stderr = String::from_utf8(apply.stderr).expect("stderr should be UTF-8");
    assert!(stderr.contains("exit code 73"));
}

#[test]
#[cfg(unix)]
fn plugin_node_apply_reports_read_only_state_dir_as_permission_error() {
    use std::os::unix::fs::PermissionsExt;

    let temp = TempDir::new().expect("temp dir should be created");
    let envlock_home = temp.path().join("envlock-home");
    let state_dir = temp.path().join("node-state");
    let node_bin = temp.path().join("fake-node.sh");
    let npm_bin = temp.path().join("fake-npm.sh");
    let pnpm_bin = temp.path().join("fake-pnpm.sh");
    let yarn_bin = temp.path().join("fake-yarn.sh");

    write_fake_tool(&node_bin, "v24.12.0");
    write_fake_tool(&npm_bin, "10.9.2");
    write_fake_tool(&pnpm_bin, "10.30.3");
    write_fake_tool(&yarn_bin, "1.22.22");

    let init = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args([
            "plugin",
            "node",
            "init",
            "--state-dir",
            state_dir.to_str().unwrap(),
        ])
        .env("ENVLOCK_HOME", &envlock_home)
        .output()
        .expect("init command should run");
    assert!(init.status.success());

    for path in [
        &state_dir,
        &state_dir.join("current"),
        &state_dir.join("current/bin"),
        &state_dir.join("locks"),
    ] {
        let mut permissions = std::fs::metadata(path)
            .expect("state path metadata should exist")
            .permissions();
        permissions.set_mode(0o555);
        std::fs::set_permissions(path, permissions).expect("state path should become read-only");
    }

    let apply = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args([
            "plugin",
            "node",
            "apply",
            "--state-dir",
            state_dir.to_str().unwrap(),
            "--node-bin",
            node_bin.to_str().unwrap(),
            "--npm-bin",
            npm_bin.to_str().unwrap(),
            "--pnpm-bin",
            pnpm_bin.to_str().unwrap(),
            "--yarn-bin",
            yarn_bin.to_str().unwrap(),
        ])
        .env("ENVLOCK_HOME", &envlock_home)
        .output()
        .expect("apply command should run");

    if apply.status.success() {
        let uid_output = Command::new("id")
            .arg("-u")
            .output()
            .expect("id command should run");
        let uid = String::from_utf8(uid_output.stdout)
            .expect("uid output should be UTF-8")
            .trim()
            .to_owned();
        assert_eq!(uid, "0", "readonly state dir should only succeed as root");
        return;
    }

    assert_eq!(apply.status.code(), Some(74));
    let stderr = String::from_utf8(apply.stderr).expect("stderr should be UTF-8");
    assert!(stderr.contains("failed to create lock directory in state dir"));
    assert!(stderr.contains("Permission denied"));
    assert!(!stderr.contains("locked by another process"));
}

#[test]
fn plugin_node_apply_recovers_from_stale_lock_dir() {
    let temp = TempDir::new().expect("temp dir should be created");
    let envlock_home = temp.path().join("envlock-home");
    let state_dir = temp.path().join("node-state");
    let node_bin = temp.path().join("fake-node.sh");
    let npm_bin = temp.path().join("fake-npm.sh");
    let pnpm_bin = temp.path().join("fake-pnpm.sh");
    let yarn_bin = temp.path().join("fake-yarn.sh");

    write_fake_tool(&node_bin, "v24.12.0");
    write_fake_tool(&npm_bin, "10.9.2");
    write_fake_tool(&pnpm_bin, "10.30.3");
    write_fake_tool(&yarn_bin, "1.22.22");

    let init = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args([
            "plugin",
            "node",
            "init",
            "--state-dir",
            state_dir.to_str().expect("state dir should be UTF-8"),
        ])
        .env("ENVLOCK_HOME", &envlock_home)
        .output()
        .expect("init command should run");
    assert!(init.status.success());

    std::fs::create_dir_all(state_dir.join("locks/apply.lock"))
        .expect("stale lock dir should be created");

    let apply = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args([
            "plugin",
            "node",
            "apply",
            "--state-dir",
            state_dir.to_str().expect("state dir should be UTF-8"),
            "--node-bin",
            node_bin.to_str().expect("node bin path should be UTF-8"),
            "--npm-bin",
            npm_bin.to_str().expect("npm bin path should be UTF-8"),
            "--pnpm-bin",
            pnpm_bin.to_str().expect("pnpm bin path should be UTF-8"),
            "--yarn-bin",
            yarn_bin.to_str().expect("yarn bin path should be UTF-8"),
        ])
        .env("ENVLOCK_HOME", &envlock_home)
        .output()
        .expect("apply command should run");

    assert!(
        apply.status.success(),
        "apply should recover from stale lock dir, stderr: {}",
        String::from_utf8_lossy(&apply.stderr)
    );
    assert!(state_dir.join("state.v2.json").is_file());
    assert!(!state_dir.join("locks/apply.lock").exists());
}

#[test]
fn plugin_node_apply_recovers_after_interrupted_apply() {
    let temp = TempDir::new().expect("temp dir should be created");
    let envlock_home = temp.path().join("envlock-home");
    let state_dir = temp.path().join("node-state");
    let node_bin = temp.path().join("slow-node.sh");
    let npm_bin = temp.path().join("fake-npm.sh");
    let pnpm_bin = temp.path().join("fake-pnpm.sh");
    let yarn_bin = temp.path().join("fake-yarn.sh");

    write_slow_fake_tool(&node_bin, "v24.12.0", 5);
    write_fake_tool(&npm_bin, "10.9.2");
    write_fake_tool(&pnpm_bin, "10.30.3");
    write_fake_tool(&yarn_bin, "1.22.22");

    let init = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args([
            "plugin",
            "node",
            "init",
            "--state-dir",
            state_dir.to_str().expect("state dir should be UTF-8"),
        ])
        .env("ENVLOCK_HOME", &envlock_home)
        .output()
        .expect("init command should run");
    assert!(init.status.success());

    let mut apply = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args([
            "plugin",
            "node",
            "apply",
            "--state-dir",
            state_dir.to_str().expect("state dir should be UTF-8"),
            "--node-bin",
            node_bin.to_str().expect("node bin path should be UTF-8"),
            "--npm-bin",
            npm_bin.to_str().expect("npm bin path should be UTF-8"),
            "--pnpm-bin",
            pnpm_bin.to_str().expect("pnpm bin path should be UTF-8"),
            "--yarn-bin",
            yarn_bin.to_str().expect("yarn bin path should be UTF-8"),
        ])
        .env("ENVLOCK_HOME", &envlock_home)
        .spawn()
        .expect("apply command should spawn");

    let lock_pid_file = state_dir.join("locks/apply.lock/pid");
    let mut observed_lock_pid = None;
    for _ in 0..100 {
        if let Ok(pid) = std::fs::read_to_string(&lock_pid_file) {
            observed_lock_pid = Some(pid.trim().to_owned());
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    let lock_pid = observed_lock_pid.expect("lock pid file should appear");
    let kill_status = Command::new("kill")
        .args(["-9", lock_pid.as_str()])
        .status()
        .expect("kill should run");
    assert!(kill_status.success(), "kill -9 should succeed");

    let interrupted = apply.wait().expect("interrupted apply should exit");
    assert!(!interrupted.success(), "interrupted apply should fail");
    assert!(state_dir.join("locks/apply.lock").exists());
    assert!(!state_dir.join("state.v2.json").exists());

    write_fake_tool(&node_bin, "v24.12.0");

    let retry = Command::new(env!("CARGO_BIN_EXE_envlock"))
        .args([
            "plugin",
            "node",
            "apply",
            "--state-dir",
            state_dir.to_str().expect("state dir should be UTF-8"),
            "--node-bin",
            node_bin.to_str().expect("node bin path should be UTF-8"),
            "--npm-bin",
            npm_bin.to_str().expect("npm bin path should be UTF-8"),
            "--pnpm-bin",
            pnpm_bin.to_str().expect("pnpm bin path should be UTF-8"),
            "--yarn-bin",
            yarn_bin.to_str().expect("yarn bin path should be UTF-8"),
        ])
        .env("ENVLOCK_HOME", &envlock_home)
        .output()
        .expect("retry apply should run");

    assert!(
        retry.status.success(),
        "retry should recover from interrupted apply, stderr: {}",
        String::from_utf8_lossy(&retry.stderr)
    );
    assert!(state_dir.join("state.v2.json").is_file());
    assert!(!state_dir.join("locks/apply.lock").exists());
}
