# tmuxship

A thin Starship-to-tmux adapter that renders beautiful, customizable tmux status lines using Starship.

Use the full power of [Starship](https://starship.rs) to style your tmux status bar with access to all tmux session, window, pane, and client information.

## Features

- **Automatic tmux integration** - All tmux variables exposed as `TMUX_*` environment variables
- **Flexible configuration** - Use separate Starship configs for left, right, and center (window) status
- **Prefix highlighting** - Style your status bar differently when prefix key is pressed
- **Git integration** - Show git status in your tmux status bar
- **Zero boilerplate** - No wrapper scripts or complex tmux format strings needed

## Quick Start

1. Install tmuxship (see [Installation](#installation))

2. Copy example configs:
   ```bash
   cp examples/starship.toml ~/.tmux/starship.toml
   cp examples/.right.toml ~/.tmux/.right.toml
   cp examples/.center.toml ~/.tmux/.center.toml
   ```

3. Add to your `~/.tmux.conf`:
    ```tmux
    setenv -g TMUX_SHIP_LEFT_CONFIG   "$HOME/.tmux/starship.toml"
    setenv -g TMUX_SHIP_RIGHT_CONFIG  "$HOME/.tmux/.right.toml"
    setenv -g TMUX_SHIP_CENTER_CONFIG "$HOME/.tmux/.center.toml"

    run-shell 'tmuxship apply'
    ```

4. Reload tmux: `tmux source ~/.tmux.conf`

See the [examples/](examples/) directory for more configuration options.

## Installation

### From source (requires Rust)

```bash
cargo install --path .
```

## Usage

### Basic commands

```bash
tmuxship left    # Render left status
tmuxship right   # Render right status
tmuxship center  # Render window status
tmuxship full    # Render all sides
tmuxship emit-tmux-conf  # Print generated tmux-native config
tmuxship apply           # Apply generated tmux config to the current tmux server
```

### Configuration

tmuxship looks for Starship config files in this order:

1. `--config` flag (if provided)
2. `TMUX_SHIP_<SIDE>_CONFIG` environment variable
3. `STARSHIP_CONFIG` environment variable
4. Side-specific files: `~/.tmux/.<side>.toml`
5. Default: `~/.tmux/starship.toml`
6. Fallback to standard Starship config locations

See [Config Resolution](#config-resolution) for the complete resolution order.

### Available tmux variables

All tmux variables are automatically available in your Starship config with a `TMUX_` prefix:

**Session**
- `TMUX_SESSION_NAME` - Current session name
- `TMUX_SESSION_WINDOWS` - Number of windows
- `TMUX_SESSION_ATTACHED` - Number of attached clients

**Window**
- `TMUX_WINDOW_INDEX` - Window index
- `TMUX_WINDOW_NAME` - Window name
- `TMUX_WINDOW_ACTIVE` - `1` if active, `0` otherwise
- `TMUX_WINDOW_ZOOMED_FLAG` - `1` if zoomed, `0` otherwise

**Pane**
- `TMUX_PANE_CURRENT_PATH` - Current directory
- `TMUX_PANE_CURRENT_COMMAND` - Running command
- `TMUX_PANE_ACTIVE` - `1` if active, `0` otherwise

**Client**
- `TMUX_CLIENT_PREFIX` - `1` if prefix key pressed, `0` otherwise
- `TMUX_CLIENT_WIDTH` / `TMUX_CLIENT_HEIGHT` - Terminal dimensions

**Host**
- `TMUX_HOST` - Full hostname
- `TMUX_HOST_SHORT` - Short hostname

See [Available Variables](#all-available-variables) for the complete list.

## Examples

### Basic session name with prefix highlighting

```toml
"$schema" = 'https://starship.rs/config-schema.json'
format = "$custom"
add_newline = false

# Highlighted when prefix key is pressed
[custom.prefix_active]
when = 'test "${TMUX_CLIENT_PREFIX:-0}" = "1"'
command = 'printf "%s" "${TMUX_SESSION_NAME}"'
format = "[$output]($style) "
style = "bg:#95E6CB bold"

# Normal state
[custom.session_normal]
when = 'test "${TMUX_CLIENT_PREFIX:-0}" != "1"'
command = 'printf "%s" "${TMUX_SESSION_NAME}"'
format = "[$output]($style) "
style = "fg:#565B66"
```

### Window status with active highlighting

```toml
format = "$custom"
add_newline = false

[custom.window_active]
when = 'test "${TMUX_WINDOW_ACTIVE:-0}" = "1"'
command = 'printf "%s:%s" "${TMUX_WINDOW_INDEX}" "${TMUX_WINDOW_NAME}"'
format = "[$output]($style)"
style = "bg:#313244 fg:#CDD6F4 bold"

[custom.window_inactive]
when = 'test "${TMUX_WINDOW_ACTIVE:-0}" != "1"'
command = 'printf "%s:%s" "${TMUX_WINDOW_INDEX}" "${TMUX_WINDOW_NAME}"'
format = "[$output]($style)"
style = "fg:#6C7086"
```

See [examples/](examples/) for more complete configurations.

## tmux Configuration

Example tmux config that lets tmux render native values such as `#S`, `#I`, and `#W`, while tmuxship reads your Starship styles and applies the generated tmux options:

```tmux
set -g status on
set -g status-left-length 100
set -g status-right-length 200
set -g status-justify centre
set -g focus-events on

# Set config paths as global env vars
setenv -g TMUX_SHIP_LEFT_CONFIG   "$HOME/.tmux/starship.toml"
setenv -g TMUX_SHIP_RIGHT_CONFIG  "$HOME/.tmux/.right.toml"
setenv -g TMUX_SHIP_CENTER_CONFIG "$HOME/.tmux/.center.toml"

# Generate status-left, status-right, and window status options.
# This keeps tmux-native values synchronous while preserving Starship-managed styles.
run-shell 'tmuxship apply'

# Refresh status on events
set-hook -g client-session-changed 'refresh-client -S'
set-hook -g client-attached        'refresh-client -S'
set-hook -g client-focus-in        'refresh-client -S'
set-hook -g pane-focus-in          'refresh-client -S'
set-hook -g window-pane-changed    'refresh-client -S'
set-hook -g window-layout-changed  'refresh-client -S'

# Window status formats are generated by `tmuxship apply`; separator remains a tmux option.
set -g window-status-separator "#[default] • #[default]"
set -g window-status-style "bg=default,fg=default"

# Optional: low idle interval for clock/long-running data
set -g status-interval 2
```

### Key points

- Config paths are set once with `setenv -g` and read by tmuxship automatically
- `tmuxship apply` generates tmux-native options from recognized Starship custom module styles
- Runtime `#(tmuxship right)` still exposes tmux variables as `TMUX_*` environment variables
- Keep `tmuxship` on your `PATH` (or use an absolute path in tmux config)
- Use separate configs for each side to customize left/right/center status independently
- Use refresh hooks for runtime-rendered shell data such as battery and git status

### Generated tmux-native config

For tmux-native values such as session names, window names, and tmux time formats, prefer letting tmux render the value while tmuxship supplies styles from your Starship config:

```tmux
run-shell 'tmuxship apply'
```

To inspect the generated config before applying it:

```bash
tmuxship emit-tmux-conf
```

The generator currently recognizes these custom module names:

- `prefix_active` and `session_normal` from the left config to generate `status-left` around tmux-native `#S`
- `window_active`, `window_inactive`, and static `window_zoom` from the center config to generate window status around tmux-native `#I`, `#W`, and `#{window_zoomed_flag}`
- the right config remains runtime-rendered as `#(tmuxship right)` for shell-driven modules such as battery

The generated config is equivalent to writing tmux options by hand, but keeps the colors and styles in your Starship TOML. For example, `prefix_active` and `session_normal` generate a `status-left` that wraps tmux-native `#S` instead of asking Starship to print `${TMUX_SESSION_NAME}`.

### Runtime rendering fallback

If you need arbitrary Starship modules for a status segment, continue using runtime rendering:

```tmux
set -g status-right '#(tmuxship right)'
set -g window-status-format        '#(TMUX_SHIP_TARGET="#{window_id}" tmuxship center)'
set -g window-status-current-format '#(TMUX_SHIP_TARGET="#{window_id}" tmuxship center)'
```

Use `TMUX_SHIP_TARGET` to force tmuxship to query a specific tmux target, such as `#{window_id}` inside `window-status-format`.

## Advanced Configuration

### Custom tmux variables

By default, tmuxship fetches common tmux variables. To reduce overhead or fetch specific variables, use `TMUX_SHIP_TMUX_VARS`:

```tmux
# Only fetch specific variables
set -g status-left '#(TMUX_SHIP_TMUX_VARS="session_name,window_index" tmuxship left)'
```

### Targeting specific tmux contexts

`tmuxship` uses tmux's default formatting context when querying variables. In places like `window-status-format`, that default is the *active* window, so every entry can show the same data. Override the target by setting `TMUX_SHIP_TARGET` to a tmux format such as `#{window_id}` or `#{pane_id}`:

```tmux
set -g window-status-format        '#(TMUX_SHIP_TARGET="#{window_id}" tmuxship center)'
set -g window-status-current-format '#(TMUX_SHIP_TARGET="#{window_id}" tmuxship center)'
```

When tmux renders each window, `#{window_id}` is expanded before the command runs, so tmuxship gets data for the correct window and session.

### Config resolution

Complete config resolution order (where `<side>` is `left`, `right`, or `center`):

1. `--config` command line flag
2. `TMUX_SHIP_<SIDE>_CONFIG` (e.g., `TMUX_SHIP_LEFT_CONFIG`)
3. `STARSHIP_CONFIG`
4. `$XDG_CONFIG_HOME/tmux/.<side>.toml`
5. `$XDG_CONFIG_HOME/tmux/starship.toml`
6. `$XDG_CONFIG_HOME/starship/.<side>.toml`
7. `$XDG_CONFIG_HOME/starship/starship.toml`
8. `$HOME/.config/tmux/.<side>.toml`
9. `$HOME/.config/tmux/starship.toml`
10. `$HOME/.tmux/.<side>.toml`
11. `$HOME/.tmux/starship.toml`
12. `$HOME/.config/starship/.<side>.toml`
13. `$HOME/.config/starship/starship.toml`

### All available variables

**Session**: `TMUX_SESSION_NAME`, `TMUX_SESSION_ID`, `TMUX_SESSION_CREATED`, `TMUX_SESSION_ATTACHED`, `TMUX_SESSION_WINDOWS`

**Window**: `TMUX_WINDOW_ID`, `TMUX_WINDOW_INDEX`, `TMUX_WINDOW_NAME`, `TMUX_WINDOW_ACTIVE`, `TMUX_WINDOW_FLAGS`, `TMUX_WINDOW_LAYOUT`, `TMUX_WINDOW_PANES`, `TMUX_WINDOW_WIDTH`, `TMUX_WINDOW_HEIGHT`, `TMUX_WINDOW_ZOOMED_FLAG`

**Pane**: `TMUX_PANE_ID`, `TMUX_PANE_INDEX`, `TMUX_PANE_TITLE`, `TMUX_PANE_CURRENT_PATH`, `TMUX_PANE_CURRENT_COMMAND`, `TMUX_PANE_PID`, `TMUX_PANE_WIDTH`, `TMUX_PANE_HEIGHT`, `TMUX_PANE_ACTIVE`, `TMUX_PANE_AT_TOP`, `TMUX_PANE_AT_BOTTOM`, `TMUX_PANE_AT_LEFT`, `TMUX_PANE_AT_RIGHT`

**Client**: `TMUX_CLIENT_PREFIX`, `TMUX_CLIENT_WIDTH`, `TMUX_CLIENT_HEIGHT`, `TMUX_CLIENT_TERMNAME`

**Host**: `TMUX_HOST`, `TMUX_HOST_SHORT`

### How it works

1. tmuxship is invoked by tmux (via `#(tmuxship left)`)
2. tmuxship fetches tmux variables and sets them as `TMUX_*` environment variables
3. Starship is executed with the resolved config file
4. ANSI color codes from Starship output are converted to tmux format strings
5. The result is written to stdout for tmux to render

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

### Development

Run the test suite:

```bash
cargo test
```

## License

MIT
