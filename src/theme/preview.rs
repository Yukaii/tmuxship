use super::catalog::{Theme, ThemeVariant};
use std::io::{self, Write};

fn parse_hex_color(hex: &str) -> Option<(u8, u8, u8)> {
    let clean = hex.trim().trim_start_matches('#');
    if clean.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&clean[0..2], 16).ok()?;
    let g = u8::from_str_radix(&clean[2..4], 16).ok()?;
    let b = u8::from_str_radix(&clean[4..6], 16).ok()?;
    Some((r, g, b))
}

pub fn hex_fg(hex: &str) -> String {
    if let Some((r, g, b)) = parse_hex_color(hex) {
        format!("\x1b[38;2;{r};{g};{b}m")
    } else {
        String::new()
    }
}

pub fn hex_bg(hex: &str) -> String {
    if let Some((r, g, b)) = parse_hex_color(hex) {
        format!("\x1b[48;2;{r};{g};{b}m")
    } else {
        String::new()
    }
}

pub fn style_to_ansi(style: &str) -> String {
    let mut ansi = String::new();
    for part in style.split_whitespace() {
        if let Some(fg) = part.strip_prefix("fg:") {
            ansi.push_str(&hex_fg(fg));
        } else if let Some(bg) = part.strip_prefix("bg:") {
            ansi.push_str(&hex_bg(bg));
        } else {
            match part {
                "bold" => ansi.push_str("\x1b[1m"),
                "dim" => ansi.push_str("\x1b[2m"),
                "italic" | "italics" => ansi.push_str("\x1b[3m"),
                "underline" | "underscore" => ansi.push_str("\x1b[4m"),
                "reverse" => ansi.push_str("\x1b[7m"),
                _ => {}
            }
        }
    }
    ansi
}

pub fn format_styled(text: &str, style: &str) -> String {
    let ansi = style_to_ansi(style);
    if ansi.is_empty() {
        text.to_string()
    } else {
        format!("{ansi}{text}\x1b[0m")
    }
}

// Extract styles from theme TOML
fn extract_custom_style(toml: &str, section: &str) -> Option<String> {
    let target = format!("[custom.{section}]");
    let mut in_section = false;
    for line in toml.lines() {
        let trimmed = line.trim();
        if trimmed == target {
            in_section = true;
            continue;
        }
        if in_section {
            if trimmed.starts_with('[') {
                break;
            }
            if let Some(rest) = trimmed.strip_prefix("style =") {
                return Some(rest.trim().trim_matches('"').trim_matches('\'').to_string());
            }
        }
    }
    None
}

fn extract_time_style(toml: &str) -> Option<String> {
    let mut in_section = false;
    for line in toml.lines() {
        let trimmed = line.trim();
        if trimmed == "[time]" {
            in_section = true;
            continue;
        }
        if in_section {
            if trimmed.starts_with('[') {
                break;
            }
            if let Some(rest) = trimmed.strip_prefix("style =") {
                return Some(rest.trim().trim_matches('"').trim_matches('\'').to_string());
            }
        }
    }
    None
}

pub fn render_preview_bar(theme: &Theme, prefix_active: bool, zoomed: bool) -> String {
    let prefix_style = extract_custom_style(&theme.left_toml, "prefix_active")
        .unwrap_or_else(|| "bg:#eb6f92 fg:#191724 bold".into());
    let normal_style = extract_custom_style(&theme.left_toml, "session_normal")
        .unwrap_or_else(|| "fg:#908caa".into());
    let active_style = extract_custom_style(&theme.center_toml, "window_active")
        .unwrap_or_else(|| "bg:#26233a fg:#ebbcba bold".into());
    let inactive_style = extract_custom_style(&theme.center_toml, "window_inactive")
        .unwrap_or_else(|| "fg:#6e6a86".into());
    let zoom_style = extract_custom_style(&theme.center_toml, "window_zoom")
        .unwrap_or_else(|| "fg:#f6c177".into());

    let time_style = extract_time_style(&theme.right_toml).unwrap_or_else(|| "fg:#9ccfd8".into());
    let host_style =
        extract_custom_style(&theme.right_toml, "host").unwrap_or_else(|| "fg:#31748f".into());
    let count_style = extract_custom_style(&theme.right_toml, "window_count")
        .unwrap_or_else(|| "fg:#c4a7e7".into());

    // Left status
    let session_text = if prefix_active {
        format_styled(" 󰇄 dev ", &prefix_style)
    } else {
        format_styled(" 󰇄 dev ", &normal_style)
    };

    // Center tabs
    let tab1 = format_styled(" 1:code ", &inactive_style);
    let zoom_text = if zoomed {
        format!(" {}", format_styled("🔍", &zoom_style))
    } else {
        String::new()
    };
    let tab2 = format_styled(&format!(" 2:server{zoom_text} "), &active_style);
    let tab3 = format_styled(" 3:logs ", &inactive_style);
    let sep = format_styled(&theme.window_separator, &inactive_style);
    let center = format!("{tab1}{sep}{tab2}{sep}{tab3}");

    // Right status
    let time_part = format_styled("14:32:05", &time_style);
    let host_part = format_styled("on laptop", &host_style);
    let count_part = format_styled("󰖲 3", &count_style);
    let right = format!("{time_part} {host_part} {count_part}");

    format!("{session_text} │ {center} │ {right}")
}

