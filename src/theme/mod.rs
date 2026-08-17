pub mod catalog;
pub mod preview;

use anyhow::{anyhow, Context, Result};
use catalog::Theme;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub use catalog::ThemeVariant;

/// Returns the cache directory for theme TOML files
pub fn get_theme_cache_dir(theme_id: &str, env: &HashMap<String, String>) -> PathBuf {
    if let Some(xdg_cache) = env.get("XDG_CACHE_HOME") {
        PathBuf::from(xdg_cache).join("tmuxship/themes").join(theme_id)
    } else if let Some(home) = env.get("HOME") {
        PathBuf::from(home).join(".cache/tmuxship/themes").join(theme_id)
    } else if let Some(cache) = dirs::cache_dir() {
        cache.join("tmuxship/themes").join(theme_id)
    } else {
        std::env::temp_dir().join("tmuxship/themes").join(theme_id)
    }
}

/// Ensures the theme's TOML files are written to the cache directory and returns the path for the given side.
pub fn ensure_theme_file(
    theme: &Theme,
    side: crate::config::Side,
    env: &HashMap<String, String>,
) -> Result<PathBuf> {
    let cache_dir = get_theme_cache_dir(&theme.id, env);
    fs::create_dir_all(&cache_dir)
        .with_context(|| format!("Failed to create cache directory at {}", cache_dir.display()))?;

    let (filename, content) = match side {
        crate::config::Side::Left => ("starship.toml", &theme.left_toml),
        crate::config::Side::Right => (".right.toml", &theme.right_toml),
        crate::config::Side::Center => (".center.toml", &theme.center_toml),
        crate::config::Side::Full => (".full.toml", &theme.full_toml),
    };

    let target_path = cache_dir.join(filename);
    let should_write = match fs::read_to_string(&target_path) {
        Ok(existing) => existing != *content,
        Err(_) => true,
    };

    if should_write {
        fs::write(&target_path, content)
            .with_context(|| format!("Failed to write cached theme file at {}", target_path.display()))?;
    }

    Ok(target_path)
}

/// Exports theme config files to a destination directory
pub fn export_theme(theme: &Theme, dest_dir: &Path) -> Result<()> {
    fs::create_dir_all(dest_dir)
        .with_context(|| format!("Failed to create directory at {}", dest_dir.display()))?;

    fs::write(dest_dir.join("starship.toml"), &theme.left_toml)
        .with_context(|| "Failed to write starship.toml")?;
    fs::write(dest_dir.join(".right.toml"), &theme.right_toml)
        .with_context(|| "Failed to write .right.toml")?;
    fs::write(dest_dir.join(".center.toml"), &theme.center_toml)
        .with_context(|| "Failed to write .center.toml")?;
    fs::write(dest_dir.join(".full.toml"), &theme.full_toml)
        .with_context(|| "Failed to write .full.toml")?;

    Ok(())
}

/// Installs a theme into ~/.tmux or target directory
pub fn install_theme(theme: &Theme, target_dir: Option<&Path>, force: bool) -> Result<PathBuf> {
    let dest = if let Some(d) = target_dir {
        d.to_path_buf()
    } else if let Some(home) = dirs::home_dir() {
        home.join(".tmux")
    } else {
        PathBuf::from(".tmux")
    };

    if dest.exists() && !force {
        // Check if existing files would be overwritten
        let left = dest.join("starship.toml");
        let right = dest.join(".right.toml");
        let center = dest.join(".center.toml");
        if left.exists() || right.exists() || center.exists() {
            return Err(anyhow!(
                "Config files already exist in {}. Use --force to overwrite.",
                dest.display()
            ));
        }
    }

    export_theme(theme, &dest)?;
    Ok(dest)
}

/// Generates a tmux.conf snippet for a theme
pub fn generate_init_snippet(theme: &Theme) -> String {
    format!(
        r#"# ---------------------------------------------------------
# tmuxship theme: {} ({})
# ---------------------------------------------------------
setenv -g TMUX_SHIP_THEME "{}"

# Apply theme options on tmux startup
run-shell 'tmuxship apply'

# Refresh hooks for dynamic updates
set-hook -g client-session-changed 'refresh-client -S'
set-hook -g client-attached        'refresh-client -S'
set-hook -g pane-focus-in          'refresh-client -S'
set -g window-status-style "bg=default,fg=default"
"#,
        theme.name, theme.id, theme.id
    )
}
