# tmuxship Themes Directory

This directory contains the built-in and community themes available in **tmuxship**.

## Directory Structure

Each theme lives in its own subdirectory under `themes/<theme-id>/`:

```
themes/
├── rose-pine/
│   ├── theme.toml      # Theme metadata, description, author, and color swatches
│   ├── starship.toml   # Left status Starship config
│   ├── .center.toml    # Center/window status Starship config
│   ├── .right.toml     # Right status Starship config
│   └── .full.toml      # Full status bar Starship config
├── catppuccin-mocha/
│   ├── theme.toml
│   ├── ...
```

## Theme Metadata Specification (`theme.toml`)

```toml
name = "Rosé Pine"
description = "All natural pine, faux fur and a bit of soho vibes for the classy minimalist"
author = "Rose Pine"
variant = "dark"           # "dark" or "light"
window_separator = " • "   # Default separator between window status items

[[swatches]]
name = "Love"
hex = "#eb6f92"

[[swatches]]
name = "Gold"
hex = "#f6c177"

[[swatches]]
name = "Rose"
hex = "#ebbcba"
```

## Contributing a New Theme

Contributing a theme to tmuxship is simple:

1. Fork the repository.
2. Create a new directory under `themes/<your-theme-id>/`.
3. Add `theme.toml`, `starship.toml`, `.center.toml`, `.right.toml`, and `.full.toml`.
4. Test your theme locally with `cargo run -- theme preview <your-theme-id>`.
5. Open a Pull Request!

## User Custom Themes

Users can also place custom themes in their local config directory:
- `$XDG_CONFIG_HOME/tmuxship/themes/<theme-id>/`
- `~/.config/tmuxship/themes/<theme-id>/`
- `~/.tmux/themes/<theme-id>/`

tmuxship will automatically discover and load them alongside built-in themes!
