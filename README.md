# tmux-ship

A thin Starship-to-tmux adapter that renders tmux-ready status lines from Starship JSON output.

## CLI

```
tmux-ship left [--config path]
tmux-ship right [--config path]
tmux-ship full [--config path]
```

Use `--config` to force a specific Starship config file for that invocation. Without it, tmux-ship resolves configs in this order:

1. `TMUX_SHIP_<SIDE>_CONFIG` for the requested side.
2. `STARSHIP_CONFIG` (applies to all sides).
3. `$XDG_CONFIG_HOME/starship/.<side>.toml`.
4. `$XDG_CONFIG_HOME/starship/starship.toml`.
5. `$HOME/.config/starship/.<side>.toml`.
6. `$HOME/.config/starship/starship.toml`.

### Rendering flow
1. Starship is executed with `starship prompt --output=json` under the resolved config path.
2. Modules from the JSON output are mapped to tmux markup (`#[fg=...,bg=...,bold]`).
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
