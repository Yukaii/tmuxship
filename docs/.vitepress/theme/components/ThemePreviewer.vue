<template>
  <div class="theme-previewer">
    <div class="previewer-terminal">
      <div class="terminal-topbar">
        <div class="terminal-dots">
          <span class="terminal-dot dot-red"></span>
          <span class="terminal-dot dot-yellow"></span>
          <span class="terminal-dot dot-green"></span>
        </div>
        <span class="terminal-title-text">tmuxship status previewer — tmux 3.4</span>
        <span style="font-size: 0.75rem; color: var(--vp-c-text-3);">Interactive</span>
      </div>

      <!-- Live Simulator Screen -->
      <div class="terminal-screen">
        <div class="sim-status-bar">
          <!-- Left segment -->
          <div class="status-left-seg">
            <span class="sim-pill" :style="prefixActive ? prefixActiveStyle : sessionNormalStyle">
              󰇄 {{ sessionName }}
            </span>
          </div>

          <!-- Center segment -->
          <div class="status-center-seg" style="display: flex; align-items: center; gap: 0.35rem;">
            <template v-for="(win, idx) in windows" :key="win.index">
              <span v-if="idx > 0" class="sim-sep" :style="windowInactiveStyle">
                {{ currentTheme.window_separator || ' • ' }}
              </span>
              <span
                class="sim-tab"
                :class="{ 'sim-tab-active': win.index === activeWindowIndex }"
                :style="win.index === activeWindowIndex ? windowActiveStyle : windowInactiveStyle"
                @click="activeWindowIndex = win.index"
              >
                {{ win.index }}:{{ win.name }}
                <span v-if="win.index === activeWindowIndex && windowZoomed" :style="windowZoomStyle"> 🔍</span>
              </span>
            </template>
          </div>

          <!-- Right segment -->
          <div class="status-right-seg" style="display: flex; align-items: center; gap: 0.5rem;">
            <span :style="timeStyle">{{ currentTime }}</span>
            <span :style="hostStyle">on laptop</span>
            <span :style="countStyle">󰖲 4</span>
          </div>
        </div>
      </div>

      <!-- Controls Bar -->
      <div class="previewer-controls">
        <div class="control-left">
          <label style="font-size: 0.85rem; font-weight: 600;">Theme:</label>
          <select v-model="selectedThemeId" class="theme-dropdown" @change="onThemeChange">
            <option v-for="t in themes" :key="t.id" :value="t.id">
              {{ t.name }} ({{ t.variant }})
            </option>
          </select>

          <label class="preview-toggle">
            <input type="checkbox" v-model="prefixActive" />
            <span>Prefix Active (⌃B)</span>
          </label>

          <label class="preview-toggle">
            <input type="checkbox" v-model="windowZoomed" />
            <span>Window Zoomed (🔍)</span>
          </label>
        </div>

        <div class="control-right">
          <label style="font-size: 0.8rem; color: var(--vp-c-text-2);">Session:</label>
          <input
            v-model="sessionName"
            style="background: var(--vp-c-bg); border: 1px solid var(--vp-c-border); color: var(--vp-c-text-1); padding: 0.25rem 0.5rem; border-radius: 4px; font-family: var(--vp-font-family-mono); width: 80px; font-size: 0.8rem;"
          />
        </div>
      </div>
    </div>

    <!-- Theme Details & Shiki Highlighted Single-File Config -->
    <div class="theme-card-details">
      <div>
        <div class="theme-header-row">
          <h3 class="theme-title" style="margin: 0;">{{ currentTheme.name }}</h3>
          <span class="theme-badge" :class="currentTheme.variant">{{ currentTheme.variant }}</span>
        </div>
        <p class="theme-desc-text">{{ currentTheme.description }}</p>
        <p style="font-size: 0.8rem; color: var(--vp-c-text-3); margin-bottom: 0.75rem;">
          Author: {{ currentTheme.author }}
        </p>

        <div style="font-size: 0.8rem; font-weight: 600; margin-bottom: 0.35rem;">Color Palette (click to copy):</div>
        <div class="swatches-flex">
          <div
            v-for="swatch in currentTheme.swatches"
            :key="swatch.name"
            class="swatch-item"
            :title="`Copy ${swatch.hex}`"
            @click="copyText(swatch.hex, `Copied ${swatch.name} (${swatch.hex})!`)"
          >
            <span class="swatch-box" :style="{ backgroundColor: swatch.hex }"></span>
            <span>{{ swatch.name }}</span>
            <span style="color: var(--vp-c-text-3); font-size: 0.75rem;">{{ swatch.hex }}</span>
          </div>
        </div>

        <div class="actions-row">
          <button class="btn-preview primary" @click="copySnippet">
            <span>📋 Copy Config</span>
          </button>
          <button class="btn-preview" @click="downloadTheme">
            <span>⬇ Download {{ currentTheme.id }}.toml</span>
          </button>
        </div>
      </div>

      <!-- Code Viewer with Shiki -->
      <div class="code-viewer">
        <div class="code-viewer-header">
          <div class="code-tabs-nav">
            <button
              class="code-tab-btn"
              :class="{ active: currentTab === 'tmux' }"
              @click="currentTab = 'tmux'"
            >
              ~/.tmux.conf
            </button>
            <button
              class="code-tab-btn"
              :class="{ active: currentTab === 'unified' }"
              @click="currentTab = 'unified'"
            >
              tmuxship.toml (Unified)
            </button>
          </div>
          <button
            class="btn-preview"
            style="padding: 0.2rem 0.5rem; font-size: 0.75rem;"
            @click="copySnippet"
          >
            Copy
          </button>
        </div>
        <div class="code-viewer-body" v-html="highlightedCode"></div>
      </div>
    </div>

    <!-- Theme Gallery Cards -->
    <div style="margin-top: 2rem;">
      <h3 style="font-size: 1.2rem; font-weight: 600; margin-bottom: 0.5rem;">Explore All Themes</h3>
      <div class="theme-cards-grid">
        <div
          v-for="t in themes"
          :key="t.id"
          class="theme-grid-card"
          :class="{ active: t.id === currentTheme.id }"
          @click="selectTheme(t.id)"
        >
          <div>
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.25rem;">
              <span style="font-weight: 600; font-size: 0.95rem;">{{ t.name }}</span>
              <span class="theme-badge" :class="t.variant">{{ t.variant }}</span>
            </div>
            <p style="font-size: 0.8rem; color: var(--vp-c-text-2); min-height: 2.2em; display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden; margin-bottom: 0.5rem;">
              {{ t.description }}
            </p>
          </div>

          <div style="display: flex; align-items: center; justify-content: space-between;">
            <div style="display: flex; gap: 3px;">
              <span
                v-for="s in (t.swatches || []).slice(0, 5)"
                :key="s.name"
                style="width: 12px; height: 12px; border-radius: 2px;"
                :style="{ backgroundColor: s.hex }"
              ></span>
            </div>
            <span style="font-size: 0.8rem; color: var(--vp-c-brand-1); font-weight: 600;">Select &rarr;</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onUnmounted, watch } from 'vue';
