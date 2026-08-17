use std::collections::HashMap;
use std::fs;
use std::path::Path;

use tmuxship::config::{resolve_config, Side};

fn env_with_home(base: &Path) -> HashMap<String, String> {
    let mut env = HashMap::new();
    env.insert("HOME".into(), base.to_string_lossy().into());
    env
}

#[test]
fn override_path_is_used() {
    let dir = tempfile::tempdir().unwrap();
    let cfg = dir.path().join("custom.toml");
    fs::write(&cfg, "[test]\n").unwrap();
    let env = env_with_home(dir.path());

    let resolved = resolve_config(Side::Left, Some(cfg.clone()), &env).unwrap();
    assert_eq!(resolved.config_path, cfg);
    assert_eq!(resolved.source, "override");
}

#[test]
fn env_var_has_priority() {
    let dir = tempfile::tempdir().unwrap();
    let cfg = dir.path().join("env.toml");
    fs::write(&cfg, "[test]\n").unwrap();
    let mut env = env_with_home(dir.path());
    env.insert("TMUX_SHIP_LEFT_CONFIG".into(), cfg.to_string_lossy().into());

    let resolved = resolve_config(Side::Left, None, &env).unwrap();
    assert_eq!(resolved.config_path, cfg);
    assert_eq!(resolved.source, "TMUX_SHIP_LEFT_CONFIG");
}

#[test]
fn starship_config_fallback() {
    let dir = tempfile::tempdir().unwrap();
    let cfg = dir.path().join("global.toml");
    fs::write(&cfg, "[test]\n").unwrap();
    let mut env = env_with_home(dir.path());
    env.insert("STARSHIP_CONFIG".into(), cfg.to_string_lossy().into());

    let resolved = resolve_config(Side::Right, None, &env).unwrap();
    assert_eq!(resolved.config_path, cfg);
    assert_eq!(resolved.source, "STARSHIP_CONFIG");
}

#[test]
fn side_specific_default_then_global() {
    let dir = tempfile::tempdir().unwrap();
    let config_root = dir.path().join(".config/starship");
    fs::create_dir_all(&config_root).unwrap();
    let left = config_root.join(".left.toml");
    let global = config_root.join("starship.toml");
    fs::write(&left, "[left]\n").unwrap();
    fs::write(&global, "[global]\n").unwrap();

    let env = env_with_home(dir.path());
    let resolved_left = resolve_config(Side::Left, None, &env).unwrap();
    assert_eq!(resolved_left.config_path, left);
    assert_eq!(resolved_left.source, "default-side");

    let resolved_right = resolve_config(Side::Right, None, &env).unwrap();
    assert_eq!(resolved_right.config_path, global);
    assert_eq!(resolved_right.source, "default-global");
}

#[test]
fn xdg_config_home_is_used() {
    let dir = tempfile::tempdir().unwrap();
    let xdg_root = dir.path().join("xdg/starship");
    fs::create_dir_all(&xdg_root).unwrap();
    let right = xdg_root.join(".right.toml");
    fs::write(&right, "[right]\n").unwrap();

    let mut env = env_with_home(dir.path());
    env.insert(
        "XDG_CONFIG_HOME".into(),
        dir.path().join("xdg").to_string_lossy().into(),
    );

    let resolved = resolve_config(Side::Right, None, &env).unwrap();
    assert_eq!(resolved.config_path, right);
    assert_eq!(resolved.source, "default-side");
}

#[test]
fn tmux_config_dir_is_used() {
    let dir = tempfile::tempdir().unwrap();
    let tmux_root = dir.path().join(".config/tmux");
    fs::create_dir_all(&tmux_root).unwrap();
    let cfg = tmux_root.join("starship.toml");
    fs::write(&cfg, "[tmux]\n").unwrap();

    let env = env_with_home(dir.path());
    let resolved = resolve_config(Side::Left, None, &env).unwrap();
    assert_eq!(resolved.config_path, cfg);
    assert_eq!(resolved.source, "tmux-global");
}

#[test]
fn tmux_config_takes_priority_over_starship_dir() {
    let dir = tempfile::tempdir().unwrap();
    let starship_root = dir.path().join(".config/starship");
    let tmux_root = dir.path().join(".config/tmux");
    fs::create_dir_all(&starship_root).unwrap();
    fs::create_dir_all(&tmux_root).unwrap();
    let starship_cfg = starship_root.join(".left.toml");
    let tmux_cfg = tmux_root.join(".left.toml");
    fs::write(&starship_cfg, "[left]\n").unwrap();
    fs::write(&tmux_cfg, "[tmux-left]\n").unwrap();

    let env = env_with_home(dir.path());
    let resolved = resolve_config(Side::Left, None, &env).unwrap();
    assert_eq!(resolved.config_path, tmux_cfg);
    assert_eq!(resolved.source, "tmux-side");
}

#[test]
fn center_side_uses_side_specific_file() {
    let dir = tempfile::tempdir().unwrap();
    let config_root = dir.path().join(".config/starship");
    fs::create_dir_all(&config_root).unwrap();
    let center = config_root.join(".center.toml");
    fs::write(&center, "[center]\n").unwrap();

    let env = env_with_home(dir.path());
    let resolved = resolve_config(Side::Center, None, &env).unwrap();
    assert_eq!(resolved.config_path, center);
    assert_eq!(resolved.source, "default-side");
}

#[test]
fn unified_tmuxship_toml_is_discovered_and_extracted() {
    let dir = tempfile::tempdir().unwrap();
    let tmux_root = dir.path().join(".config/tmux");
    fs::create_dir_all(&tmux_root).unwrap();
    let unified = tmux_root.join("tmuxship.toml");
    fs::write(
        &unified,
        r#"
[left]
format = "$custom"
[left.custom.session]
command = "printf 'left'"

[center]
format = "$custom"
[center.custom.window]
command = "printf 'center'"
"#,
    )
    .unwrap();

    let env = env_with_home(dir.path());
    let resolved_left = resolve_config(Side::Left, None, &env).unwrap();
    assert_eq!(resolved_left.source, "tmux-unified");
    assert!(resolved_left.config_path.is_file());
    let left_content = fs::read_to_string(resolved_left.config_path).unwrap();
    assert!(left_content.contains("custom.session"));

    let resolved_center = resolve_config(Side::Center, None, &env).unwrap();
    assert_eq!(resolved_center.source, "tmux-unified");
    let center_content = fs::read_to_string(resolved_center.config_path).unwrap();
    assert!(center_content.contains("custom.window"));
}

#[test]
fn tmux_ship_config_env_var_works() {
    let dir = tempfile::tempdir().unwrap();
    let cfg = dir.path().join("my-unified.toml");
    fs::write(
        &cfg,
        r#"
[left]
format = "left-status"
"#,
    )
    .unwrap();

    let mut env = env_with_home(dir.path());
    env.insert("TMUX_SHIP_CONFIG".into(), cfg.to_string_lossy().into());

    let resolved = resolve_config(Side::Left, None, &env).unwrap();
    assert_eq!(resolved.source, "TMUX_SHIP_CONFIG");
    assert!(resolved.config_path.is_file());
}

#[test]
fn error_when_missing() {
    let dir = tempfile::tempdir().unwrap();
    let env = env_with_home(dir.path());
    let err = resolve_config(Side::Full, None, &env).unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("Unable to locate"));
}
