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
      text: Online Theme & Segment Builder
      link: /themes/
    - theme: alt
      text: Segment Examples
      link: /guide/segments
    - theme: alt
      text: Getting Started
      link: /guide/getting-started
    - theme: alt
      text: GitHub
      link: https://github.com/Yukaii/tmuxship

features:
  - icon: 🎨
    title: Legendary Themes & Builder
    details: 17+ presets (Rosé Pine, Catppuccin, Tokyo Night, Nord, Gruvbox, etc.) plus an interactive online theme & powerline segment builder.
  - icon: 📐
    title: Powerline Segments
    details: Customize rounded pills, classic arrows, and slanted segments with modular widgets like git, directory, battery, cpu, and clock.
  - icon: ⚡
    title: Synchronous & Flicker-Free
    details: tmuxship apply runs once at startup to generate native tmux status options, delivering zero CPU overhead and zero flicker.
  - icon: 🧩
    title: Full Starship Power
    details: Automatic injection of TMUX_* environment variables (session, window, pane, client, zoom, prefix) directly into Starship prompts.
---

<div style="margin-top: 3rem;">
  <h2 style="font-size: 1.8rem; font-weight: 700; text-align: center; margin-bottom: 0.5rem;">Online Theme & Segment Builder</h2>
  <p style="text-align: center; color: var(--vp-c-text-2); margin-bottom: 1.5rem;">Interactively customize powerline styles, toggle widgets, test themes, and copy ready-to-use configurations.</p>
  <ThemePreviewer />
</div>
