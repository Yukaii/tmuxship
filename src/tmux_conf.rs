use crate::config::{resolve_config, Side};
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
    env: &HashMap<String, String>,
) -> Result<Option<StarshipConfig>> {
    let config = match resolve_config(side, None, env) {
        Ok(config) => config,
        Err(_) => return Ok(None),
    };
    let raw = fs::read_to_string(&config.config_path)
        .with_context(|| format!("Failed to read {}", config.config_path.display()))?;
    let parsed = toml::from_str(&raw)
        .with_context(|| format!("Failed to parse {}", config.config_path.display()))?;
    Ok(Some(parsed))
}

fn tmux_style(starship_style: &str) -> String {
    let parts = starship_style
        .split_whitespace()
        .map(|part| {
            if let Some(value) = part.strip_prefix("fg:") {
                format!("fg={}", value)
            } else if let Some(value) = part.strip_prefix("bg:") {
                format!("bg={}", value)
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

pub fn emit_tmux_conf(env: &HashMap<String, String>) -> Result<Vec<TmuxOption>> {
    let mut options = Vec::new();

    if let Some(left) = read_starship_config(Side::Left, env)? {
        if let (Some(prefix), Some(normal)) = (
            custom_style(&left, "prefix_active"),
            custom_style(&left, "session_normal"),
        ) {
            options.push(TmuxOption {
                name: "status-left".to_string(),
                value: format!(
                    "#{{?client_prefix,{}#S #[default],{}#S #[default]}}",
                    prefix, normal
                ),
            });
        }
    }

    if read_starship_config(Side::Right, env)?.is_some() {
        options.push(TmuxOption {
            name: "status-right".to_string(),
            value: "#(tmuxship right)".to_string(),
        });
    }

    if let Some(center) = read_starship_config(Side::Center, env)? {
        if let Some(inactive) = custom_style(&center, "window_inactive") {
            options.push(TmuxOption {
                name: "window-status-format".to_string(),
                value: format!("{}##I #W #[default]", inactive),
            });
        }

        if let Some(active) = custom_style(&center, "window_active") {
            let zoom = match (
                custom_style(&center, "window_zoom"),
                custom_static_output(&center, "window_zoom"),
            ) {
                (Some(style), Some(output)) => {
                    format!(" #{{?window_zoomed_flag,{}{}#[default],}}", style, output)
                }
                _ => String::new(),
            };
            options.push(TmuxOption {
                name: "window-status-current-format".to_string(),
                value: format!("{}##I #W{}#[default]", active, zoom),
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
