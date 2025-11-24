use anyhow::{anyhow, Result};
use serde::Serialize;
use shellexpand::tilde;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
pub enum Side {
    Left,
    Right,
    Full,
    Center,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConfigResolution {
    pub side: Side,
    pub config_path: PathBuf,
    pub source: String,
}

impl ConfigResolution {
    fn new(side: Side, path: PathBuf, source: impl Into<String>) -> Self {
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

pub fn resolve_config(
    side: Side,
    override_path: Option<PathBuf>,
    env: &HashMap<String, String>,
) -> Result<ConfigResolution> {
    let normalized_side = match side {
        Side::Left => "LEFT",
        Side::Right => "RIGHT",
        Side::Full => "FULL",
        Side::Center => "CENTER",
    };

    if let Some(path) = override_path {
        let expanded = ensure_file(expand_user(&path), "Config override does not exist")?;
        return Ok(ConfigResolution::new(side, expanded, "override"));
    }

    let side_env_var = format!("TMUX_SHIP_{}_CONFIG", normalized_side);
    if let Some(value) = env.get(&side_env_var) {
        let expanded = ensure_file(
            expand_user(Path::new(value)),
            "Environment-specified config does not exist",
        )?;
        return Ok(ConfigResolution::new(side, expanded, side_env_var));
    }

    if let Some(value) = env.get("STARSHIP_CONFIG") {
        let expanded = ensure_file(
            expand_user(Path::new(value)),
            "STARSHIP_CONFIG points to missing file",
        )?;
        return Ok(ConfigResolution::new(side, expanded, "STARSHIP_CONFIG"));
    }

    let mut candidate_dirs: Vec<(PathBuf, &str)> = Vec::new();
    if let Some(xdg_config) = env.get("XDG_CONFIG_HOME") {
        let xdg = expand_user(Path::new(xdg_config));
        candidate_dirs.push((xdg.join("tmux"), "tmux"));
        candidate_dirs.push((xdg.join("starship"), "starship"));
    }
    if let Some(home) = env.get("HOME") {
        let home = expand_user(Path::new(home));
        candidate_dirs.push((home.join(".config/tmux"), "tmux"));
        candidate_dirs.push((home.join(".tmux"), "tmux"));
        candidate_dirs.push((home.join(".config/starship"), "starship"));
    } else if let Some(home_dir) = dirs::home_dir() {
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
        let side_path = base.join(side_filename);
        if side_path.is_file() {
            let source = if kind == "tmux" { "tmux-side" } else { "default-side" };
            return Ok(ConfigResolution::new(side, side_path, source));
        }
        let global_path = base.join("starship.toml");
        if global_path.is_file() {
            let source = if kind == "tmux" { "tmux-global" } else { "default-global" };
            return Ok(ConfigResolution::new(side, global_path, source));
        }
    }

    Err(anyhow!(
        "Unable to locate a Starship config file for tmuxship (side={:?})",
        side
    ))
}
