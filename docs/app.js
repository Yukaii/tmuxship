// State
let themes = [];
let currentTheme = null;
let prefixActive = false;
let windowZoomed = false;
let activeWindowIndex = 2;
let sessionName = "dev";
let currentCodeTab = "tmux";
let activeFilter = "all";

// DOM Elements
const themeSelect = document.getElementById("theme-select");
const prefixToggle = document.getElementById("prefix-toggle");
const zoomToggle = document.getElementById("zoom-toggle");
const sessionInput = document.getElementById("session-input");
const statusLeft = document.getElementById("status-left");
const statusCenter = document.getElementById("status-center");
const statusRight = document.getElementById("status-right");
const themeMetaName = document.getElementById("theme-meta-name");
const themeMetaBadge = document.getElementById("theme-meta-badge");
const themeMetaDesc = document.getElementById("theme-meta-desc");
const themeMetaAuthor = document.getElementById("theme-meta-author");
const paletteRow = document.getElementById("palette-row");
const codeBlock = document.getElementById("code-block");
const themeGrid = document.getElementById("theme-grid");
const themeToggleBtn = document.getElementById("theme-toggle-btn");
const toastContainer = document.getElementById("toast-container");

// Toast Notification
function showToast(message, icon = "✔") {
  const toast = document.createElement("div");
  toast.className = "toast";
  toast.innerHTML = `<span>${icon}</span> <span>${message}</span>`;
  toastContainer.appendChild(toast);
  setTimeout(() => {
    toast.style.opacity = "0";
    toast.style.transform = "translateX(100%)";
    toast.style.transition = "all 0.3s ease";
    setTimeout(() => toast.remove(), 300);
  }, 2400);
}

// Copy Helper
function copyToClipboard(text, message = "Copied to clipboard!") {
  navigator.clipboard.writeText(text).then(() => {
    showToast(message);
  }).catch(() => {
    // Fallback
    const textarea = document.createElement("textarea");
    textarea.value = text;
    document.body.appendChild(textarea);
    textarea.select();
    document.execCommand("copy");
    textarea.remove();
    showToast(message);
  });
}

// Parse starship style string into CSS properties
function styleToCss(styleStr) {
  if (!styleStr) return {};
  const css = {};
  const parts = styleStr.split(/\s+/);
  for (const part of parts) {
    if (part.startsWith("fg:")) {
      css.color = part.substring(3);
    } else if (part.startsWith("bg:")) {
      css.backgroundColor = part.substring(3);
    } else if (part === "bold") {
      css.fontWeight = "700";
    } else if (part === "dim") {
      css.opacity = "0.7";
    } else if (part === "italic" || part === "italics") {
      css.fontStyle = "italic";
    } else if (part === "underline" || part === "underscore") {
      css.textDecoration = "underline";
    }
  }
  return css;
}

function applyCss(element, cssObj) {
  for (const [key, value] of Object.entries(cssObj)) {
    element.style[key] = value;
  }
}