import { useData } from 'vitepress';
import { createHighlighter } from 'shiki';
import rawThemes from '../../../public/themes.json';

const { isDark } = useData();

const themes = ref(rawThemes || []);
const selectedThemeId = ref('rose-pine');
const prefixActive = ref(false);
const windowZoomed = ref(false);
const activeWindowIndex = ref(2);
const sessionName = ref('dev');
const currentTab = ref('tmux');
const currentTime = ref('');
const highlightedCode = ref('');

let highlighter = null;
let timer = null;

const windows = [
  { index: 1, name: 'code' },
  { index: 2, name: 'server' },
  { index: 3, name: 'logs' },
  { index: 4, name: 'tests' },
];

const currentTheme = computed(() => {
  return themes.value.find(t => t.id === selectedThemeId.value) || themes.value[0] || {};
});

function parseStyleString(styleStr) {
  if (!styleStr) return {};
  const css = {};
  const parts = styleStr.split(/\s+/);
  for (const part of parts) {
    if (part.startsWith('fg:')) {
      css.color = part.substring(3);
    } else if (part.startsWith('bg:')) {
      css.backgroundColor = part.substring(3);
    } else if (part === 'bold') {
      css.fontWeight = '700';
    } else if (part === 'italic' || part === 'italics') {
      css.fontStyle = 'italic';
    } else if (part === 'underline') {
      css.textDecoration = 'underline';
    }
  }
  return css;
}

