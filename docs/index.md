---
layout: home

hero:
  name: "tmuxship"
  text: "Starship Status Bars for tmux"
  tagline: "Ultra-fast, flicker-free status lines with live preview and built-in themes for Rosé Pine, Catppuccin, Tokyo Night, Nord, Gruvbox, and more."
  image:
    src: "data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><text y='.9em' font-size='90'>🚀</text></svg>"
    alt: tmuxship logo
  actions:
    - theme: brand
      text: Live Theme Previewer
      link: /themes/
    - theme: alt
      text: Getting Started
      link: /guide/getting-started
    - theme: alt
      text: GitHub
      link: https://github.com/Yukaii/tmuxship

features:
  - icon: 🎨
    title: Legendary Built-in Themes
    details: 17+ presets ready out-of-the-box (Rosé Pine, Catppuccin, Tokyo Night, Nord, Gruvbox, Dracula, Kanagawa, One Dark, Solarized) with zero configuration.
  - icon: ⚡
    title: Synchronous & Flicker-Free
    details: tmuxship apply runs once at startup to generate native tmux status options, delivering zero CPU overhead and zero flicker.
  - icon: 📄
    title: Simple Unified Config
    details: Define everything in a single tmuxship.toml file with [left], [center], and [right] sections, or customize individual sides.
  - icon: 🧩
    title: Full Starship Power
    details: Automatic injection of TMUX_* environment variables (session, window, pane, client, zoom, prefix) directly into Starship prompts.
---

<div style="margin-top: 3rem;">
  <h2 style="font-size: 1.8rem; font-weight: 700; text-align: center; margin-bottom: 0.5rem;">Live Theme Previewer</h2>
  <p style="text-align: center; color: var(--vp-c-text-2); margin-bottom: 1.5rem;">Test themes, toggle prefix keys, switch window tabs, and preview single-file configurations.</p>
  <ThemePreviewer />
</div>
