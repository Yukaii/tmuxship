use crate::config::ConfigResolution;
use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;
use std::process::Command;

#[derive(Debug, Clone, Default, PartialEq)]
struct Style {
    fg: Option<String>,
    bg: Option<String>,
    bold: bool,
    dim: bool,
    italics: bool,
    underscore: bool,
    reverse: bool,
    blink: bool,
    strikethrough: bool,
    hidden: bool,
}

fn color_name_from_code(code: i32, bright: bool) -> Option<&'static str> {
    match (bright, code) {
        (false, 30) => Some("black"),
        (false, 31) => Some("red"),
        (false, 32) => Some("green"),
        (false, 33) => Some("yellow"),
        (false, 34) => Some("blue"),
        (false, 35) => Some("magenta"),
        (false, 36) => Some("cyan"),
        (false, 37) => Some("white"),
        (true, 90) => Some("bright-black"),
        (true, 91) => Some("bright-red"),
        (true, 92) => Some("bright-green"),
        (true, 93) => Some("bright-yellow"),
        (true, 94) => Some("bright-blue"),
        (true, 95) => Some("bright-magenta"),
        (true, 96) => Some("bright-cyan"),
        (true, 97) => Some("bright-white"),
        _ => None,
    }
}

fn apply_color_sequence(params: &[&str], fg: bool, style: &mut Style) -> usize {
    if params.len() >= 2 && params[0] == "5" {
        if let Ok(n) = params[1].parse::<u8>() {
            let colour = format!("colour{}", n);
            if fg {
                style.fg = Some(colour);
            } else {
                style.bg = Some(colour);
            }
        }
        return 2;
    }

    if params.len() >= 4 && params[0] == "2" {
        if let (Ok(r), Ok(g), Ok(b)) = (
            params[1].parse::<u8>(),
            params[2].parse::<u8>(),
            params[3].parse::<u8>(),
        ) {
            let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);
            if fg {
                style.fg = Some(hex);
            } else {
                style.bg = Some(hex);
            }
        }
        return 4;
    }

    0
}

fn apply_sgr(params: &[&str], style: &mut Style) {
    let mut idx = 0;
    while idx < params.len() {
        let p = params[idx];
        idx += 1;

        if p.is_empty() {
            continue;
        }

        match p {
            "0" => *style = Style::default(),
            "1" => style.bold = true,
            "2" => style.dim = true,
            "3" => style.italics = true,
            "4" => style.underscore = true,
            "5" => style.blink = true,
            "7" => style.reverse = true,
            "8" => style.hidden = true,
            "9" => style.strikethrough = true,
            "22" => {
                style.bold = false;
                style.dim = false;
            }
            "23" => style.italics = false,
            "24" => style.underscore = false,
            "25" => style.blink = false,
            "27" => style.reverse = false,
            "28" => style.hidden = false,
            "29" => style.strikethrough = false,
            "39" => style.fg = None,
            "49" => style.bg = None,
            c => {
                if let Ok(code) = c.parse::<i32>() {
                    if let Some(name) = color_name_from_code(code, code >= 90) {
                        if code >= 90 {
                            style.fg = Some(name.to_string());
                        } else if code >= 30 && code <= 37 {
                            style.fg = Some(name.to_string());
                        } else if code >= 100 && code <= 107 {
                            style.bg = Some(name.to_string());
                        } else if code >= 40 && code <= 47 {
                            style.bg = Some(name.to_string());
                        }
                    } else if code == 38 {
                        let consumed = apply_color_sequence(&params[idx..], true, style);
                        idx += consumed;
                    } else if code == 48 {
                        let consumed = apply_color_sequence(&params[idx..], false, style);
                        idx += consumed;
                    }
                }
            }
        }
    }
}

fn style_to_tmux(style: &Style) -> String {
    let mut parts: Vec<String> = Vec::new();
    if let Some(fg) = &style.fg {
        parts.push(format!("fg={}", fg));
    }
    if let Some(bg) = &style.bg {
        parts.push(format!("bg={}", bg));
    }
    if style.bold {
        parts.push("bold".into());
    }
    if style.dim {
        parts.push("dim".into());
    }
    if style.italics {
        parts.push("italics".into());
    }
    if style.underscore {
        parts.push("underscore".into());
    }
    if style.reverse {
        parts.push("reverse".into());
    }
    if style.blink {
        parts.push("blink".into());
    }
    if style.strikethrough {
        parts.push("strikethrough".into());
    }
    if style.hidden {
        parts.push("hidden".into());
    }

    if parts.is_empty() {
        String::new()
    } else {
        format!("#[{}]", parts.join(","))
    }
}

