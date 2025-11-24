# tmux-ship

A thin Starship-to-tmux adapter that renders tmux-ready status lines from Starship JSON output.

## CLI

```
tmux-ship left [--config path]
tmux-ship right [--config path]
tmux-ship center [--config path]
tmux-ship full [--config path]
```

Use `--config` to force a specific Starship config file for that invocation. Without it, tmux-ship resolves configs in this order:

1. `TMUX_SHIP_<SIDE>_CONFIG` for the requested side.
2. `STARSHIP_CONFIG` (applies to all sides).
3. `$XDG_CONFIG_HOME/tmux/.<side>.toml`.
4. `$XDG_CONFIG_HOME/tmux/starship.toml`.
5. `$XDG_CONFIG_HOME/starship/.<side>.toml`.
6. `$XDG_CONFIG_HOME/starship/starship.toml`.
7. `$HOME/.config/tmux/.<side>.toml`.
8. `$HOME/.config/tmux/starship.toml`.
9. `$HOME/.tmux/.<side>.toml`.
10. `$HOME/.tmux/starship.toml`.
11. `$HOME/.config/starship/.<side>.toml`.
12. `$HOME/.config/starship/starship.toml`.

### Rendering flow
1. Starship is executed with `starship prompt` under the resolved config path.
2. ANSI styles from Starship output are translated to tmux markup (`#[fg=...,bg=...,bold]`).
3. The concatenated result is written to stdout for use in `status-left`, `status-right`, or `status-format` slots.

### Passing tmux variables to Starship

When you want Starship segments to react to tmux state (session name, window index, etc.), set
`TMUX_SHIP_TMUX_VARS` to a comma-separated list of tmux format names (without the `#{}` wrapper). tmux-ship
will fetch their values via `tmux display-message -p -F` and expose them to Starship as uppercase variables with a
`TMUX_` prefix.

Example:

```
# tmux status-left/right definition
set -g status-left '#(TMUX_SHIP_TMUX_VARS="session_name,window_index" tmux-ship left)'

# starship config snippet
[env_var.tmux_session]
variable = "TMUX_SESSION_NAME"
format = " [# $env_value]($style)"
```

## Development

Run the Rust test suite with:

```
cargo test
```

## tmux status configuration

Example tmux config that uses tmux-ship for status lines and window status, with immediate refresh on common events:

```tmux
set -g status on
set -g status-left-length 100
set -g status-right-length 200
set -g status-justify centre

# Set config paths as global env vars
setenv -g TMUX_SHIP_LEFT_CONFIG   "$HOME/.tmux/starship.toml"
setenv -g TMUX_SHIP_RIGHT_CONFIG  "$HOME/.tmux/.right.toml"
setenv -g TMUX_SHIP_CENTER_CONFIG "$HOME/.tmux/.center.toml"

# Statuslines via tmux-ship + Starship
set -g status-left  '#(TMUX_PREFIX_LABEL="#{?client_prefix,#{session_name},}" TMUX_SESSION_NAME="#{session_name}" tmux-ship left)'
set -g status-right '#(tmux-ship right)'

# Refresh status on events
set-hook -g client-session-changed 'refresh-client -S'
set-hook -g client-attached        'refresh-client -S'
set-hook -g pane-focus-in          'refresh-client -S'
set-hook -g window-pane-changed    'refresh-client -S'
set-hook -g window-layout-changed  'refresh-client -S'

# Window status via tmux-ship center
set -g window-status-separator " • "
set -g window-status-style "bg=default,fg=default"
set -g window-status-format        '#(TMUX_WINDOW_INDEX="#{window_index}" TMUX_WINDOW_NAME="#{window_name}" TMUX_WINDOW_ACTIVE="#{window_active}" TMUX_WINDOW_ZOOMED_FLAG="#{window_zoomed_flag}" tmux-ship center)'
set -g window-status-current-format '#(TMUX_WINDOW_INDEX="#{window_index}" TMUX_WINDOW_NAME="#{window_name}" TMUX_WINDOW_ACTIVE="1" TMUX_WINDOW_ZOOMED_FLAG="#{window_zoomed_flag}" tmux-ship center)'

# Optional: low idle interval for clock/long-running data
set -g status-interval 2
```

Notes:
- Config paths are set once with `setenv -g` and read by tmux-ship automatically via `TMUX_SHIP_<SIDE>_CONFIG`
- Pass tmux variables directly as environment variables (e.g., `TMUX_SESSION_NAME="#{session_name}"`)
- `window-status-current-format` hardcodes `TMUX_WINDOW_ACTIVE="1"` since it's always the active window
- Keep `tmux-ship` on your `PATH` (or use an absolute path)
- Use a dedicated `~/.tmux/.center.toml` Starship config to format window entries with `TMUX_WINDOW_ACTIVE` and `TMUX_WINDOW_ZOOMED_FLAG` for active/zoomed styling
