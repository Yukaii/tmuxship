use assert_cmd::prelude::*;
use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;
use tempfile::tempdir;

fn make_fake_starship(dir: &PathBuf) -> PathBuf {
    let bin_path = dir.join("starship");
    let mut script = fs::File::create(&bin_path).unwrap();
    writeln!(
        script,
        r#"#!/usr/bin/env bash

cat <<JSON
[{{"content":"${{STARSHIP_CONFIG}}","style":"fg:green"}},{{"content":"${{TMUX_SESSION_NAME:-}}","style":"fg:blue"}},{{"content":"${{TMUX_WINDOW_INDEX:-}}"}}]
JSON"#
    )
    .unwrap();
    let mut perms = script.metadata().unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&bin_path, perms).unwrap();
    bin_path
}

fn make_fake_tmux(dir: &PathBuf) -> PathBuf {
    let bin_path = dir.join("tmux");
    let mut script = fs::File::create(&bin_path).unwrap();
    writeln!(
        script,
        r#"#!/usr/bin/env bash

if [[ "$1" == "display-message" ]]; then
  delimiter=$'\x1f'
  printf "tmux-session%s9\n" "$delimiter"
else
  echo "unexpected tmux invocation: $@" >&2
  exit 1
fi
"#
    )
    .unwrap();
    let mut perms = script.metadata().unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&bin_path, perms).unwrap();
    bin_path
}

#[test]
fn cli_renders_using_resolved_config() {
    let dir = tempdir().unwrap();
    let config_root = dir.path().join(".config/starship");
    fs::create_dir_all(&config_root).unwrap();
    let left_config = config_root.join(".left.toml");
    fs::write(&left_config, "[test]\n").unwrap();

    let fake_bin_dir = dir.path().join("bin");
    fs::create_dir_all(&fake_bin_dir).unwrap();
    make_fake_starship(&fake_bin_dir);

    let mut cmd = Command::cargo_bin("tmux-ship").unwrap();
    cmd.arg("left")
        .env("HOME", dir.path())
        .env(
            "PATH",
            format!(
                "{}:{}",
                fake_bin_dir.display(),
                std::env::var("PATH").unwrap()
            ),
        )
        .env("STARSHIP_CACHE", dir.path())
        .env_remove("STARSHIP_CONFIG");

    let output = cmd.assert().success().get_output().stdout.clone();
    let stdout = String::from_utf8(output).unwrap();
    assert!(stdout.contains(left_config.to_string_lossy().as_ref()));
    assert!(stdout.contains("#[fg=green]"));
}

#[test]
fn cli_honors_config_flag() {
    let dir = tempdir().unwrap();
    let cfg = dir.path().join("special.toml");
    fs::write(&cfg, "[test]\n").unwrap();
    let fake_bin_dir = dir.path().join("bin");
    fs::create_dir_all(&fake_bin_dir).unwrap();
    make_fake_starship(&fake_bin_dir);

    let output = Command::cargo_bin("tmux-ship")
        .unwrap()
        .args(["right", "--config", cfg.to_string_lossy().as_ref()])
        .env(
            "PATH",
            format!(
                "{}:{}",
                fake_bin_dir.display(),
                std::env::var("PATH").unwrap()
            ),
        )
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    assert!(stdout.contains(cfg.to_string_lossy().as_ref()));
}

#[test]
fn cli_exports_tmux_vars_to_starship() {
    let dir = tempdir().unwrap();
    let config_root = dir.path().join(".config/starship");
    fs::create_dir_all(&config_root).unwrap();
    fs::write(config_root.join(".full.toml"), "[test]\n").unwrap();

    let fake_bin_dir = dir.path().join("bin");
    fs::create_dir_all(&fake_bin_dir).unwrap();
    make_fake_starship(&fake_bin_dir);
    make_fake_tmux(&fake_bin_dir);

    let output = Command::cargo_bin("tmux-ship")
        .unwrap()
        .arg("full")
        .env("HOME", dir.path())
        .env(
            "PATH",
            format!(
                "{}:{}",
                fake_bin_dir.display(),
                std::env::var("PATH").unwrap()
            ),
        )
        .env("STARSHIP_CACHE", dir.path())
        .env("TMUX_SHIP_TMUX_VARS", "session_name,window_index")
        .env_remove("STARSHIP_CONFIG")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    assert!(stdout.contains("tmux-session"));
    assert!(stdout.contains("9"));
}
