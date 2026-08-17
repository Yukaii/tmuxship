# Getting Started

Get up and running with **tmuxship** in less than a minute.

## Prerequisites

- [tmux](https://github.com/tmux/tmux) 3.0+
- [Starship](https://starship.rs) installed on your `$PATH`
- [Rust](https://www.rust-lang.org/tools/install) (if installing via Cargo)

## 1. Install

Install tmuxship using Cargo:

```bash
cargo install tmuxship
```

## 2. Choose a Theme

You can preview all built-in themes in your terminal with real ANSI mockups:

```bash
tmuxship theme preview
```

Or preview a specific theme like Rosé Pine or Catppuccin:

```bash
tmuxship theme preview rose-pine
tmuxship theme preview catppuccin-mocha
```

## 3. Activate in tmux

Add the following lines to your `~/.tmux.conf`:

```sh
# Option A: Use a built-in theme directly (Zero configuration needed)
setenv -g TMUX_SHIP_THEME "rose-pine"

# Apply theme options on tmux startup
run-shell 'tmuxship apply'

# Event hooks for dynamic updates
set-hook -g client-session-changed 'refresh-client -S'
set-hook -g client-attached        'refresh-client -S'
set-hook -g pane-focus-in          'refresh-client -S'
set -g window-status-style "bg=default,fg=default"
```

*Or Option B: Use a single unified `tmuxship.toml` configuration:*

```sh
setenv -g TMUX_SHIP_CONFIG "$HOME/.tmux/tmuxship.toml"
run-shell 'tmuxship apply'
```

## 4. Reload tmux

Reload your configuration inside tmux:

```bash
tmux source ~/.tmux.conf
```
