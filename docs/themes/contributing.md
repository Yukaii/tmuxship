# Contributing a Theme

Contributing a new theme to tmuxship requires creating just **one single file**.

## 1. Create a Theme File

Create a file named `themes/<theme-id>.toml`:

```toml
# Metadata
name = "My Theme"
description = "A clean and aesthetic status bar"
author = "Your Name"
variant = "dark"           # "dark" or "light"
window_separator = " • "

[[swatches]]
name = "Primary"
hex = "#89b4fa"

[[swatches]]
name = "Accent"
hex = "#cba6f7"

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

## 2. Test Locally

Test your theme in your terminal:

```bash
cargo run -- theme preview my-theme
```

## 3. Submit a Pull Request

Open a Pull Request on GitHub. Once merged, your theme will automatically be compiled into tmuxship and published to the online marketplace!
