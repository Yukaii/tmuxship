# Segment Customization & Powerline Styling

Beyond simple text and color theming, `tmuxship` lets you craft rich, modular **Powerline status segments** — similar to [rose-pine/tmux](https://github.com/rose-pine/tmux) and [catppuccin/tmux](https://github.com/catppuccin/tmux).

You can also use the interactive **[Online Theme & Segment Builder](/themes/)** to customize segments and copy ready-to-use configurations with a single click.

---

## Powerline Styles Overview

Powerline status bars use special glyphs from Nerd Fonts to create seamless connected shapes and pill capsules:

| Style | Separator Glyphs | Description |
|---|---|---|
| **Rounded Pills / Bubbles** | `` (`\uE0B6`) & `` (`\uE0B4`) | Catppuccin / Rosé Pine style standalone capsule badges |
| **Classic Powerline** | `` (`\uE0B0`) & `` (`\uE0B2`) | Traditional triangle arrows connecting adjacent blocks |
| **Slanted / Angled** | `` (`\uE0B8`) & `` (`\uE0BA`) | Modern slanted diagonal separators |
| **Clean Minimalist** | `•` (`\u2022`) or `│` (`\u2502`) | Flat spaced badges with subtle delimiters |
| **Solid Blocks** | `█` (`\u2588`) | Flush contiguous rectangular blocks |

---

## Powerline Examples

### 1. Rounded Pills / Bubbles (`` / ``)

Similar to `catppuccin/tmux` and `rose-pine/tmux` pill segments:

```toml
# ~/.tmux/tmuxship.toml
name = "Catppuccin Rounded"
window_separator = " "

[left]
format = "$custom"
add_newline = false

[left.custom.prefix_active]
when = 'test "${TMUX_CLIENT_PREFIX:-0}" = "1"'
command = 'printf "#[bg=#f38ba8,fg=#11111b,bold] 󰇄 %s #[bg=default,fg=#f38ba8]" "${TMUX_SESSION_NAME:-tmux}"'
format = "$output "

[left.custom.session_normal]
when = 'test "${TMUX_CLIENT_PREFIX:-0}" != "1"'
command = 'printf "#[bg=#89b4fa,fg=#11111b,bold] 󰇄 %s #[bg=default,fg=#89b4fa]" "${TMUX_SESSION_NAME:-tmux}"'
format = "$output "

[center]
format = "$custom"
add_newline = false

[center.custom.window_active]
when = 'test "${TMUX_WINDOW_ACTIVE:-0}" = "1"'
command = 'printf "#[bg=#cba6f7,fg=#11111b,bold] #I #[bg=#313244,fg=#cdd6f4,bold] #W #[bg=default,fg=#313244]"'
format = "$output"

[center.custom.window_inactive]
when = 'test "${TMUX_WINDOW_ACTIVE:-0}" != "1"'
command = 'printf "#[bg=#45475a,fg=#bac2de] #I #[bg=#181825,fg=#6c7086] #W #[bg=default,fg=#181825]"'
format = "$output"

[right]
format = "$custom"
add_newline = false

[right.custom.host]
when = "true"
shell = "bash"
command = 'printf "#[bg=#45475a,fg=#b4befe] 󰒋 %s #[bg=default,fg=#45475a]" "${TMUX_HOST_SHORT:-$(hostname -s)}"'
format = "$output "

[right.custom.datetime]
when = "true"
shell = "bash"
command = 'printf "#[bg=#fab387,fg=#11111b,bold] 󱑂 %s #[bg=default,fg=#fab387]" "$(date +%H:%M)"'
format = "$output"
```

---

### 2. Slanted / Angled Segments (`` / ``)

Modern diagonal powerline styling:

```toml
[left.custom.session_normal]
when = 'test "${TMUX_CLIENT_PREFIX:-0}" != "1"'
command = 'printf "#[bg=#7aa2f7,fg=#1a1b26,bold] 󰇄 %s #[bg=default,fg=#7aa2f7]" "${TMUX_SESSION_NAME:-tmux}"'
format = "$output "

[center.custom.window_active]
when = 'test "${TMUX_WINDOW_ACTIVE:-0}" = "1"'
command = 'printf "#[bg=#bb9af7,fg=#1a1b26,bold] #I #[bg=#24283b,fg=#c0caf5,bold] #W #[bg=default,fg=#24283b]"'
format = "$output "

[right.custom.time]
when = "true"
shell = "bash"
command = 'printf "#[bg=#414868,fg=#7aa2f7]#[bg=#7aa2f7,fg=#1a1b26,bold] 󱑂 %s " "$(date +%H:%M)"'
format = "$output"
```

---

### 3. Classic Powerline Arrow (`` / ``)

Continuous airline-style arrows:

```toml
[left.custom.session_normal]
when = 'test "${TMUX_CLIENT_PREFIX:-0}" != "1"'
command = 'printf "#[bg=#31748f,fg=#e0def4,bold] 󰇄 %s #[bg=default,fg=#31748f]" "${TMUX_SESSION_NAME:-tmux}"'
format = "$output "

[right.custom.host]
when = "true"
shell = "bash"
command = 'printf "#[bg=default,fg=#26233a]#[bg=#26233a,fg=#9ccfd8] 󰒋 %s " "${TMUX_HOST_SHORT:-$(hostname -s)}"'
format = "$output"

[right.custom.time]
when = "true"
shell = "bash"
command = 'printf "#[bg=#26233a,fg=#f6c177]#[bg=#f6c177,fg=#191724,bold] 󱑂 %s " "$(date +%H:%M)"'
format = "$output"
```

---

## Modular Segment Recipes

Here is a collection of popular status segment modules you can mix and match:

### Working Directory (Path)
Displays the current pane's directory, abbreviating `$HOME` as `~`:
```toml
[left.custom.directory]
shell = "bash"
when = 'test -n "${TMUX_PANE_CURRENT_PATH}"'
command = 'p="${TMUX_PANE_CURRENT_PATH}"; p="${p/#$HOME/~}"; printf "󰉋 %s" "$p"'
format = "[$output]($style) "
style = "fg:#a6adc8"
```

### Git Branch Status
Shows the branch name when inside a git work tree:
```toml
[left.custom.git_branch]
shell = "bash"
when = 'test -n "${TMUX_PANE_CURRENT_PATH}" && git -C "${TMUX_PANE_CURRENT_PATH}" rev-parse --is-inside-work-tree >/dev/null 2>&1'
command = 'b=$(git -C "${TMUX_PANE_CURRENT_PATH}" branch --show-current 2>/dev/null); test -n "$b" && printf "󰊢 %s" "$b"'
format = "[$output]($style) "
style = "fg:#a6e3a1"
```

### Battery Percentage
Queries system battery on macOS (`pmset`) or Linux (`/sys/class/power_supply`):
```toml
[right.custom.battery]
when = "which pmset >/dev/null 2>&1 || test -d /sys/class/power_supply/BAT0"
shell = "bash"
command = 'pct=$(pmset -g batt 2>/dev/null | grep -o "[0-9]\\+%" | head -1 || cat /sys/class/power_supply/BAT0/capacity 2>/dev/null); test -n "$pct" && printf "󰁹 %s" "$pct"'
format = "[$output]($style) "
style = "fg:#a6e3a1"
```

### CPU Load
Shows 1-minute load average:
```toml
[right.custom.cpu]
when = "true"
shell = "bash"
command = 'load=$(uptime | awk -F "load average:" "{print \$2}" | cut -d, -f1 | tr -d " "); printf "󰍛 %s" "$load"'
format = "[$output]($style) "
style = "fg:#7dcfff"
```

---

## Interactive Builder

To visually compose, preview, and export powerline segments with any color palette, visit the **[Online Theme & Segment Builder](/themes/)**.
