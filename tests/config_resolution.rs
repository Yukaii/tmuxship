use std::collections::HashMap;
use std::fs;
use std::path::Path;

use tmux_ship::config::{resolve_config, Side};

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
fn error_when_missing() {
    let dir = tempfile::tempdir().unwrap();
    let env = env_with_home(dir.path());
    let err = resolve_config(Side::Full, None, &env).unwrap_err();
    let msg = format!("{}", err);
    assert!(msg.contains("Unable to locate"));
}
