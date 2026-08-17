# tmuxship Themes Directory

This directory contains the built-in themes available in **tmuxship**.

## Theme Format: Single-File (`<theme-id>.toml`)

Every theme is a single, self-contained TOML file located at `themes/<theme-id>.toml`:

```
themes/
├── rose-pine.toml
├── catppuccin-mocha.toml
├── tokyo-night.toml
├── nord.toml
└── ...
```

## Structure of a Theme File (`themes/my-theme.toml`)

```toml
# Metadata
name = "My Custom Theme"
description = "A sleek, modern status bar theme"
author = "Your Name"
variant = "dark"           # "dark" or "light"
window_separator = " • "   # Default separator between window status tabs

[[swatches]]
name = "Accent"
hex = "#89b4fa"

[[swatches]]
name = "Success"
hex = "#a6e3a1"

# Left Status (Session name & prefix highlighting)
[left]
format = "$custom"
add_newline = false

[left.custom.prefix_active]
when = 'test "${TMUX_CLIENT_PREFIX:-0}" = "1"'
command = 'printf "󰇄 %s" "${TMUX_SESSION_NAME:-tmux}"'
format = "[$output]($style) "
style = "bg:#89b4fa fg:#11111b bold"

[left.custom.session_normal]
when = 'test "${TMUX_CLIENT_PREFIX:-0}" != "1"'
command = 'printf "󰇄 %s" "${TMUX_SESSION_NAME:-tmux}"'
format = "[$output]($style) "
style = "fg:#6c7086"

# Center Status (Window tabs with active, inactive, and zoom indicators)
[center]
format = "$custom"
add_newline = false

[center.custom.window_active]
when = 'test "${TMUX_WINDOW_ACTIVE:-0}" = "1"'
command = 'printf "%s:%s" "${TMUX_WINDOW_INDEX:-1}" "${TMUX_WINDOW_NAME:-sh}"'
format = "[$output]($style)"
style = "bg:#313244 fg:#cdd6f4 bold"

[center.custom.window_inactive]
when = 'test "${TMUX_WINDOW_ACTIVE:-0}" != "1"'
command = 'printf "%s:%s" "${TMUX_WINDOW_INDEX:-1}" "${TMUX_WINDOW_NAME:-sh}"'
format = "[$output]($style)"
style = "fg:#6c7086"

[center.custom.window_zoom]
when = 'test "${TMUX_WINDOW_ACTIVE:-0}" = "1" && test "${TMUX_WINDOW_ZOOMED_FLAG:-0}" = "1"'
command = 'printf "🔍"'
format = " $output"
style = "fg:#f9e2af"

# Right Status (Time, host, window count, git)
[right]
format = "$time$custom"
add_newline = false

[right.time]
disabled = false
format = "[$time]($style) "
style = "fg:#89b4fa"
time_format = "%H:%M:%S"

[right.custom.host]
when = "true"
shell = "bash"
command = 'printf "%s" "${TMUX_HOST_SHORT:-$(hostname -s)}"'
format = "on [$output]($style) "
style = "fg:#a6e3a1"

[right.custom.window_count]
when = "true"
shell = "bash"
command = 'printf "%s" "${TMUX_SESSION_WINDOWS:-1}"'
format = "[󰖲 $output]($style)"
style = "fg:#cba6f7"
```

## Contributing a New Theme

1. Fork the repository.
2. Create a single file `themes/<your-theme-id>.toml`.
3. Preview and verify your theme locally:
   ```bash
   cargo run -- theme preview <your-theme-id>
   ```
4. Open a Pull Request!

## User Custom Themes

You can also place `.toml` theme files in your local config directory without recompiling:
- `$XDG_CONFIG_HOME/tmuxship/themes/<theme-id>.toml`
- `~/.config/tmuxship/themes/<theme-id>.toml`
- `~/.tmux/themes/<theme-id>.toml`
- Any folder specified in `$TMUX_SHIP_THEMES_DIR`
