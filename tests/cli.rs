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

fn make_fake_tmux_setter(dir: &PathBuf) -> PathBuf {
    let bin_path = dir.join("tmux");
    let mut script = fs::File::create(&bin_path).unwrap();
    writeln!(
        script,
        r#"#!/usr/bin/env bash

if [[ "$1" == "set-option" ]]; then
  printf '%s\n' "$@" >> "${{TMUX_SHIP_APPLY_LOG}}"
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

fn write_generator_configs(root: &PathBuf) {
    fs::create_dir_all(root).unwrap();
    fs::write(
        root.join("starship.toml"),
        r##"
[custom.prefix_active]
style = "bg:#95E6CB bold"

[custom.session_normal]
style = "fg:#565B66"
"##,
    )
    .unwrap();
    fs::write(
        root.join(".right.toml"),
        r##"
[custom.battery]
style = "fg:#FFFFFF"
"##,
    )
    .unwrap();
    fs::write(
        root.join(".center.toml"),
        r##"
[custom.window_active]
style = "fg:#BFBDB6 bold"

[custom.window_inactive]
style = "fg:#475266"

[custom.window_zoom]
command = "printf \"\uf293 \""
style = "fg:#39BAE6"
"##,
    )
    .unwrap();
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

    let mut cmd = Command::cargo_bin("tmuxship").unwrap();
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
        .env_remove("TMUX_SHIP_LEFT_CONFIG")
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

    let output = Command::cargo_bin("tmuxship")
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

    let output = Command::cargo_bin("tmuxship")
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
        .env_remove("TMUX_SHIP_FULL_CONFIG")
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

    Command::cargo_bin("tmuxship")
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

    let output = Command::cargo_bin("tmuxship")
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
        .env(
            "TMUX_SHIP_CENTER_CONFIG",
            center_cfg.to_string_lossy().as_ref(),
        )
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    assert!(stdout.contains(center_cfg.to_string_lossy().as_ref()));
    assert!(stdout.contains("9"));
}

#[test]
fn cli_emits_tmux_conf_from_starship_styles() {
    let dir = tempdir().unwrap();
    let config_root = dir.path().join(".config/tmux");
    write_generator_configs(&config_root);

    let output = Command::cargo_bin("tmuxship")
        .unwrap()
        .arg("emit-tmux-conf")
        .env("HOME", dir.path())
        .env_remove("TMUX_SHIP_LEFT_CONFIG")
        .env_remove("TMUX_SHIP_RIGHT_CONFIG")
        .env_remove("TMUX_SHIP_CENTER_CONFIG")
        .env_remove("STARSHIP_CONFIG")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();
    assert!(stdout.contains("set -g status-left '#{?client_prefix,#[bg=#95E6CB,bold]#S #[default],#[fg=#565B66]#S #[default]}'"));
    assert!(stdout.contains("set -g status-right '#(tmuxship right)'"));
    assert!(stdout.contains("set -g window-status-separator '#[fg=#475266] • #[default]'"));
    assert!(stdout.contains("set -g window-status-format '#[fg=#475266]###I #W #[default]'"));
    assert!(stdout.contains(&format!("set -g window-status-current-format '#[fg=#BFBDB6,bold]###I #W #{{?window_zoomed_flag,#[fg=#39BAE6]{} #[default],}}#[default]'", "\u{f293}")));
}

#[test]
fn cli_apply_sets_generated_tmux_options() {
    let dir = tempdir().unwrap();
    let config_root = dir.path().join(".config/tmux");
    write_generator_configs(&config_root);

    let fake_bin_dir = dir.path().join("bin");
    fs::create_dir_all(&fake_bin_dir).unwrap();
    make_fake_tmux_setter(&fake_bin_dir);
    let log_path = dir.path().join("apply.log");

    Command::cargo_bin("tmuxship")
        .unwrap()
        .arg("apply")
        .env("HOME", dir.path())
        .env("TMUX_SHIP_APPLY_LOG", &log_path)
        .env(
            "PATH",
            format!(
                "{}:{}",
                fake_bin_dir.display(),
                std::env::var("PATH").unwrap()
            ),
        )
        .env_remove("TMUX_SHIP_LEFT_CONFIG")
        .env_remove("TMUX_SHIP_RIGHT_CONFIG")
        .env_remove("TMUX_SHIP_CENTER_CONFIG")
        .env_remove("STARSHIP_CONFIG")
        .assert()
        .success();

    let log = fs::read_to_string(log_path).unwrap();
    assert!(log.contains("status-left"));
    assert!(log
        .contains("#{?client_prefix,#[bg=#95E6CB,bold]#S #[default],#[fg=#565B66]#S #[default]}"));
    assert!(log.contains("status-right"));
    assert!(log.contains("window-status-current-format"));
}