pub fn render_from_ansi(ansi: &str) -> String {
    let mut rendered = String::new();
    let mut style = Style::default();
    let mut buffer = String::new();
    let mut chars = ansi.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\u{1b}' {
            if let Some('[') = chars.peek() {
                chars.next();

                if !buffer.is_empty() {
                    let prefix = style_to_tmux(&style);
                    if prefix.is_empty() {
                        rendered.push_str(&buffer);
                    } else {
                        rendered.push_str(&format!("{}{}#[default]", prefix, buffer));
                    }
                    buffer.clear();
                }

                let mut code = String::new();
                while let Some(c) = chars.next() {
                    if c == 'm' {
                        break;
                    }
                    code.push(c);
                }
                let params: Vec<&str> = code.split(';').collect();
                apply_sgr(&params, &mut style);
                continue;
            }
        }
        buffer.push(ch);
    }

    if !buffer.is_empty() {
        let prefix = style_to_tmux(&style);
        if prefix.is_empty() {
            rendered.push_str(&buffer);
        } else {
            rendered.push_str(&format!("{}{}#[default]", prefix, buffer));
        }
    }

    rendered
}

fn default_tmux_vars() -> Vec<String> {
    vec![
        // Session variables
        "session_name".to_string(),
        "session_id".to_string(),
        "session_created".to_string(),
        "session_attached".to_string(),
        "session_windows".to_string(),
        // Window variables
        "window_id".to_string(),
        "window_index".to_string(),
        "window_name".to_string(),
        "window_active".to_string(),
        "window_flags".to_string(),
        "window_layout".to_string(),
        "window_panes".to_string(),
        "window_width".to_string(),
        "window_height".to_string(),
        "window_zoomed_flag".to_string(),
        // Pane variables
        "pane_id".to_string(),
        "pane_index".to_string(),
        "pane_title".to_string(),
        "pane_current_path".to_string(),
        "pane_current_command".to_string(),
        "pane_pid".to_string(),
        "pane_width".to_string(),
        "pane_height".to_string(),
        "pane_active".to_string(),
        "pane_at_top".to_string(),
        "pane_at_bottom".to_string(),
        "pane_at_left".to_string(),
        "pane_at_right".to_string(),
        // Client variables
        "client_prefix".to_string(),
        "client_width".to_string(),
        "client_height".to_string(),
        "client_termname".to_string(),
        // Host variables
        "host".to_string(),
        "host_short".to_string(),
    ]
}

fn tmux_env_vars(env: &HashMap<String, String>) -> Result<Vec<(String, String)>> {
    let target = env.get("TMUX_SHIP_TARGET").map(|t| t.as_str());
    if let Some(t) = target {
        if !t
            .chars()
            .all(|c| c.is_ascii_graphic() && !c.is_whitespace())
        {
            return Err(anyhow!(
                "TMUX_SHIP_TARGET contained invalid characters: {}",
                t
            ));
        }
    }

    let vars: Vec<String> = if let Some(raw_list) = env.get("TMUX_SHIP_TMUX_VARS") {
        // User explicitly specified variables
        raw_list
            .split(',')
            .map(|v| v.trim())
            .filter(|v| !v.is_empty())
            .map(|v| v.to_string())
            .collect()
    } else {
        // Auto-fetch default set of common tmux variables
        default_tmux_vars()
    };

    if vars.is_empty() {
        return Ok(Vec::new());
    }

    for var in &vars {
        if !var
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            return Err(anyhow!(
                "TMUX_SHIP_TMUX_VARS contained an invalid tmux variable name: {}",
                var
            ));
        }
    }

    let format = vars
        .iter()
        .map(|v| format!("#{{{}}}", v))
        .collect::<Vec<_>>()
        .join("\n");

    let mut cmd = Command::new("tmux");
    cmd.arg("display-message").arg("-p").arg("-F").arg(format);
    if let Some(target) = target {
        cmd.arg("-t").arg(target);
    }

    let output = cmd.output().context("Failed to query tmux for variables")?;

    if !output.status.success() {
        return Err(anyhow!(
            "tmux exited with status {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let stdout = String::from_utf8(output.stdout).context("tmux output was not UTF-8")?;
    let mut values: Vec<String> = stdout.lines().map(|v| v.to_string()).collect();
    if values.len() < vars.len() {
        values.resize(vars.len(), String::new());
    }
    if values.len() > vars.len() {
        values.truncate(vars.len());
    }

    let env_vars = vars
        .iter()
        .zip(values.iter())
        .map(|(var, value)| {
            let sanitized: String = var
                .chars()
                .map(|c| {
                    if c.is_ascii_alphanumeric() {
                        c.to_ascii_uppercase()
                    } else {
                        '_'
                    }
                })
                .collect();
            (format!("TMUX_{}", sanitized), value.to_string())
        })
        .collect();

    Ok(env_vars)
}

pub fn run_starship(config: &ConfigResolution, env: &HashMap<String, String>) -> Result<String> {
    let tmux_env = tmux_env_vars(env)?;

    let output = Command::new("starship")
        .arg("prompt")
        .env("STARSHIP_CONFIG", &config.config_path)
        .env("STARSHIP_SHELL", "sh")
        // Force color even though stdout is not a TTY.
        .env("CLICOLOR_FORCE", "1")
        .envs(tmux_env)
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
