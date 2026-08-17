# How It Works

tmuxship offers two execution models: **Startup Generation (`tmuxship apply`)** and **Runtime Rendering (`#(tmuxship <side>)`)**.

## Execution Models Comparison

| Mode | How it works | Ideal for |
|---|---|---|
| **`tmuxship apply`** (Recommended) | Runs once at startup/reload. Converts Starship custom styles into native tmux format strings (`#S`, `#I`, `#W`, `#{window_zoomed_flag}`). | Session names, window tabs, host, colors — zero background processes, zero flicker. |
| **`#(tmuxship <side>)`** (Runtime) | Runs every `status-interval`. Queries tmux for environment variables, executes Starship, and converts ANSI escape sequences to tmux formats. | External live data (battery level, live git status, weather, remote APIs). |

## Hybrid Mode (The Best of Both Worlds)

You can mix both modes seamlessly: use `tmuxship apply` for fast, native left and window tabs, and override only `status-right` with runtime rendering:

```sh
# Generate native status-left and window-status options
run-shell 'tmuxship apply'

# Override right side with live runtime rendering
set -g status-right '#(tmuxship right)'
```
