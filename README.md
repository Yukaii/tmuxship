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

## Development

Run the Rust test suite with:

```
cargo test
```
