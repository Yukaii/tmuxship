# tmuxship

A thin Starship-to-tmux adapter that renders tmux-ready status lines from Starship JSON output.

## CLI

```
tmuxship left [--config path]
tmuxship right [--config path]
tmuxship center [--config path]
tmuxship full [--config path]
```

Use `--config` to force a specific Starship config file for that invocation. Without it, tmuxship resolves configs in this order:

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

### Automatic tmux variables

**By default**, tmuxship automatically fetches common tmux variables and exposes them to Starship as uppercase environment variables with a `TMUX_` prefix. This means you can use tmux state in your Starship config without any additional configuration:

```toml
# starship config - just use the variables directly!
[env_var.TMUX_SESSION_NAME]
format = " # $env_value"

[env_var.TMUX_WINDOW_INDEX]
format = " ⌘$env_value"

[env_var.TMUX_PANE_CURRENT_PATH]
format = " in $env_value"
```

The following variables are automatically available:

**Session**: `TMUX_SESSION_NAME`, `TMUX_SESSION_ID`, `TMUX_SESSION_CREATED`, `TMUX_SESSION_ATTACHED`, `TMUX_SESSION_WINDOWS`

**Window**: `TMUX_WINDOW_ID`, `TMUX_WINDOW_INDEX`, `TMUX_WINDOW_NAME`, `TMUX_WINDOW_ACTIVE`, `TMUX_WINDOW_FLAGS`, `TMUX_WINDOW_LAYOUT`, `TMUX_WINDOW_PANES`, `TMUX_WINDOW_WIDTH`, `TMUX_WINDOW_HEIGHT`, `TMUX_WINDOW_ZOOMED_FLAG`

**Pane**: `TMUX_PANE_ID`, `TMUX_PANE_INDEX`, `TMUX_PANE_TITLE`, `TMUX_PANE_CURRENT_PATH`, `TMUX_PANE_CURRENT_COMMAND`, `TMUX_PANE_PID`, `TMUX_PANE_WIDTH`, `TMUX_PANE_HEIGHT`, `TMUX_PANE_ACTIVE`, `TMUX_PANE_AT_TOP`, `TMUX_PANE_AT_BOTTOM`, `TMUX_PANE_AT_LEFT`, `TMUX_PANE_AT_RIGHT`

**Client**: `TMUX_CLIENT_PREFIX`, `TMUX_CLIENT_WIDTH`, `TMUX_CLIENT_HEIGHT`, `TMUX_CLIENT_TERMNAME`

**Host**: `TMUX_HOST`, `TMUX_HOST_SHORT`

### Custom tmux variables

If you need different variables or want to reduce overhead, set `TMUX_SHIP_TMUX_VARS` to a comma-separated list of tmux format names (without the `#{}` wrapper):

```
# Only fetch specific variables
set -g status-left '#(TMUX_SHIP_TMUX_VARS="session_name,window_index" tmuxship left)'
```

## Development

Run the Rust test suite with:

```
cargo test
```

## tmux status configuration

Example tmux config that uses tmuxship for status lines and window status, with immediate refresh on common events:

```tmux
set -g status on
set -g status-left-length 100
set -g status-right-length 200
set -g status-justify centre

# Set config paths as global env vars
setenv -g TMUX_SHIP_LEFT_CONFIG   "$HOME/.tmux/starship.toml"
setenv -g TMUX_SHIP_RIGHT_CONFIG  "$HOME/.tmux/.right.toml"
setenv -g TMUX_SHIP_CENTER_CONFIG "$HOME/.tmux/.center.toml"

# Statuslines via tmuxship + Starship (variables are auto-fetched)
set -g status-left  '#(tmuxship left)'
set -g status-right '#(tmuxship right)'

# Refresh status on events
set-hook -g client-session-changed 'refresh-client -S'
set-hook -g client-attached        'refresh-client -S'
set-hook -g pane-focus-in          'refresh-client -S'
set-hook -g window-pane-changed    'refresh-client -S'
set-hook -g window-layout-changed  'refresh-client -S'

# Window status via tmuxship center (variables are auto-fetched)
set -g window-status-separator " • "
set -g window-status-style "bg=default,fg=default"
set -g window-status-format        '#(tmuxship center)'
set -g window-status-current-format '#(tmuxship center)'

# Optional: low idle interval for clock/long-running data
set -g status-interval 2
```

Notes:
- Config paths are set once with `setenv -g` and read by tmuxship automatically via `TMUX_SHIP_<SIDE>_CONFIG`
- tmux variables are automatically fetched and available as `TMUX_*` environment variables in your Starship config
- Keep `tmuxship` on your `PATH` (or use an absolute path)
- Use a dedicated `~/.tmux/.center.toml` Starship config to format window entries with `TMUX_WINDOW_ACTIVE` and `TMUX_WINDOW_ZOOMED_FLAG` for active/zoomed styling