// Extract style from TOML string
function extractStyle(toml, section) {
  if (!toml) return "";
  const lines = toml.split("\n");
  let inSection = false;
  const target = `[custom.${section}]`;
  for (const line of lines) {
    const trimmed = line.trim();
    if (trimmed === target || (section === "time" && trimmed === "[time]")) {
      inSection = true;
      continue;
    }
    if (inSection) {
      if (trimmed.startsWith("[")) break;
      if (trimmed.startsWith("style =")) {
        return trimmed.replace("style =", "").trim().replace(/^["']|["']$/g, "");
      }
    }
  }
  return "";
}

// Update Rendered Status Bar
function updateStatusBar() {
  if (!currentTheme) return;

  // 1. Left segment
  const prefixStyle = styleToCss(extractStyle(currentTheme.left_toml, "prefix_active") || "bg:#eb6f92 fg:#191724 bold");
  const normalStyle = styleToCss(extractStyle(currentTheme.left_toml, "session_normal") || "fg:#908caa");

  statusLeft.innerHTML = "";
  const pill = document.createElement("span");
  pill.className = "status-pill";
  pill.textContent = ` 󰇄 ${sessionName} `;
  if (prefixActive) {
    applyCss(pill, prefixStyle);
  } else {
    applyCss(pill, normalStyle);
  }
  statusLeft.appendChild(pill);

  // 2. Center segment (Window tabs)
  const activeStyle = styleToCss(extractStyle(currentTheme.center_toml, "window_active") || "bg:#26233a fg:#ebbcba bold");
  const inactiveStyle = styleToCss(extractStyle(currentTheme.center_toml, "window_inactive") || "fg:#6e6a86");
  const zoomStyle = styleToCss(extractStyle(currentTheme.center_toml, "window_zoom") || "fg:#f6c177");

  statusCenter.innerHTML = "";
  const windows = [
    { index: 1, name: "code" },
    { index: 2, name: "server" },
    { index: 3, name: "logs" },
    { index: 4, name: "tests" },
  ];

  windows.forEach((win, idx) => {
    if (idx > 0) {
      const sep = document.createElement("span");
      sep.className = "tab-sep";
      sep.textContent = currentTheme.window_separator || " • ";
      applyCss(sep, inactiveStyle);
      statusCenter.appendChild(sep);
    }

    const tab = document.createElement("span");
    tab.className = `tab-item ${win.index === activeWindowIndex ? "tab-active" : ""}`;
    tab.textContent = `${win.index}:${win.name}`;
    
    if (win.index === activeWindowIndex) {
      applyCss(tab, activeStyle);
      if (windowZoomed) {
        const zoomIcon = document.createElement("span");
        zoomIcon.textContent = " 🔍";
        applyCss(zoomIcon, zoomStyle);
        tab.appendChild(zoomIcon);
      }
    } else {
      applyCss(tab, inactiveStyle);
    }

    tab.onclick = () => {
      activeWindowIndex = win.index;
      updateStatusBar();
    };

    statusCenter.appendChild(tab);
  });

  // 3. Right segment
  const timeStyle = styleToCss(extractStyle(currentTheme.right_toml, "time") || "fg:#9ccfd8");
  const hostStyle = styleToCss(extractStyle(currentTheme.right_toml, "host") || "fg:#31748f");
  const countStyle = styleToCss(extractStyle(currentTheme.right_toml, "window_count") || "fg:#c4a7e7");

  statusRight.innerHTML = "";

  const timeSpan = document.createElement("span");
  const now = new Date();
  timeSpan.textContent = now.toTimeString().split(" ")[0];
  applyCss(timeSpan, timeStyle);

  const hostSpan = document.createElement("span");
  hostSpan.textContent = "on laptop";
  applyCss(hostSpan, hostStyle);

  const countSpan = document.createElement("span");
  countSpan.textContent = "󰖲 4";
  applyCss(countSpan, countStyle);

  statusRight.appendChild(timeSpan);
  statusRight.appendChild(hostSpan);
  statusRight.appendChild(countSpan);
}

// Update Theme Details and Code Snippet
function updateThemeInfo() {
  if (!currentTheme) return;

  themeMetaName.textContent = currentTheme.name;
  themeMetaBadge.textContent = currentTheme.variant;
  themeMetaBadge.className = `badge badge-${currentTheme.variant}`;
  themeMetaDesc.textContent = currentTheme.description;
  themeMetaAuthor.textContent = `Author: ${currentTheme.author}`;

  // Swatches
  paletteRow.innerHTML = "";
  if (currentTheme.swatches) {
    currentTheme.swatches.forEach(swatch => {
      const pill = document.createElement("div");
      pill.className = "swatch-pill";
      pill.innerHTML = `
        <span class="swatch-color" style="background-color: ${swatch.hex}"></span>
        <span>${swatch.name}</span>
        <span style="color: var(--text-muted)">${swatch.hex}</span>
      `;
      pill.onclick = () => copyToClipboard(swatch.hex, `Copied ${swatch.name} (${swatch.hex})!`);
      paletteRow.appendChild(pill);
    });
  }

  updateCodeBlock();
  updateStatusBar();
}

function updateCodeBlock() {
  if (!currentTheme) return;

  if (currentCodeTab === "tmux") {
    codeBlock.textContent = `# ~/.tmux.conf
setenv -g TMUX_SHIP_THEME "${currentTheme.id}"

# Apply theme styles
run-shell 'tmuxship apply'

# Refresh hooks
set-hook -g client-session-changed 'refresh-client -S'
set-hook -g client-attached        'refresh-client -S'
set-hook -g pane-focus-in          'refresh-client -S'
set -g window-status-style "bg=default,fg=default"`;
  } else if (currentCodeTab === "starship") {
    codeBlock.textContent = currentTheme.left_toml || "# Left config";
  } else if (currentCodeTab === "center") {
    codeBlock.textContent = currentTheme.center_toml || "# Center config";
  } else if (currentCodeTab === "right") {
    codeBlock.textContent = currentTheme.right_toml || "# Right config";
  }
}

// Render Theme Marketplace Grid
function renderThemeGrid() {
  themeGrid.innerHTML = "";
  
  const filtered = themes.filter(theme => {
    if (activeFilter === "all") return true;
    if (activeFilter === "dark" || activeFilter === "light") {
      return theme.variant === activeFilter;
    }
    return theme.id.includes(activeFilter) || theme.name.toLowerCase().includes(activeFilter);
  });

  filtered.forEach(theme => {
    const card = document.createElement("div");
    card.className = `theme-card ${currentTheme && currentTheme.id === theme.id ? "active" : ""}`;

    const swatchesHtml = (theme.swatches || []).map(s => 
      `<span class="mini-swatch" style="background-color: ${s.hex}" title="${s.name}"></span>`
    ).join("");

    // Mini mockup
    const prefixStyle = styleToCss(extractStyle(theme.left_toml, "prefix_active"));
    const activeStyle = styleToCss(extractStyle(theme.center_toml, "window_active"));
    const timeStyle = styleToCss(extractStyle(theme.right_toml, "time"));

    card.innerHTML = `
      <div>
        <div class="theme-card-header">
          <span class="theme-card-name">${theme.name}</span>
          <span class="badge badge-${theme.variant}">${theme.variant}</span>
        </div>
        <p class="theme-card-desc">${theme.description}</p>
        <div class="mini-bar">
          <span style="background-color: ${prefixStyle.backgroundColor || '#89b4fa'}; color: ${prefixStyle.color || '#111'}; padding: 0 4px; border-radius: 2px; font-weight: 700">󰇄</span>
          <span style="background-color: ${activeStyle.backgroundColor || '#313244'}; color: ${activeStyle.color || '#fff'}; padding: 0 4px; border-radius: 2px">1:code</span>
          <span style="color: ${timeStyle.color || '#89b4fa'}">14:32</span>
        </div>
      </div>
      <div style="display: flex; align-items: center; justify-content: space-between; margin-top: 0.5rem">
        <div class="mini-swatches">${swatchesHtml}</div>
        <span style="font-size: 0.8rem; color: var(--accent); font-weight: 600">Select &rarr;</span>
      </div>
    `;

    card.onclick = () => {
      selectTheme(theme.id);
      window.scrollTo({ top: document.querySelector(".previewer-section").offsetTop - 80, behavior: "smooth" });
    };

    themeGrid.appendChild(card);
  });
}

function selectTheme(themeId) {
  const found = themes.find(t => t.id === themeId);
  if (found) {
    currentTheme = found;
    themeSelect.value = found.id;
    updateThemeInfo();
    renderThemeGrid();
  }
}

// Download Theme TOML Files
function downloadThemeFiles() {
  if (!currentTheme) return;
  
  const files = [
    { name: "starship.toml", content: currentTheme.left_toml },
    { name: ".center.toml", content: currentTheme.center_toml },
    { name: ".right.toml", content: currentTheme.right_toml },
    { name: ".full.toml", content: currentTheme.full_toml },
  ];

  files.forEach(file => {
    const blob = new Blob([file.content], { type: "text/plain" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = file.name;
    document.body.appendChild(a);
    a.click();
    a.remove();
    URL.revokeObjectURL(url);
  });

  showToast(`Downloaded config files for ${currentTheme.name}!`);
}

// Setup Event Listeners
function setupEventListeners() {
  themeSelect.onchange = (e) => selectTheme(e.target.value);
  
  prefixToggle.onchange = (e) => {
    prefixActive = e.target.checked;
    updateStatusBar();
  };

  zoomToggle.onchange = (e) => {
    windowZoomed = e.target.checked;
    updateStatusBar();
  };

  sessionInput.oninput = (e) => {
    sessionName = e.target.value || "dev";
    updateStatusBar();
  };

  document.querySelectorAll(".code-tab").forEach(tab => {
    tab.onclick = () => {
      document.querySelectorAll(".code-tab").forEach(t => t.classList.remove("active"));
      tab.classList.add("active");
      currentCodeTab = tab.dataset.tab;
      updateCodeBlock();
    };
  });

  document.querySelectorAll(".filter-btn").forEach(btn => {
    btn.onclick = () => {
      document.querySelectorAll(".filter-btn").forEach(b => b.classList.remove("active"));
      btn.classList.add("active");
      activeFilter = btn.dataset.filter;
      renderThemeGrid();
    };
  });

  // Dark/Light Theme Toggle for the Website
  themeToggleBtn.onclick = () => {
    const isLight = document.body.getAttribute("data-theme") === "light";
    if (isLight) {
      document.body.removeAttribute("data-theme");
      localStorage.setItem("tmuxship-theme", "dark");
    } else {
      document.body.setAttribute("data-theme", "light");
      localStorage.setItem("tmuxship-theme", "light");
    }
  };

  const savedTheme = localStorage.getItem("tmuxship-theme");
  if (savedTheme === "light") {
    document.body.setAttribute("data-theme", "light");
  }

  // Update clock every second
  setInterval(updateStatusBar, 1000);
}

// Initialize Application
async function init() {
  setupEventListeners();

  try {
    const res = await fetch("themes.json");
    if (res.ok) {
      themes = await res.json();
    }
  } catch (err) {
    console.warn("Could not fetch themes.json, using fallback", err);
  }

  if (!themes || themes.length === 0) {
    console.error("No themes loaded");
    return;
  }

  // Populate Select
  themeSelect.innerHTML = "";
  themes.forEach(t => {
    const opt = document.createElement("option");
    opt.value = t.id;
    opt.textContent = `${t.name} (${t.variant})`;
    themeSelect.appendChild(opt);
  });

  // Default theme
  selectTheme("rose-pine");
  renderThemeGrid();
}

window.addEventListener("DOMContentLoaded", init);
