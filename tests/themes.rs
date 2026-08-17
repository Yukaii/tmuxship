use std::collections::HashMap;
use std::fs;
use tempfile::tempdir;
use tmuxship::config::{resolve_config_with_theme, Side};
use tmuxship::theme::catalog::{all_themes, find_theme, ThemeVariant};
use tmuxship::theme::{ensure_theme_file, export_theme, generate_init_snippet, install_theme};
use tmuxship::tmux_conf::emit_tmux_conf_with_theme;

#[test]
fn test_all_themes_catalog() {
    let themes = all_themes();
    assert!(themes.len() >= 17);

    // Verify key legendary themes exist
    assert!(find_theme("rose-pine").is_some());
    assert!(find_theme("rose-pine-moon").is_some());
    assert!(find_theme("rose-pine-dawn").is_some());
    assert!(find_theme("catppuccin-mocha").is_some());
    assert!(find_theme("catppuccin-latte").is_some());
    assert!(find_theme("tokyo-night").is_some());
    assert!(find_theme("nord").is_some());
    assert!(find_theme("gruvbox-dark").is_some());
    assert!(find_theme("dracula").is_some());
    assert!(find_theme("kanagawa").is_some());
    assert!(find_theme("onedark").is_some());
    assert!(find_theme("solarized-dark").is_some());

    // Verify theme properties
    let rp = find_theme("rose-pine").unwrap();
    assert_eq!(rp.id, "rose-pine");
    assert_eq!(rp.variant, ThemeVariant::Dark);
    assert!(!rp.left_toml.is_empty());
    assert!(!rp.center_toml.is_empty());
    assert!(!rp.right_toml.is_empty());
    assert!(!rp.full_toml.is_empty());
}

#[test]
fn test_ensure_theme_file_creates_cached_files() {
    let dir = tempdir().unwrap();
    let mut env = HashMap::new();
    env.insert(
        "XDG_CACHE_HOME".to_string(),
        dir.path().to_string_lossy().to_string(),
    );

    let theme = find_theme("rose-pine").unwrap();
    let left_path = ensure_theme_file(&theme, Side::Left, &env).unwrap();
    let right_path = ensure_theme_file(&theme, Side::Right, &env).unwrap();
    let center_path = ensure_theme_file(&theme, Side::Center, &env).unwrap();

    assert!(left_path.is_file());
    assert!(right_path.is_file());
    assert!(center_path.is_file());

    let left_content = fs::read_to_string(left_path).unwrap();
    assert_eq!(left_content, theme.left_toml);
}

#[test]
fn test_resolve_config_with_theme_override() {
    let dir = tempdir().unwrap();
    let mut env = HashMap::new();
    env.insert(
        "XDG_CACHE_HOME".to_string(),
        dir.path().to_string_lossy().to_string(),
    );

    let resolved = resolve_config_with_theme(Side::Left, None, Some("rose-pine"), &env).unwrap();
    assert_eq!(resolved.source, "theme:rose-pine");
    assert!(resolved.config_path.is_file());
}

#[test]
fn test_resolve_config_with_theme_env() {
    let dir = tempdir().unwrap();
    let mut env = HashMap::new();
    env.insert(
        "XDG_CACHE_HOME".to_string(),
        dir.path().to_string_lossy().to_string(),
    );
    env.insert(
        "TMUX_SHIP_THEME".to_string(),
        "catppuccin-mocha".to_string(),
    );

    let resolved = resolve_config_with_theme(Side::Center, None, None, &env).unwrap();
    assert_eq!(resolved.source, "theme:catppuccin-mocha");
    assert!(resolved.config_path.is_file());
}

#[test]
fn test_export_and_install_theme() {
    let dir = tempdir().unwrap();
    let export_dir = dir.path().join("exported");
    let theme = find_theme("tokyo-night").unwrap();

    export_theme(&theme, &export_dir).unwrap();
    assert!(export_dir.join("starship.toml").is_file());
    assert!(export_dir.join(".center.toml").is_file());
    assert!(export_dir.join(".right.toml").is_file());
    assert!(export_dir.join(".full.toml").is_file());

    let install_dir = dir.path().join("installed");
    install_theme(&theme, Some(&install_dir), false).unwrap();
    assert!(install_dir.join("starship.toml").is_file());

    // Installing without force should fail if files already exist
    assert!(install_theme(&theme, Some(&install_dir), false).is_err());
    // Installing with force should succeed
    assert!(install_theme(&theme, Some(&install_dir), true).is_ok());
}

#[test]
fn test_generate_init_snippet() {
    let theme = find_theme("rose-pine").unwrap();
    let snippet = generate_init_snippet(&theme);
    assert!(snippet.contains("setenv -g TMUX_SHIP_THEME \"rose-pine\""));
    assert!(snippet.contains("run-shell 'tmuxship apply'"));
}

#[test]
fn test_custom_theme_directory_discovery() {
    let dir = tempdir().unwrap();
    let custom_themes_dir = dir.path().join("my-themes");
    let custom_theme_dir = custom_themes_dir.join("synthwave");
    fs::create_dir_all(&custom_theme_dir).unwrap();

    fs::write(
        custom_theme_dir.join("theme.toml"),
        r#"
name = "Synthwave 84"
description = "Neon 80s aesthetics"
author = "Robb Owen"
variant = "dark"
"#,
    )
    .unwrap();
    fs::write(custom_theme_dir.join("starship.toml"), "# synthwave left\n").unwrap();
    fs::write(
        custom_theme_dir.join(".center.toml"),
        "# synthwave center\n",
    )
    .unwrap();
    fs::write(custom_theme_dir.join(".right.toml"), "# synthwave right\n").unwrap();

    let mut env = HashMap::new();
    env.insert(
        "TMUX_SHIP_THEMES_DIR".to_string(),
        custom_themes_dir.to_string_lossy().to_string(),
    );
    env.insert(
        "XDG_CACHE_HOME".to_string(),
        dir.path().to_string_lossy().to_string(),
    );

    let found = tmuxship::theme::catalog::find_theme_with_env("synthwave", &env);
    assert!(found.is_some());
    let custom = found.unwrap();
    assert_eq!(custom.name, "Synthwave 84");
    assert!(custom.is_custom);
}

#[test]
fn test_emit_tmux_conf_with_theme() {
    let dir = tempdir().unwrap();
    let mut env = HashMap::new();
    env.insert(
        "XDG_CACHE_HOME".to_string(),
        dir.path().to_string_lossy().to_string(),
    );

    let options = emit_tmux_conf_with_theme(Some("rose-pine"), &env).unwrap();
    let conf_map: HashMap<_, _> = options.into_iter().map(|o| (o.name, o.value)).collect();

    assert!(conf_map.contains_key("status-left"));
    assert!(conf_map["status-left"].contains("#[bg=#eb6f92,fg=#191724,bold]#S"));
    assert!(conf_map.contains_key("status-right"));
    assert_eq!(
        conf_map["status-right"],
        "#(tmuxship right --theme rose-pine)"
    );
    assert!(conf_map.contains_key("window-status-current-format"));
    assert!(
        conf_map["window-status-current-format"].contains("#[bg=#26233a,fg=#ebbcba,bold]###I #W")
    );
}
