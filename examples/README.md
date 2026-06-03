# Starship Configuration Examples

This directory contains example Starship configurations for use with tmuxship.

## Files

- **`starship.toml`** - Basic left status with session name and prefix highlighting
- **`.right.toml`** - Right status with time, host, and window count
- **`.center.toml`** - Window status with active/inactive styling and zoom indicator
- **`advanced-left.toml`** - Advanced left status with git integration

## Installation

Copy the example configs to your tmux config directory:

```bash
# Basic setup (left status only)
cp examples/starship.toml ~/.tmux/starship.toml

# Full setup (all three sides)
cp examples/starship.toml ~/.tmux/starship.toml
cp examples/.right.toml ~/.tmux/.right.toml
cp examples/.center.toml ~/.tmux/.center.toml
```

Or use the advanced left config:

```bash
cp examples/advanced-left.toml ~/.tmux/starship.toml
```

## Available Variables

All tmux variables are automatically exposed as `TMUX_*` environment variables:

### Session
- `TMUX_SESSION_NAME` - Current session name
- `TMUX_SESSION_ID` - Session ID
- `TMUX_SESSION_CREATED` - Creation timestamp
- `TMUX_SESSION_ATTACHED` - Number of attached clients
- `TMUX_SESSION_WINDOWS` - Number of windows

### Window
- `TMUX_WINDOW_ID` - Window ID
- `TMUX_WINDOW_INDEX` - Window index
- `TMUX_WINDOW_NAME` - Window name
- `TMUX_WINDOW_ACTIVE` - 1 if active, 0 otherwise
- `TMUX_WINDOW_FLAGS` - Window flags (e.g., `*`, `-`, `Z`)
- `TMUX_WINDOW_ZOOMED_FLAG` - 1 if zoomed, 0 otherwise
- `TMUX_WINDOW_PANES` - Number of panes

### Pane
- `TMUX_PANE_ID` - Pane ID
- `TMUX_PANE_INDEX` - Pane index
- `TMUX_PANE_CURRENT_PATH` - Current working directory
- `TMUX_PANE_CURRENT_COMMAND` - Current command running in pane
- `TMUX_PANE_ACTIVE` - 1 if active, 0 otherwise

### Client
- `TMUX_CLIENT_PREFIX` - 1 if prefix key pressed, 0 otherwise
- `TMUX_CLIENT_WIDTH` - Client width in columns
- `TMUX_CLIENT_HEIGHT` - Client height in rows

### Host
- `TMUX_HOST` - Full hostname
- `TMUX_HOST_SHORT` - Short hostname

## Tips

### Conditional Styling

Use `when` clauses to show different content based on tmux state:

```toml
[custom.example]
when = 'test "${TMUX_CLIENT_PREFIX:-0}" = "1"'
# Only shows when prefix key is pressed
```

### Custom Commands

Run shell commands to format tmux variables:

```toml
[custom.example]
command = 'printf "Session: %s" "${TMUX_SESSION_NAME}"'
format = "[$output]($style)"
```

### Color Schemes

The examples use Catppuccin Mocha colors. Adjust the color codes to match your theme:

- `#95E6CB` - Teal (prefix highlight)
- `#89B4FA` - Blue (info)
- `#A6E3A1` - Green (success)
- `#F9E2AF` - Yellow (warning)
- `#CBA6F7` - Mauve (accent)
- `#565B66` - Gray (muted)
- `#313244` - Dark gray (background)
- `#CDD6F4` - Light gray (foreground)

## tmux Configuration

Example tmux config to use these Starship configs. This uses `tmuxship apply` so tmux renders native values like session and window names synchronously, while tmuxship supplies styles from the Starship TOML files:

```tmux
# Enable status bar
set -g status on
set -g status-left-length 100
set -g status-right-length 200
set -g status-justify centre
set -g focus-events on

# Set config paths
setenv -g TMUX_SHIP_LEFT_CONFIG   "$HOME/.tmux/starship.toml"
setenv -g TMUX_SHIP_RIGHT_CONFIG  "$HOME/.tmux/.right.toml"
setenv -g TMUX_SHIP_CENTER_CONFIG "$HOME/.tmux/.center.toml"

# Generate status-left, status-right, and window status options
run-shell 'tmuxship apply'

# Generic window status options
set -g window-status-style "bg=default,fg=default"

# Refresh on events
set-hook -g client-session-changed 'refresh-client -S'
set-hook -g client-attached        'refresh-client -S'
set-hook -g client-focus-in        'refresh-client -S'
set-hook -g pane-focus-in          'refresh-client -S'
set-hook -g window-pane-changed    'refresh-client -S'

# Periodic refresh (optional)
set -g status-interval 2
```

`tmuxship apply` generates tmux options from recognized custom modules such as `prefix_active`, `session_normal`, `window_active`, `window_inactive`, and `window_zoom`. The right status remains runtime-rendered as `#(tmuxship right)` for shell-driven modules such as battery.

If you prefer runtime-rendered window status, pass the window ID so tmuxship queries the correct window:

```tmux
set -g window-status-format        '#(TMUX_SHIP_TARGET="#{window_id}" tmuxship center)'
set -g window-status-current-format '#(TMUX_SHIP_TARGET="#{window_id}" tmuxship center)'
```
