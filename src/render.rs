use crate::config::ConfigResolution;
use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::process::Command;

#[derive(Debug, Deserialize, PartialEq)]
struct StarshipModule {
    #[serde(default)]
    content: String,
    #[serde(default)]
    text: String,
    #[serde(default)]
    style: Option<String>,
}

const COLOR_NAMES: &[&str] = &[
    "black",
    "red",
    "green",
    "yellow",
    "blue",
    "magenta",
    "cyan",
    "white",
    "bright-black",
    "bright-red",
    "bright-green",
    "bright-yellow",
    "bright-blue",
    "bright-magenta",
    "bright-cyan",
    "bright-white",
];

fn parse_modules(json_blob: &str) -> Result<Vec<StarshipModule>> {
    let modules: Vec<StarshipModule> = serde_json::from_str(json_blob)
        .context("Expected JSON array from starship prompt --output=json")?;
    Ok(modules)
}

fn style_to_tmux(style: Option<&str>) -> String {
    let Some(style) = style else {
        return String::new();
    };

    let mut fg: Option<&str> = None;
    let mut bg: Option<&str> = None;
    let mut modifiers: Vec<&str> = Vec::new();

    for token in style.split_whitespace() {
        if let Some(value) = token.strip_prefix("fg:") {
            if COLOR_NAMES.contains(&value) || value.starts_with('#') {
                fg = Some(value);
            }
        } else if let Some(value) = token.strip_prefix("bg:") {
            if COLOR_NAMES.contains(&value) || value.starts_with('#') {
                bg = Some(value);
            }
        } else {
            match token {
                "bold" => modifiers.push("bold"),
                "italic" => modifiers.push("italics"),
                "underline" => modifiers.push("underscore"),
                "dimmed" => modifiers.push("dim"),
                "reverse" | "inverse" => modifiers.push("reverse"),
                "blink" => modifiers.push("blink"),
                "strikethrough" => modifiers.push("strikethrough"),
                "hidden" => modifiers.push("hidden"),
                _ => {}
            }
        }
    }

    let mut parts: Vec<String> = Vec::new();
    if let Some(fg) = fg {
        parts.push(format!("fg={}", fg));
    }
    if let Some(bg) = bg {
        parts.push(format!("bg={}", bg));
    }
    parts.extend(modifiers.iter().map(|m| m.to_string()));

    if parts.is_empty() {
        String::new()
    } else {
        format!("#[{}]", parts.join(","))
    }
}

fn render_modules(modules: Vec<StarshipModule>) -> String {
    let mut rendered = String::new();
    for module in modules {
        let content = if module.content.is_empty() {
            module.text
        } else {
            module.content
        };
        let prefix = style_to_tmux(module.style.as_deref());
        if prefix.is_empty() {
            rendered.push_str(&content);
        } else {
            rendered.push_str(&format!("{}{}#[default]", prefix, content));
        }
    }
    rendered
}

pub fn render_from_json(json_blob: &str) -> Result<String> {
    Ok(render_modules(parse_modules(json_blob)?))
}

pub fn run_starship(config: &ConfigResolution) -> Result<String> {
    let output = Command::new("starship")
        .arg("prompt")
        .arg("--output=json")
        .env("STARSHIP_CONFIG", &config.config_path)
        .output()
        .context("Failed to run starship prompt")?;

    if !output.status.success() {
        return Err(anyhow!(
            "starship exited with status {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let stdout = String::from_utf8(output.stdout).context("Starship output was not UTF-8")?;
    Ok(stdout)
}
