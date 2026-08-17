use crate::config::{resolve_config_with_theme, Side};
use crate::theme::catalog::find_theme_with_env;
use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct StarshipConfig {
    #[serde(default)]
    custom: HashMap<String, CustomModule>,
}

#[derive(Debug, Deserialize)]
struct CustomModule {
    command: Option<String>,
    style: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TmuxOption {
    pub name: String,
    pub value: String,
}

fn read_starship_config(
    side: Side,
    theme_override: Option<&str>,
    env: &HashMap<String, String>,
) -> Result<Option<StarshipConfig>> {
    let config = match resolve_config_with_theme(side, None, theme_override, env) {
        Ok(config) => config,
        Err(_) => return Ok(None),
    };
    let raw = fs::read_to_string(&config.config_path)
        .with_context(|| format!("Failed to read {}", config.config_path.display()))?;
    let toml_val: toml::Value = toml::from_str(&raw)
        .with_context(|| format!("Failed to parse {}", config.config_path.display()))?;

    let side_key = side.as_str();
    if let Some(sub_val) = toml_val.get(side_key) {
        if sub_val.is_table() {
            let parsed: StarshipConfig = sub_val.clone().try_into().with_context(|| {
                format!(
                    "Failed to parse [{}] from {}",
                    side_key,
                    config.config_path.display()
                )
            })?;
            return Ok(Some(parsed));
        }
    }

    let parsed = toml::from_str(&raw)
        .with_context(|| format!("Failed to parse {}", config.config_path.display()))?;
    Ok(Some(parsed))
}

fn tmux_style(starship_style: &str) -> String {
    let parts = starship_style
        .split_whitespace()
        .map(|part| {
            if let Some(value) = part.strip_prefix("fg:") {
                format!("fg={value}")
            } else if let Some(value) = part.strip_prefix("bg:") {
                format!("bg={value}")
            } else {
                part.to_string()
            }
        })
        .collect::<Vec<_>>();

    format!("#[{}]", parts.join(","))
}

fn custom_style(config: &StarshipConfig, name: &str) -> Option<String> {
    config
        .custom
        .get(name)
        .and_then(|module| module.style.as_deref())
        .map(tmux_style)
}

fn custom_static_output(config: &StarshipConfig, name: &str) -> Option<String> {
    let command = config.custom.get(name)?.command.as_deref()?.trim();
    let rest = command.strip_prefix("printf ")?.trim_start();
    let quote = rest.chars().next()?;
    if quote != '\'' && quote != '"' {
        return None;
    }

    let mut output = String::new();
    let mut escaped = false;
    for ch in rest[quote.len_utf8()..].chars() {
        if escaped {
            output.push(ch);
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else if ch == quote {
            return Some(output);
        } else {
            output.push(ch);
        }
    }

    None
}

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

#[allow(dead_code)]
pub fn emit_tmux_conf(env: &HashMap<String, String>) -> Result<Vec<TmuxOption>> {
    emit_tmux_conf_with_theme(None, env)
}

pub fn emit_tmux_conf_with_theme(
    theme_override: Option<&str>,
    env: &HashMap<String, String>,
) -> Result<Vec<TmuxOption>> {
    let mut options = Vec::new();

    // Determine window separator: user env > theme default > standard default
    let active_theme_name =
        theme_override.or_else(|| env.get("TMUX_SHIP_THEME").map(|s| s.as_str()));
    let theme_separator = active_theme_name
        .and_then(|name| find_theme_with_env(name, env))
        .map(|t| t.window_separator)
        .unwrap_or_else(|| " • ".to_string());

    let window_separator = env
        .get("TMUX_SHIP_WINDOW_SEPARATOR")
        .map(|value| value.as_str())
        .unwrap_or(&theme_separator);

    if let Some(left) = read_starship_config(Side::Left, theme_override, env)? {
        if let (Some(prefix), Some(normal)) = (
            custom_style(&left, "prefix_active"),
            custom_style(&left, "session_normal"),
        ) {
            options.push(TmuxOption {
                name: "status-left".to_string(),
                value: format!("#{{?client_prefix,{prefix}#S #[default],{normal}#S #[default]}}"),
            });
        }
    }

    if read_starship_config(Side::Right, theme_override, env)?.is_some() {
        let right_cmd = if let Some(theme) = theme_override {
            format!("#(tmuxship right --theme {theme})")
        } else {
            "#(tmuxship right)".to_string()
        };
        options.push(TmuxOption {
            name: "status-right".to_string(),
            value: right_cmd,
        });
    }

    if let Some(center) = read_starship_config(Side::Center, theme_override, env)? {
        if let Some(inactive) = custom_style(&center, "window_inactive") {
            options.push(TmuxOption {
                name: "window-status-separator".to_string(),
                value: format!("{inactive}{window_separator}#[default]"),
            });

            options.push(TmuxOption {
                name: "window-status-format".to_string(),
                value: format!("{inactive}###I #W #[default]"),
            });
        }

        if let Some(active) = custom_style(&center, "window_active") {
            let zoom = match (
                custom_style(&center, "window_zoom"),
                custom_static_output(&center, "window_zoom"),
            ) {
                (Some(style), Some(output)) => {
                    format!(" #{{?window_zoomed_flag,{style}{output}#[default],}}")
                }
                _ => String::new(),
            };
            options.push(TmuxOption {
                name: "window-status-current-format".to_string(),
                value: format!("{active}###I #W{zoom}#[default]"),
            });
        }
    }

    Ok(options)
}

pub fn format_tmux_conf(options: &[TmuxOption]) -> String {
    options
        .iter()
        .map(|option| format!("set -g {} {}", option.name, shell_quote(&option.value)))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn apply_tmux_conf(options: &[TmuxOption]) -> Result<()> {
    for option in options {
        let output = Command::new("tmux")
            .arg("set-option")
            .arg("-g")
            .arg(&option.name)
            .arg(&option.value)
            .output()
            .with_context(|| format!("Failed to set tmux option {}", option.name))?;

        if !output.status.success() {
            anyhow::bail!(
                "tmux exited with status {}: {}",
                output.status,
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    Ok(())
}
