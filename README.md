# tmuxship

[![Crates.io](https://img.shields.io/crates/v/tmuxship.svg)](https://crates.io/crates/tmuxship)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A thin [Starship](https://starship.rs) adapter for tmux that renders beautiful, customizable status bars.

Use the full power of Starship to style your tmux status bar, with automatic access to tmux session, window, pane, and client information as environment variables.

## Features

- **Full Starship support** — Use any Starship module, prompt, or theme in your tmux status bar
- **Automatic tmux context** — Session, window, pane, and client variables exposed as `TMUX_*` environment variables
- **Per-side configuration** — Separate Starship config files for left, right, and center (window) status
- **Prefix highlighting** — Style your status bar differently when the prefix key is active
- **Zero boilerplate** — No wrapper scripts or complex tmux format strings required

![tmux status bar rendered by tmuxship](screenshots/full-bar.svg)

## Quick Start

**1. Install tmuxship**

```bash
cargo install tmuxship
```

**2. Copy the example configs**

```bash
cp examples/starship.toml ~/.tmux/starship.toml
cp examples/.right.toml   ~/.tmux/.right.toml
cp examples/.center.toml  ~/.tmux/.center.toml
```

**3. Add to `~/.tmux.conf`**

```tmux
setenv -g TMUX_SHIP_LEFT_CONFIG   "$HOME/.tmux/starship.toml"
setenv -g TMUX_SHIP_RIGHT_CONFIG  "$HOME/.tmux/.right.toml"
setenv -g TMUX_SHIP_CENTER_CONFIG "$HOME/.tmux/.center.toml"

run-shell 'tmuxship apply'
```

**4. Reload tmux**

```bash
tmux source ~/.tmux.conf
```

See the [examples/](examples/) directory for complete configuration samples.

## Installation

### From crates.io

```bash
cargo install tmuxship
```

### From source

```bash
cargo install --path .
```

## Usage

### Commands

| Command | Description |
|---|---|
| `tmuxship left` | Render the left status segment |
| `tmuxship right` | Render the right status segment |
| `tmuxship center` | Render the window status segment |
| `tmuxship full` | Render all segments |
| `tmuxship emit-tmux-conf` | Print the generated tmux config |
| `tmuxship apply` | Apply the generated config to the running tmux server |

### Config Resolution

tmuxship resolves Starship config files in the following order (where `<side>` is `left`, `right`, or `center`):

1. `--config` flag
2. `TMUX_SHIP_<SIDE>_CONFIG` (e.g. `TMUX_SHIP_LEFT_CONFIG`)
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

### Available tmux Variables

All tmux variables are automatically injected into the Starship environment with a `TMUX_` prefix.

**Session**

| Variable | Description |
|---|---|
| `TMUX_SESSION_NAME` | Current session name |
| `TMUX_SESSION_ID` | Session ID |
| `TMUX_SESSION_CREATED` | Session creation timestamp |
| `TMUX_SESSION_ATTACHED` | Number of attached clients |
| `TMUX_SESSION_WINDOWS` | Number of windows |

**Window**

| Variable | Description |
|---|---|
| `TMUX_WINDOW_ID` | Window ID |
| `TMUX_WINDOW_INDEX` | Window index |
| `TMUX_WINDOW_NAME` | Window name |
| `TMUX_WINDOW_ACTIVE` | `1` if active, `0` otherwise |
| `TMUX_WINDOW_FLAGS` | Window flags |
| `TMUX_WINDOW_LAYOUT` | Window layout |
| `TMUX_WINDOW_PANES` | Number of panes |
| `TMUX_WINDOW_WIDTH` / `TMUX_WINDOW_HEIGHT` | Window dimensions |
| `TMUX_WINDOW_ZOOMED_FLAG` | `1` if zoomed, `0` otherwise |

**Pane**

| Variable | Description |
|---|---|
| `TMUX_PANE_ID` | Pane ID |
| `TMUX_PANE_INDEX` | Pane index |
| `TMUX_PANE_TITLE` | Pane title |
| `TMUX_PANE_CURRENT_PATH` | Current working directory |
| `TMUX_PANE_CURRENT_COMMAND` | Running command |
| `TMUX_PANE_PID` | Process ID |
| `TMUX_PANE_WIDTH` / `TMUX_PANE_HEIGHT` | Pane dimensions |
| `TMUX_PANE_ACTIVE` | `1` if active, `0` otherwise |
| `TMUX_PANE_AT_TOP` / `TMUX_PANE_AT_BOTTOM` / `TMUX_PANE_AT_LEFT` / `TMUX_PANE_AT_RIGHT` | Edge position flags |

**Client**

| Variable | Description |
|---|---|
| `TMUX_CLIENT_PREFIX` | `1` if prefix key is active, `0` otherwise |
| `TMUX_CLIENT_WIDTH` / `TMUX_CLIENT_HEIGHT` | Terminal dimensions |
| `TMUX_CLIENT_TERMNAME` | Terminal name |

**Host**

| Variable | Description |
|---|---|
| `TMUX_HOST` | Full hostname |
| `TMUX_HOST_SHORT` | Short hostname |

## Examples

### Full status bar

![tmux status bar rendered by tmuxship](screenshots/full-bar.svg)

*Left: session name · Center: window list (active highlighted) · Right: time, host, window count*

### Left status — session with prefix highlighting

| Normal | Prefix active |
|---|---|
| ![left normal](screenshots/left-normal.svg) | ![left prefix](screenshots/left-prefix.svg) |

The session name is subtle in normal state and gets a bright green background when you press the tmux prefix key.

`starship.toml` (set via `TMUX_SHIP_LEFT_CONFIG`):
```toml
"$schema" = 'https://starship.rs/config-schema.json'
format = "$custom"
add_newline = false

# Shown when the prefix key is active
[custom.prefix_active]
when = 'test "${TMUX_CLIENT_PREFIX:-0}" = "1"'
command = 'printf "%s" "${TMUX_SESSION_NAME}"'
format = "[$output]($style) "
style = "bg:#95E6CB bold"

# Shown in normal state
[custom.session_normal]
when = 'test "${TMUX_CLIENT_PREFIX:-0}" != "1"'
command = 'printf "%s" "${TMUX_SESSION_NAME}"'
format = "[$output]($style) "
style = "fg:#565B66"
```

### Center — window list with active/inactive styling

| Active | Inactive | Zoomed |
|---|---|---|
| ![window active](screenshots/window-active.svg) | ![window inactive](screenshots/window-inactive.svg) | ![window zoomed](screenshots/window-zoom.svg) |

Active windows get a bold, highlighted style. Inactive windows are muted. Zoomed windows show an indicator.

`.center.toml` (set via `TMUX_SHIP_CENTER_CONFIG`):
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

# Optional: show an icon when the active window is zoomed
[custom.window_zoom]
when = 'test "${TMUX_WINDOW_ACTIVE:-0}" = "1" && test "${TMUX_WINDOW_ZOOMED_FLAG:-0}" = "1"'
command = 'printf "🔍"'
format = " $output"
```

### Right — time, host, and window count

![right status](screenshots/right.svg)

`.right.toml` (set via `TMUX_SHIP_RIGHT_CONFIG`):
```toml
"$schema" = 'https://starship.rs/config-schema.json'

format = "$time$custom"
add_newline = false

[time]
disabled = false
format = "[$time]($style) "
style = "fg:#89B4FA"
time_format = "%H:%M:%S"

[custom.host]
when = "true"
shell = "bash"
command = 'printf "%s" "${TMUX_HOST_SHORT:-$(hostname -s)}"'
format = "on [$output]($style) "
style = "fg:#A6E3A1"

[custom.window_count]
when = "true"
shell = "bash"
command = 'printf "%s" "${TMUX_SESSION_WINDOWS}"'
format = "[󰖲 $output]($style)"
style = "fg:#CBA6F7"
```

### Advanced left — session, git branch, and directory

![advanced left](screenshots/left-advanced.svg)

Combine built-in Starship modules with custom tmux-aware modules for a rich left status.

`advanced-left.toml`:
```toml
"$schema" = 'https://starship.rs/config-schema.json'

format = "$custom$directory$git_branch$git_status"
add_newline = false

# Session name with prefix highlighting
[custom.session_prefix]
shell = "bash"
when = 'test "${TMUX_CLIENT_PREFIX:-0}" = "1"'
command = 'printf "󰇄 %s" "${TMUX_SESSION_NAME:-?}"'
format = "[$output]($style) "
style = "bg:#95E6CB fg:#1E1E2E bold"

[custom.session_normal]
shell = "bash"
when = 'test "${TMUX_CLIENT_PREFIX:-0}" != "1"'
command = 'printf "󰇄 %s" "${TMUX_SESSION_NAME:-?}"'
format = "[$output]($style) "
style = "fg:#565B66"

# Current directory (from the active pane's CWD)
[directory]
format = "[$path]($style) "
style = "fg:#89B4FA"
truncation_length = 3
truncate_to_repo = true

# Git branch
[git_branch]
format = "on [$symbol$branch]($style) "
symbol = " "
style = "fg:#A6E3A1"

# Git status indicators
[git_status]
format = '([\[$all_status$ahead_behind\]]($style) )'
style = "fg:#F9E2AF"
```

See [examples/](examples/) for all available configuration samples.

## tmux Configuration

A complete `~/.tmux.conf` example using tmuxship:

```tmux
set -g status on
set -g status-left-length 100
set -g status-right-length 200
set -g status-justify centre
set -g focus-events on

# Config paths for each status segment
setenv -g TMUX_SHIP_LEFT_CONFIG   "$HOME/.tmux/starship.toml"
setenv -g TMUX_SHIP_RIGHT_CONFIG  "$HOME/.tmux/.right.toml"
setenv -g TMUX_SHIP_CENTER_CONFIG "$HOME/.tmux/.center.toml"
setenv -g TMUX_SHIP_WINDOW_SEPARATOR " • "

# Generate status-left, status-right, and window-status options from Starship configs.
# Styles come from Starship TOML; tmux-native format strings (#S, #I, #W) render the values.
run-shell 'tmuxship apply'

# Refresh the status bar on relevant tmux events
set-hook -g client-session-changed 'refresh-client -S'
set-hook -g client-attached        'refresh-client -S'
set-hook -g client-focus-in        'refresh-client -S'
set-hook -g pane-focus-in          'refresh-client -S'
set-hook -g window-pane-changed    'refresh-client -S'
set-hook -g window-layout-changed  'refresh-client -S'

set -g window-status-style "bg=default,fg=default"

# Optional: lower interval for clock or frequently changing data
set -g status-interval 2
```

**Key points:**

- Config paths are set once with `setenv -g` and picked up automatically by tmuxship
- `tmuxship apply` generates tmux options from Starship custom module styles, keeping color definitions in TOML
- Runtime `#(tmuxship right)` still injects `TMUX_*` environment variables for shell-driven modules
- Keep `tmuxship` on your `PATH`, or use an absolute path in your tmux config

### Generated Config vs. Runtime Rendering

`tmuxship apply` generates static tmux options at startup. For segments that need live data (battery, git status, etc.), use runtime rendering instead:

```tmux
set -g status-right '#(tmuxship right)'
set -g window-status-format         '#(TMUX_SHIP_TARGET="#{window_id}" tmuxship center)'
set -g window-status-current-format '#(TMUX_SHIP_TARGET="#{window_id}" tmuxship center)'
```

Set `TMUX_SHIP_TARGET` to a tmux format string like `#{window_id}` so tmuxship queries data for the correct window rather than the active one.

To inspect what `tmuxship apply` would generate before applying:

```bash
tmuxship emit-tmux-conf
```

**Recognized custom module names for generated config:**

- `prefix_active` and `session_normal` (left config) → generates `status-left` around tmux-native `#S`
- `window_active`, `window_inactive`, `window_zoom` (center config) → generates window status around `#I`, `#W`, and `#{window_zoomed_flag}`; separator text is controlled by `TMUX_SHIP_WINDOW_SEPARATOR`
- Right config is always runtime-rendered as `#(tmuxship right)`

## Advanced Configuration

### Limiting Fetched Variables

By default, tmuxship fetches all common tmux variables. To reduce overhead, specify only the variables you need:

```tmux
set -g status-left '#(TMUX_SHIP_TMUX_VARS="session_name,window_index" tmuxship left)'
```

### How It Works

1. tmux invokes tmuxship via `#(tmuxship left)` (or another side)
2. tmuxship queries tmux and exposes variables as `TMUX_*` environment variables
3. Starship is executed with the resolved config file
4. ANSI color codes from Starship output are converted to tmux format strings
5. The result is written to stdout for tmux to render

## Contributing

Contributions are welcome. Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

### Development

```bash
cargo test
```

#### Generating Screenshots

Screenshots are generated from Starship ANSI output using [ansisvg](https://github.com/wader/ansisvg):

```bash
# Install ansisvg (requires Go)
go install github.com/wader/ansisvg@latest

# Generate screenshots from example configs
./scripts/generate-screenshots.sh
```

Output SVGs are placed in `screenshots/`.

## License

MIT