pub fn display_theme_preview(theme: &Theme) {
    let variant_pill = match theme.variant {
        ThemeVariant::Dark => "\x1b[48;2;40;44;52m\x1b[38;2;171;178;191m dark \x1b[0m",
        ThemeVariant::Light => "\x1b[48;2;220;225;232m\x1b[38;2;50;55;65m light \x1b[0m",
    };

    let custom_pill = if theme.is_custom {
        " \x1b[48;2;60;80;100m\x1b[38;2;180;220;255m custom \x1b[0m"
    } else {
        ""
    };

    println!(
        "\x1b[1m\x1b[38;2;97;175;239m◆\x1b[0m \x1b[1m{}\x1b[0m  \x1b[2m({})\x1b[0m  {}{}",
        theme.name, theme.id, variant_pill, custom_pill
    );
    println!("  \x1b[2mAuthor: {}\x1b[0m", theme.author);
    println!("  \x1b[3m{}\x1b[0m", theme.description);

    // Swatches
    if !theme.swatches.is_empty() {
        print!("  Palette: ");
        for swatch in &theme.swatches {
            let fg = hex_fg(&swatch.hex);
            print!("{fg}■\x1b[0m \x1b[2m{}\x1b[0m  ", swatch.name);
        }
        println!();
    }

    // Mockups
    println!("  \x1b[2m┌─ Status Bar Preview (Normal) ───────────────────────────────────┐\x1b[0m");
    println!("  │ {} │", render_preview_bar(theme, false, false));
    println!(
        "  \x1b[2m├─ Status Bar Preview (Prefix Key Active & Window Zoomed) ─────────┤\x1b[0m"
    );
    println!("  │ {} │", render_preview_bar(theme, true, true));
    println!("  \x1b[2m└─────────────────────────────────────────────────────────────────┘\x1b[0m");
    println!(
        "  \x1b[2mQuick apply: tmuxship apply --theme {}\x1b[0m\n",
        theme.id
    );
}

pub fn preview_themes(filter: Option<&str>) -> io::Result<()> {
    let mut stdout = io::stdout();
    writeln!(
        stdout,
        "\x1b[1m\x1b[38;2;137;180;250mtmuxship\x1b[0m \x1b[1mTheme Gallery & Preview\x1b[0m"
    )?;
    writeln!(
        stdout,
        "\x1b[2mExplore built-in themes for your tmux status bar\x1b[0m\n"
    )?;

    let themes = super::catalog::all_themes();
    let filter_lower = filter.map(|f| f.to_lowercase());

    let mut count = 0;
    for theme in themes {
        if let Some(ref query) = filter_lower {
            let matches = theme.id.to_lowercase().contains(query)
                || theme.name.to_lowercase().contains(query)
                || theme.variant.to_string().contains(query)
                || theme.author.to_lowercase().contains(query);
            if !matches {
                continue;
            }
        }
        display_theme_preview(&theme);
        count += 1;
    }

    if count == 0 {
        writeln!(
            stdout,
            "\x1b[33mNo themes matched filter: {:?}\x1b[0m",
            filter.unwrap_or("")
        )?;
    } else {
        writeln!(
            stdout,
            "\x1b[2mShowing {count} theme(s). To use a theme, run: tmuxship apply --theme <theme_id>\x1b[0m"
        )?;
    }

    Ok(())
}