function extractStyle(toml, section) {
  if (!toml) return '';
  const lines = toml.split('\n');
  let inSection = false;
  const target = `[custom.${section}]`;
  const altTarget = `[${section}]`;
  for (const line of lines) {
    const trimmed = line.trim();
    if (trimmed.includes(target) || trimmed.includes(altTarget) || (section === 'time' && trimmed.includes('[time]')) || (trimmed.endsWith(`custom.${section}]`))) {
      inSection = true;
      continue;
    }
    if (inSection) {
      if (trimmed.startsWith('[') && !trimmed.includes(section)) break;
      if (trimmed.startsWith('style =')) {
        return trimmed.replace('style =', '').trim().replace(/^["']|["']$/g, '');
      }
    }
  }
  return '';
}

const prefixActiveStyle = computed(() => {
  return parseStyleString(extractStyle(currentTheme.value.left_toml || currentTheme.value.unified_toml, 'prefix_active') || 'bg:#eb6f92 fg:#191724 bold');
});

const sessionNormalStyle = computed(() => {
  return parseStyleString(extractStyle(currentTheme.value.left_toml || currentTheme.value.unified_toml, 'session_normal') || 'fg:#908caa');
});

const windowActiveStyle = computed(() => {
  return parseStyleString(extractStyle(currentTheme.value.center_toml || currentTheme.value.unified_toml, 'window_active') || 'bg:#26233a fg:#ebbcba bold');
});

const windowInactiveStyle = computed(() => {
  return parseStyleString(extractStyle(currentTheme.value.center_toml || currentTheme.value.unified_toml, 'window_inactive') || 'fg:#6e6a86');
});

const windowZoomStyle = computed(() => {
  return parseStyleString(extractStyle(currentTheme.value.center_toml || currentTheme.value.unified_toml, 'window_zoom') || 'fg:#f6c177');
});

const timeStyle = computed(() => {
  return parseStyleString(extractStyle(currentTheme.value.right_toml || currentTheme.value.unified_toml, 'time') || 'fg:#9ccfd8');
});

const hostStyle = computed(() => {
  return parseStyleString(extractStyle(currentTheme.value.right_toml || currentTheme.value.unified_toml, 'host') || 'fg:#31748f');
});

const countStyle = computed(() => {
  return parseStyleString(extractStyle(currentTheme.value.right_toml || currentTheme.value.unified_toml, 'window_count') || 'fg:#c4a7e7');
});

function selectTheme(themeId) {
  selectedThemeId.value = themeId;
  updateHighlight();
}

function onThemeChange() {
  updateHighlight();
}

const rawCodeToHighlight = computed(() => {
  if (currentTab.value === 'tmux') {
    return `# ~/.tmux.conf
setenv -g TMUX_SHIP_THEME "${currentTheme.value.id}"

# Apply theme styles
run-shell 'tmuxship apply'

# Dynamic refresh hooks
set-hook -g client-session-changed 'refresh-client -S'
set-hook -g client-attached        'refresh-client -S'
set-hook -g pane-focus-in          'refresh-client -S'
set -g window-status-style "bg=default,fg=default"`;
  }
  return currentTheme.value.unified_toml || `# ${currentTheme.value.name}\n${currentTheme.value.left_toml}`;
});

async function updateHighlight() {
  const code = rawCodeToHighlight.value;
  const lang = currentTab.value === 'tmux' ? 'bash' : 'toml';
  const theme = isDark.value ? 'vitesse-dark' : 'vitesse-light';

  if (!highlighter) {
    highlighter = await createHighlighter({
      themes: ['vitesse-dark', 'vitesse-light'],
      langs: ['toml', 'bash', 'shell']
    });
  }

  highlightedCode.value = highlighter.codeToHtml(code, { lang, theme });
}

function copyText(text, message = 'Copied to clipboard!') {
  navigator.clipboard.writeText(text);
  alert(message);
}

function copySnippet() {
  navigator.clipboard.writeText(rawCodeToHighlight.value);
  alert('Copied snippet to clipboard!');
}

function downloadTheme() {
  const content = currentTheme.value.unified_toml;
  const blob = new Blob([content], { type: 'text/plain' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = `${currentTheme.value.id}.toml`;
  document.body.appendChild(a);
  a.click();
  a.remove();
  URL.revokeObjectURL(url);
}

function updateTime() {
  currentTime.value = new Date().toTimeString().split(' ')[0];
}

watch([currentTab, isDark, selectedThemeId], () => {
  updateHighlight();
});

onMounted(async () => {
  updateTime();
  timer = setInterval(updateTime, 1000);
  await updateHighlight();
});

onUnmounted(() => {
  if (timer) clearInterval(timer);
});
</script>
