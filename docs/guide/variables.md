# Injected tmux Variables

All tmux properties are automatically queried and passed into Starship's environment with the `TMUX_` prefix.

## Session Variables

| Variable | Description |
|---|---|
| `TMUX_SESSION_NAME` | Name of the active session |
| `TMUX_SESSION_ID` | Session identifier |
| `TMUX_SESSION_CREATED` | Creation timestamp |
| `TMUX_SESSION_ATTACHED` | Number of attached clients |
| `TMUX_SESSION_WINDOWS` | Total number of windows in session |

## Window Variables

| Variable | Description |
|---|---|
| `TMUX_WINDOW_ID` | Window identifier |
| `TMUX_WINDOW_INDEX` | Window index (e.g. `1`, `2`) |
| `TMUX_WINDOW_NAME` | Window name (e.g. `zsh`, `nvim`) |
| `TMUX_WINDOW_ACTIVE` | `1` if window is currently focused, `0` otherwise |
| `TMUX_WINDOW_FLAGS` | Window status flags |
| `TMUX_WINDOW_ZOOMED_FLAG` | `1` if the active pane is zoomed, `0` otherwise |
| `TMUX_WINDOW_PANES` | Number of panes in window |

## Pane Variables

| Variable | Description |
|---|---|
| `TMUX_PANE_ID` | Pane identifier |
| `TMUX_PANE_INDEX` | Index of the pane |
| `TMUX_PANE_CURRENT_PATH` | Current working directory of pane process |
| `TMUX_PANE_CURRENT_COMMAND` | Current active command in pane |
| `TMUX_PANE_ACTIVE` | `1` if active pane, `0` otherwise |

## Client & Host Variables

| Variable | Description |
|---|---|
| `TMUX_CLIENT_PREFIX` | `1` if prefix key is active, `0` otherwise |
| `TMUX_CLIENT_WIDTH` / `TMUX_CLIENT_HEIGHT` | Terminal client dimensions |
| `TMUX_HOST` | Full hostname |
| `TMUX_HOST_SHORT` | Short hostname |
