use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeVariant {
    Dark,
    Light,
}

impl std::fmt::Display for ThemeVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThemeVariant::Dark => write!(f, "dark"),
            ThemeVariant::Light => write!(f, "light"),
        }
    }
}

fn default_variant() -> ThemeVariant {
    ThemeVariant::Dark
}

fn default_separator() -> String {
    " • ".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorSwatch {
    pub name: String,
    pub hex: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeMeta {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub author: String,
    #[serde(default = "default_variant")]
    pub variant: ThemeVariant,
    #[serde(default = "default_separator")]
    pub window_separator: String,
    #[serde(default)]
    pub swatches: Vec<ColorSwatch>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Theme {
    pub id: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub variant: ThemeVariant,
    pub window_separator: String,
    pub swatches: Vec<ColorSwatch>,
    pub unified_toml: String,
    pub left_toml: String,
    pub center_toml: String,
    pub right_toml: String,
    pub full_toml: String,
    #[serde(skip)]
    pub is_custom: bool,
}

pub fn parse_theme(id: &str, raw_toml: &str, is_custom: bool) -> Result<Theme> {
    let toml_val: toml::Value = toml::from_str(raw_toml)
        .with_context(|| format!("Failed to parse theme TOML for '{id}'"))?;

    let meta: ThemeMeta = match toml::from_str(raw_toml) {
        Ok(m) => m,
        Err(_) => ThemeMeta {
            name: id.to_string(),
            description: String::new(),
            author: "Unknown".to_string(),
            variant: ThemeVariant::Dark,
            window_separator: " • ".to_string(),
            swatches: Vec::new(),
        },
    };

    let name = if meta.name.is_empty() {
        id.to_string()
    } else {
        meta.name
    };

    let extract_sub = |key: &str| -> String {
        if let Some(val) = toml_val.get(key) {
            if val.is_table() {
                return toml::to_string_pretty(val).unwrap_or_default();
            }
        }
        String::new()
    };

    let left = extract_sub("left");
    let center = extract_sub("center");
    let right = extract_sub("right");
    let mut full = extract_sub("full");
    if full.is_empty() && !left.is_empty() {
        full = format!("{left}\n{right}");
    }

    Ok(Theme {
        id: id.to_string(),
        name,
        description: meta.description,
        author: meta.author,
        variant: meta.variant,
        window_separator: meta.window_separator,
        swatches: meta.swatches,
        unified_toml: raw_toml.to_string(),
        left_toml: left,
        center_toml: center,
        right_toml: right,
        full_toml: full,
        is_custom,
    })
}

pub fn builtin_themes() -> Vec<Theme> {
    let raw_themes: &[(&str, &str)] = &[
        ("rose-pine", include_str!("../../themes/rose-pine.toml")),
        (
            "rose-pine-moon",
            include_str!("../../themes/rose-pine-moon.toml"),
        ),
        (
            "rose-pine-dawn",
            include_str!("../../themes/rose-pine-dawn.toml"),
        ),
        (
            "catppuccin-mocha",
            include_str!("../../themes/catppuccin-mocha.toml"),
        ),
        (
            "catppuccin-macchiato",
            include_str!("../../themes/catppuccin-macchiato.toml"),
        ),
        (
            "catppuccin-frappe",
            include_str!("../../themes/catppuccin-frappe.toml"),
        ),
        (
            "catppuccin-latte",
            include_str!("../../themes/catppuccin-latte.toml"),
        ),
        ("tokyo-night", include_str!("../../themes/tokyo-night.toml")),
        (
            "tokyo-night-moon",
            include_str!("../../themes/tokyo-night-moon.toml"),
        ),
        ("nord", include_str!("../../themes/nord.toml")),
        (
            "gruvbox-dark",
            include_str!("../../themes/gruvbox-dark.toml"),
        ),
        (
            "gruvbox-light",
            include_str!("../../themes/gruvbox-light.toml"),
        ),
        ("dracula", include_str!("../../themes/dracula.toml")),
        ("kanagawa", include_str!("../../themes/kanagawa.toml")),
        ("onedark", include_str!("../../themes/onedark.toml")),
        (
            "solarized-dark",
            include_str!("../../themes/solarized-dark.toml"),
        ),
        (
            "solarized-light",
            include_str!("../../themes/solarized-light.toml"),
        ),
    ];

    raw_themes
        .iter()
        .filter_map(|(id, raw)| parse_theme(id, raw, false).ok())
        .collect()
}

pub fn custom_theme_dirs(env: &HashMap<String, String>) -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Some(custom_dir) = env.get("TMUX_SHIP_THEMES_DIR") {
        dirs.push(PathBuf::from(custom_dir));
    }
    if let Some(xdg) = env.get("XDG_CONFIG_HOME") {
        dirs.push(PathBuf::from(xdg).join("tmuxship/themes"));
        dirs.push(PathBuf::from(xdg).join("tmux/themes"));
    }
    if let Some(home) = env.get("HOME") {
        let h = PathBuf::from(home);
        dirs.push(h.join(".config/tmuxship/themes"));
        dirs.push(h.join(".tmux/themes"));
    } else if let Some(home_dir) = dirs::home_dir() {
        dirs.push(home_dir.join(".config/tmuxship/themes"));
        dirs.push(home_dir.join(".tmux/themes"));
    }
    dirs
}

fn load_custom_theme_from_path(path: &Path) -> Option<Theme> {
    if path.is_file() {
        // Single file: <id>.toml or tmuxship.toml
        let id = path.file_stem()?.to_str()?;
        let raw = fs::read_to_string(path).ok()?;
        return parse_theme(id, &raw, true).ok();
    } else if path.is_dir() {
        // Directory: check theme.toml or tmuxship.toml
        let id = path.file_name()?.to_str()?;
        let candidates = ["tmuxship.toml", "theme.toml"];
        for name in candidates {
            let p = path.join(name);
            if p.is_file() {
                let raw = fs::read_to_string(p).ok()?;
                return parse_theme(id, &raw, true).ok();
            }
        }
    }
    None
}

#[allow(dead_code)]
pub fn find_theme(name_or_id: &str) -> Option<Theme> {
    find_theme_with_env(name_or_id, &HashMap::new())
}

pub fn find_theme_with_env(name_or_id: &str, env: &HashMap<String, String>) -> Option<Theme> {
    let query = name_or_id.trim().to_lowercase().replace(' ', "-");

    // 1. Check custom theme directories
    for base in custom_theme_dirs(env) {
        if base.is_dir() {
            // Check <query>.toml
            let single_file = base.join(format!("{query}.toml"));
            if let Some(theme) = load_custom_theme_from_path(&single_file) {
                return Some(theme);
            }
            // Check <query>/
            let dir_path = base.join(&query);
            if let Some(theme) = load_custom_theme_from_path(&dir_path) {
                return Some(theme);
            }
        }
    }

    // 2. Fall back to built-in themes
    let builtins = builtin_themes();
    builtins.into_iter().find(|t| {
        t.id == query
            || t.id.replace('-', "") == query.replace('-', "")
            || t.name.to_lowercase() == query
    })
}

pub fn all_themes() -> Vec<Theme> {
    all_themes_with_env(&HashMap::new())
}

pub fn all_themes_with_env(env: &HashMap<String, String>) -> Vec<Theme> {
    let mut map: HashMap<String, Theme> = HashMap::new();

    // 1. Add built-in themes
    for t in builtin_themes() {
        map.insert(t.id.clone(), t);
    }

    // 2. Add custom themes from disk (overrides or adds to list)
    for base in custom_theme_dirs(env) {
        if let Ok(entries) = fs::read_dir(base) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(theme) = load_custom_theme_from_path(&path) {
                    map.insert(theme.id.clone(), theme);
                }
            }
        }
    }

    let mut result: Vec<Theme> = map.into_values().collect();
    result.sort_by(|a, b| a.id.cmp(&b.id));
    result
}
