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

if [[ "$1" != "prompt" ]]; then
  echo "unexpected starship invocation: $@" >&2
  exit 1
fi

if [[ "${{CLICOLOR_FORCE:-}}" != "1" ]]; then
  echo "CLICOLOR_FORCE was not set" >&2
  exit 1
fi

printf '\e[32m%s\e[0m\e[34m%s\e[0m%s\n' "${{STARSHIP_CONFIG}}" "${{TMUX_SESSION_NAME:-}}" "${{TMUX_WINDOW_INDEX:-}}"
"#
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

expected_target="${{TMUX_SHIP_EXPECT_TARGET:-}}"

if [[ "$1" == "display-message" ]]; then
  target=""
  shift
  while [[ $# -gt 0 ]]; do
    case "$1" in
      -t)
        target="$2"
        shift 2
        ;;
      *)
        shift
        ;;
    esac
  done

  if [[ -n "$expected_target" && "$target" != "$expected_target" ]]; then
    echo "unexpected target: $target" >&2
    exit 1
  fi

  printf "tmux-session\n9\n"
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

#[test]
fn cli_uses_tmux_target_when_provided() {
    let dir = tempdir().unwrap();
    let config_root = dir.path().join(".config/starship");
    fs::create_dir_all(&config_root).unwrap();
    fs::write(config_root.join(".center.toml"), "[test]\n").unwrap();

    let fake_bin_dir = dir.path().join("bin");
    fs::create_dir_all(&fake_bin_dir).unwrap();
    make_fake_starship(&fake_bin_dir);
    make_fake_tmux(&fake_bin_dir);

    let target = "@42";

    Command::cargo_bin("tmux-ship")
        .unwrap()
        .arg("center")
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
        .env("TMUX_SHIP_TMUX_VARS", "session_name")
        .env("TMUX_SHIP_CENTER_CONFIG", config_root.join(".center.toml"))
        .env("TMUX_SHIP_TARGET", target)
        .env("TMUX_SHIP_EXPECT_TARGET", target)
        .env_remove("STARSHIP_CONFIG")
        .assert()
        .success();
}

#[test]
fn cli_center_side_uses_env_and_tmux_vars() {
    let dir = tempdir().unwrap();
    let config_root = dir.path().join(".config/starship");
    fs::create_dir_all(&config_root).unwrap();
    let center_cfg = config_root.join(".center.toml");
    fs::write(&center_cfg, "[center]\n").unwrap();

    let fake_bin_dir = dir.path().join("bin");
    fs::create_dir_all(&fake_bin_dir).unwrap();
    make_fake_starship(&fake_bin_dir);
    make_fake_tmux(&fake_bin_dir);

    let output = Command::cargo_bin("tmux-ship")
        .unwrap()
        .arg("center")
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
        .env("TMUX_SHIP_TMUX_VARS", "window_index")
        .env("TMUX_SHIP_CENTER_CONFIG", center_cfg.to_string_lossy().as_ref())
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    assert!(stdout.contains(center_cfg.to_string_lossy().as_ref()));
    assert!(stdout.contains("9"));
}
