# Configuration

tmuxship supports both **Single Unified Configuration (`tmuxship.toml`)** and **Per-Side Files**.

## Single Unified Configuration (`tmuxship.toml`)

Instead of maintaining multiple separate files, define your left status, center window tabs, and right status in a single file:

```toml
# ~/.tmux/tmuxship.toml or ~/.config/tmux/tmuxship.toml
name = "My Custom Theme"
window_separator = " • "

# --- Left Status ---
[left]
format = "$custom"
add_newline = false

[left.custom.prefix_active]
when = 'test "${TMUX_CLIENT_PREFIX:-0}" = "1"'
command = 'printf "󰇄 %s" "${TMUX_SESSION_NAME:-tmux}"'
format = "[$output]($style) "
style = "bg:#eb6f92 fg:#191724 bold"

[left.custom.session_normal]
when = 'test "${TMUX_CLIENT_PREFIX:-0}" != "1"'
command = 'printf "󰇄 %s" "${TMUX_SESSION_NAME:-tmux}"'
format = "[$output]($style) "
style = "fg:#908caa"

# --- Center Status (Window Tabs) ---
[center]
format = "$custom"
add_newline = false

[center.custom.window_active]
when = 'test "${TMUX_WINDOW_ACTIVE:-0}" = "1"'
command = 'printf "%s:%s" "${TMUX_WINDOW_INDEX:-1}" "${TMUX_WINDOW_NAME:-sh}"'
format = "[$output]($style)"
style = "bg:#26233a fg:#ebbcba bold"

[center.custom.window_inactive]
when = 'test "${TMUX_WINDOW_ACTIVE:-0}" != "1"'
command = 'printf "%s:%s" "${TMUX_WINDOW_INDEX:-1}" "${TMUX_WINDOW_NAME:-sh}"'
format = "[$output]($style)"
style = "fg:#6e6a86"

[center.custom.window_zoom]
when = 'test "${TMUX_WINDOW_ACTIVE:-0}" = "1" && test "${TMUX_WINDOW_ZOOMED_FLAG:-0}" = "1"'
command = 'printf "🔍"'
format = " $output"
style = "fg:#f6c177"

# --- Right Status ---
[right]
format = "$time$custom"
add_newline = false

[right.time]
disabled = false
format = "[$time]($style) "
style = "fg:#9ccfd8"
time_format = "%H:%M:%S"

[right.custom.host]
when = "true"
shell = "bash"
command = 'printf "%s" "${TMUX_HOST_SHORT:-$(hostname -s)}"'
format = "on [$output]($style) "
style = "fg:#31748f"

[right.custom.window_count]
when = "true"
shell = "bash"
command = 'printf "%s" "${TMUX_SESSION_WINDOWS:-1}"'
format = "[󰖲 $output]($style)"
style = "fg:#c4a7e7"
```

## Config Resolution Order

When tmuxship resolves configuration files for each side, it follows this priority:

1. `--config <path>` command-line flag
2. `TMUX_SHIP_<SIDE>_CONFIG` environment variable (e.g. `TMUX_SHIP_LEFT_CONFIG`)
3. `TMUX_SHIP_CONFIG` environment variable (points to unified `tmuxship.toml`)
4. `STARSHIP_CONFIG` environment variable
5. `--theme <name>` flag or `TMUX_SHIP_THEME` environment variable
6. `$XDG_CONFIG_HOME/tmuxship/tmuxship.toml` or `$XDG_CONFIG_HOME/tmux/tmuxship.toml`
7. `$XDG_CONFIG_HOME/tmux/.<side>.toml` or `starship.toml`
8. `$XDG_CONFIG_HOME/starship/.<side>.toml` or `starship.toml`
9. `$HOME/.config/tmuxship/tmuxship.toml` or `$HOME/.config/tmux/tmuxship.toml`
10. `$HOME/.tmux/tmuxship.toml`
11. `$HOME/.config/tmux/.<side>.toml` or `starship.toml`
12. `$HOME/.tmux/.<side>.toml` or `starship.toml`
13. `$HOME/.config/starship/.<side>.toml` or `starship.toml`
