use anyhow::{anyhow, Context, Result};
use serde::Serialize;
use shellexpand::tilde;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::theme::{catalog::find_theme_with_env, ensure_theme_file};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
pub enum Side {
    Left,
    Right,
    Full,
    Center,
}

impl Side {
    pub fn as_str(&self) -> &'static str {
        match self {
            Side::Left => "left",
            Side::Right => "right",
            Side::Full => "full",
            Side::Center => "center",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConfigResolution {
    pub side: Side,
    pub config_path: PathBuf,
    pub source: String,
}

impl ConfigResolution {
    pub fn new(side: Side, path: PathBuf, source: impl Into<String>) -> Self {
        Self {
            side,
            config_path: path,
            source: source.into(),
        }
    }
}

fn expand_user(path: &Path) -> PathBuf {
    let as_str = path.to_string_lossy();
    let expanded = tilde(&as_str);
    PathBuf::from(expanded.as_ref())
}

fn ensure_file(path: PathBuf, context: &str) -> Result<PathBuf> {
    if path.is_file() {
        Ok(path)
    } else {
        Err(anyhow!("{}: {}", context, path.display()))
    }
}

/// Checks if a file is a unified tmuxship.toml (containing [left], [center], [right], or [full] tables).
/// If so, extracts the requested side into a cached Starship-ready config file.
pub fn extract_side_if_unified(
    file_path: &Path,
    side: Side,
    env: &HashMap<String, String>,
) -> Result<PathBuf> {
    let raw = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read config file at {}", file_path.display()))?;

    let toml_val: toml::Value = match toml::from_str(&raw) {
        Ok(val) => val,
        Err(_) => return Ok(file_path.to_path_buf()), // Not valid TOML or legacy format, pass as-is
    };

    let side_key = side.as_str();
    if let Some(side_val) = toml_val.get(side_key) {
        if side_val.is_table() {
            // It's a unified configuration file!
            let cache_base = if let Some(xdg_cache) = env.get("XDG_CACHE_HOME") {
                PathBuf::from(xdg_cache).join("tmuxship/extracted")
            } else if let Some(home) = env.get("HOME") {
                PathBuf::from(home).join(".cache/tmuxship/extracted")
            } else if let Some(cache) = dirs::cache_dir() {
                cache.join("tmuxship/extracted")
            } else {
                std::env::temp_dir().join("tmuxship/extracted")
            };

            let file_stem = file_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("config");

            let target_dir = cache_base.join(file_stem);
            fs::create_dir_all(&target_dir)?;

            let target_path = target_dir.join(format!("{side_key}.toml"));
            let mut extracted_toml = toml::to_string_pretty(side_val)?;

            // If the root TOML has palettes/palette and side does not, attach it
            if let Some(palettes) = toml_val.get("palettes") {
                extracted_toml.push_str(&format!(
                    "\n[palettes]\n{}",
                    toml::to_string_pretty(palettes)?
                ));
            } else if let Some(palette) = toml_val.get("palette") {
                extracted_toml.push_str(&format!(
                    "\n[palette]\n{}",
                    toml::to_string_pretty(palette)?
                ));
            }

            let should_write = match fs::read_to_string(&target_path) {
                Ok(existing) => existing != extracted_toml,
                Err(_) => true,
            };

            if should_write {
                fs::write(&target_path, extracted_toml)?;
            }

            return Ok(target_path);
        }
    }

    // Single standalone starship config file
    Ok(file_path.to_path_buf())
}

#[allow(dead_code)]
pub fn resolve_config(
    side: Side,
    override_path: Option<PathBuf>,
    env: &HashMap<String, String>,
) -> Result<ConfigResolution> {
    resolve_config_with_theme(side, override_path, None, env)
}

pub fn resolve_config_with_theme(
    side: Side,
    override_path: Option<PathBuf>,
    theme_override: Option<&str>,
    env: &HashMap<String, String>,
) -> Result<ConfigResolution> {
    let normalized_side = match side {
        Side::Left => "LEFT",
        Side::Right => "RIGHT",
        Side::Full => "FULL",
        Side::Center => "CENTER",
    };

    // 1. Direct CLI override (--config)
    if let Some(path) = override_path {
        let expanded = ensure_file(expand_user(&path), "Config override does not exist")?;
        let final_path = extract_side_if_unified(&expanded, side, env)?;
        return Ok(ConfigResolution::new(side, final_path, "override"));
    }

    // 2. Side-specific env var (e.g. TMUX_SHIP_LEFT_CONFIG)
    let side_env_var = format!("TMUX_SHIP_{normalized_side}_CONFIG");
    if let Some(value) = env.get(&side_env_var) {
        let expanded = ensure_file(
            expand_user(Path::new(value)),
            "Environment-specified config does not exist",
        )?;
        return Ok(ConfigResolution::new(side, expanded, side_env_var));
    }

    // 3. Unified TMUX_SHIP_CONFIG environment variable
    if let Some(value) = env.get("TMUX_SHIP_CONFIG") {
        let expanded = ensure_file(
            expand_user(Path::new(value)),
            "TMUX_SHIP_CONFIG points to missing file",
        )?;
        let final_path = extract_side_if_unified(&expanded, side, env)?;
        return Ok(ConfigResolution::new(side, final_path, "TMUX_SHIP_CONFIG"));
    }

    // 4. STARSHIP_CONFIG environment variable
    if let Some(value) = env.get("STARSHIP_CONFIG") {
        let expanded = ensure_file(
            expand_user(Path::new(value)),
            "STARSHIP_CONFIG points to missing file",
        )?;
        return Ok(ConfigResolution::new(side, expanded, "STARSHIP_CONFIG"));
    }

    // 5. Theme override or TMUX_SHIP_THEME
    let theme_name = theme_override.or_else(|| env.get("TMUX_SHIP_THEME").map(|s| s.as_str()));
    if let Some(name) = theme_name {
        if let Some(theme) = find_theme_with_env(name, env) {
            let cached_path = ensure_theme_file(&theme, side, env)?;
            return Ok(ConfigResolution::new(
                side,
                cached_path,
                format!("theme:{}", theme.id),
            ));
        } else {
            return Err(anyhow!(
                "Unknown theme: '{name}'. Run `tmuxship theme list` to see available themes."
            ));
        }
    }

    // 6. Directory search for unified tmuxship.toml or side configs
    let mut candidate_dirs: Vec<(PathBuf, &str)> = Vec::new();
    if let Some(xdg_config) = env.get("XDG_CONFIG_HOME") {
        let xdg = expand_user(Path::new(xdg_config));
        candidate_dirs.push((xdg.join("tmuxship"), "tmuxship"));
        candidate_dirs.push((xdg.join("tmux"), "tmux"));
        candidate_dirs.push((xdg.join("starship"), "starship"));
    }
    if let Some(home) = env.get("HOME") {
        let home = expand_user(Path::new(home));
        candidate_dirs.push((home.join(".config/tmuxship"), "tmuxship"));
        candidate_dirs.push((home.join(".config/tmux"), "tmux"));
        candidate_dirs.push((home.join(".tmux"), "tmux"));
        candidate_dirs.push((home.join(".config/starship"), "starship"));
    } else if let Some(home_dir) = dirs::home_dir() {
        candidate_dirs.push((home_dir.join(".config/tmuxship"), "tmuxship"));
        candidate_dirs.push((home_dir.join(".config/tmux"), "tmux"));
        candidate_dirs.push((home_dir.join(".tmux"), "tmux"));
        candidate_dirs.push((home_dir.join(".config/starship"), "starship"));
    }

    let side_filename = match side {
        Side::Left => ".left.toml",
        Side::Right => ".right.toml",
        Side::Full => ".full.toml",
        Side::Center => ".center.toml",
    };

    for (base, kind) in candidate_dirs {
        if !base.exists() {
            continue;
        }

        // Check for unified tmuxship.toml first
        let unified_path = base.join("tmuxship.toml");
        if unified_path.is_file() {
            let final_path = extract_side_if_unified(&unified_path, side, env)?;
            return Ok(ConfigResolution::new(
                side,
                final_path,
                format!("{kind}-unified"),
            ));
        }

        // Check for side-specific file
        let side_path = base.join(side_filename);
        if side_path.is_file() {
            let source = if kind == "tmux" || kind == "tmuxship" {
                "tmux-side"
            } else {
                "default-side"
            };
            return Ok(ConfigResolution::new(side, side_path, source));
        }

        // Check for global starship.toml
        let global_path = base.join("starship.toml");
        if global_path.is_file() {
            let source = if kind == "tmux" || kind == "tmuxship" {
                "tmux-global"
            } else {
                "default-global"
            };
            return Ok(ConfigResolution::new(side, global_path, source));
        }
    }

    Err(anyhow!(
        "Unable to locate a Starship config file for tmuxship (side={side:?})"
    ))
}
