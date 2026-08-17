# CLI Reference

`tmuxship` provides rendering commands, configuration generators, and a built-in theme manager.

## Main Commands

| Command | Description |
|---|---|
| `tmuxship left` | Render the left status segment (supports `--theme <id>` and `--config <path>`) |
| `tmuxship right` | Render the right status segment (supports `--theme <id>` and `--config <path>`) |
| `tmuxship center` | Render the window status segment (supports `--theme <id>` and `--config <path>`) |
| `tmuxship full` | Render all segments |
| `tmuxship emit-tmux-conf` | Print generated tmux options to stdout (dry run) |
| `tmuxship apply` | Apply generated options directly to the running tmux server |
| `tmuxship theme` | Explore, preview, export, and install themes |
| `tmuxship init <theme>` | Print `~/.tmux.conf` snippet for quick activation |

## Theme Management Commands

```bash
# List all available built-in and user custom themes
tmuxship theme list
tmuxship theme list --json
tmuxship theme list --filter dark

# Preview themes with realistic ANSI terminal mockups
tmuxship theme preview
tmuxship theme preview rose-pine
tmuxship theme preview --filter light

# Show TOML configuration for a theme
tmuxship theme show rose-pine
tmuxship theme show catppuccin-mocha --side left

# Export theme configuration files to a folder
tmuxship theme export rose-pine --dir ./my-theme

# Install theme directly to ~/.tmux/
tmuxship theme install rose-pine

# Print tmux.conf snippet for a theme
tmuxship theme init rose-pine
```

## Global & Rendering Flags

- `--config <PATH>`: Force a specific Starship or unified `tmuxship.toml` config file.
- `--theme <THEME>`: Use a built-in or custom theme preset.
- `--show-config`: Print the resolved file path without rendering.
