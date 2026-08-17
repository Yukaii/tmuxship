<template>
  <div class="theme-previewer">
    <!-- Top Preset Quick-Selector Bar -->
    <div class="builder-quick-presets">
      <span class="preset-label">⚡ Quick Presets:</span>
      <div class="preset-buttons-group">
        <button
          v-for="p in quickPresets"
          :key="p.name"
          class="preset-chip"
          :class="{ active: selectedThemeId === p.themeId && powerlineStyle === p.style }"
          @click="applyQuickPreset(p)"
        >
          <span class="preset-chip-dot" :style="{ backgroundColor: p.dotColor }"></span>
          {{ p.name }}
        </button>
      </div>
    </div>

    <!-- Live Simulator Terminal Window -->
    <div class="previewer-terminal">
      <div class="terminal-topbar">
        <div class="terminal-dots">
          <span class="terminal-dot dot-red"></span>
          <span class="terminal-dot dot-yellow"></span>
          <span class="terminal-dot dot-green"></span>
        </div>
        <span class="terminal-title-text">tmuxship status bar simulator — tmux 3.4</span>
        <div class="terminal-badges">
          <span class="style-badge">{{ currentStyleLabel }}</span>
          <span class="theme-badge" :class="currentTheme.variant">{{ currentTheme.variant }}</span>
        </div>
      </div>

      <!-- Live Simulator Screen -->
      <div class="terminal-screen">
        <div class="sim-status-bar" :style="{ backgroundColor: palette.bgDark }">
          
          <!-- LEFT STATUS BAR -->
          <div class="status-left-seg">
            <!-- Session Segment -->
            <template v-if="showSession">
              <div v-if="powerlineStyle === 'rounded'" class="power-seg rounded">
                <span class="cap-left" :style="{ color: prefixActive ? palette.prefixColor : palette.primary }"></span>
                <span class="seg-body" :style="{ backgroundColor: prefixActive ? palette.prefixColor : palette.primary, color: palette.bgDark, fontWeight: 'bold' }">
                  󰇄 {{ sessionName }}
                </span>
                <span class="cap-right" :style="{ color: prefixActive ? palette.prefixColor : palette.primary }"></span>
              </div>
              <div v-else-if="powerlineStyle === 'arrow'" class="power-seg arrow">
                <span class="seg-body" :style="{ backgroundColor: prefixActive ? palette.prefixColor : palette.primary, color: palette.bgDark, fontWeight: 'bold' }">
                  󰇄 {{ sessionName }}
                </span>
                <span class="sep-arrow" :style="{ color: prefixActive ? palette.prefixColor : palette.primary, backgroundColor: showPath ? palette.bgSurface : 'transparent' }"></span>
              </div>
              <div v-else-if="powerlineStyle === 'slanted'" class="power-seg slanted">
                <span class="seg-body" :style="{ backgroundColor: prefixActive ? palette.prefixColor : palette.primary, color: palette.bgDark, fontWeight: 'bold' }">
                  󰇄 {{ sessionName }}
                </span>
                <span class="sep-slanted" :style="{ color: prefixActive ? palette.prefixColor : palette.primary, backgroundColor: showPath ? palette.bgSurface : 'transparent' }"></span>
              </div>
              <div v-else-if="powerlineStyle === 'block'" class="power-seg block">
                <span class="seg-body" :style="{ backgroundColor: prefixActive ? palette.prefixColor : palette.primary, color: palette.bgDark, fontWeight: 'bold' }">
                  󰇄 {{ sessionName }}
                </span>
              </div>
              <div v-else class="power-seg minimal">
                <span :style="{ color: prefixActive ? palette.prefixColor : palette.primary, fontWeight: 'bold' }">
                  󰇄 {{ sessionName }}
                </span>
              </div>
            </template>

            <!-- Working Directory Segment -->
            <template v-if="showPath">
              <div v-if="powerlineStyle === 'rounded'" class="power-seg rounded">
                <span class="cap-left" :style="{ color: palette.bgSurface }"></span>
                <span class="seg-body" :style="{ backgroundColor: palette.bgSurface, color: palette.teal }">
                  󰉋 {{ shortPath }}
                </span>
                <span class="cap-right" :style="{ color: palette.bgSurface }"></span>
              </div>
              <div v-else-if="powerlineStyle === 'arrow'" class="power-seg arrow">
                <span class="seg-body" :style="{ backgroundColor: palette.bgSurface, color: palette.teal }">
                  󰉋 {{ shortPath }}
                </span>
                <span class="sep-arrow" :style="{ color: palette.bgSurface, backgroundColor: showGit ? palette.bgMute : 'transparent' }"></span>
              </div>
              <div v-else-if="powerlineStyle === 'slanted'" class="power-seg slanted">
                <span class="seg-body" :style="{ backgroundColor: palette.bgSurface, color: palette.teal }">
                  󰉋 {{ shortPath }}
                </span>
                <span class="sep-slanted" :style="{ color: palette.bgSurface, backgroundColor: showGit ? palette.bgMute : 'transparent' }"></span>
              </div>
              <div v-else-if="powerlineStyle === 'block'" class="power-seg block">
                <span class="seg-body" :style="{ backgroundColor: palette.bgSurface, color: palette.teal }">
                  󰉋 {{ shortPath }}
                </span>
              </div>
              <div v-else class="power-seg minimal">
                <span :style="{ color: palette.teal }">󰉋 {{ shortPath }}</span>
              </div>
            </template>

            <!-- Git Branch Segment -->
            <template v-if="showGit">
              <div v-if="powerlineStyle === 'rounded'" class="power-seg rounded">
                <span class="cap-left" :style="{ color: palette.bgMute }"></span>
                <span class="seg-body" :style="{ backgroundColor: palette.bgMute, color: palette.green }">
                  󰊢 {{ gitBranch }}
                </span>
                <span class="cap-right" :style="{ color: palette.bgMute }"></span>
              </div>
              <div v-else-if="powerlineStyle === 'arrow'" class="power-seg arrow">
                <span class="seg-body" :style="{ backgroundColor: palette.bgMute, color: palette.green }">
                  󰊢 {{ gitBranch }}
                </span>
                <span class="sep-arrow" :style="{ color: palette.bgMute }"></span>
              </div>
              <div v-else-if="powerlineStyle === 'slanted'" class="power-seg slanted">
                <span class="seg-body" :style="{ backgroundColor: palette.bgMute, color: palette.green }">
                  󰊢 {{ gitBranch }}
                </span>
                <span class="sep-slanted" :style="{ color: palette.bgMute }"></span>
              </div>
              <div v-else-if="powerlineStyle === 'block'" class="power-seg block">
                <span class="seg-body" :style="{ backgroundColor: palette.bgMute, color: palette.green }">
                  󰊢 {{ gitBranch }}
                </span>
              </div>
              <div v-else class="power-seg minimal">
                <span :style="{ color: palette.green }">󰊢 {{ gitBranch }}</span>
              </div>
            </template>
          </div>

          <!-- CENTER STATUS BAR (WINDOW TABS) -->
          <div class="status-center-seg">
            <template v-for="(win, idx) in windowList" :key="win.index">
              <!-- Rounded Pill Tab -->
              <div
                v-if="powerlineStyle === 'rounded'"
                class="power-seg rounded clickable"
                :class="{ active: win.index === activeWindowIndex }"
                @click="activeWindowIndex = win.index"
              >
                <span
                  class="cap-left"
                  :style="{ color: win.index === activeWindowIndex ? palette.purple : palette.bgSurface }"
                ></span>
                <span
                  class="seg-body num-badge"
                  :style="{
                    backgroundColor: win.index === activeWindowIndex ? palette.purple : palette.bgSurface,
                    color: win.index === activeWindowIndex ? palette.bgDark : palette.fgMuted,
                    fontWeight: win.index === activeWindowIndex ? 'bold' : 'normal'
                  }"
                >
                  <span v-if="showWindowIcons" class="win-icon">{{ win.icon }} </span>{{ win.index }}
                </span>
                <span
                  class="seg-body title-badge"
                  :style="{
                    backgroundColor: win.index === activeWindowIndex ? palette.bgSurface : palette.bgMute,
                    color: win.index === activeWindowIndex ? palette.fgMain : palette.fgMuted,
                    fontWeight: win.index === activeWindowIndex ? 'bold' : 'normal'
                  }"
                >
                  {{ win.name }}
                  <span v-if="win.index === activeWindowIndex && windowZoomed" :style="{ color: palette.yellow }"> 🔍</span>
                </span>
                <span
                  class="cap-right"
                  :style="{ color: win.index === activeWindowIndex ? palette.bgSurface : palette.bgMute }"
                ></span>
              </div>

              <!-- Powerline Arrow Tab -->
              <div
                v-else-if="powerlineStyle === 'arrow'"
                class="power-seg arrow clickable"
                :class="{ active: win.index === activeWindowIndex }"
                @click="activeWindowIndex = win.index"
              >
                <span
                  class="seg-body"
                  :style="{
                    backgroundColor: win.index === activeWindowIndex ? palette.purple : palette.bgSurface,
                    color: win.index === activeWindowIndex ? palette.bgDark : palette.fgMuted,
                    fontWeight: win.index === activeWindowIndex ? 'bold' : 'normal'
                  }"
                >
                  <span v-if="showWindowIcons">{{ win.icon }} </span>{{ win.index }} {{ win.name }}
                  <span v-if="win.index === activeWindowIndex && windowZoomed" :style="{ color: palette.yellow }"> 🔍</span>
                </span>
                <span
                  class="sep-arrow"
                  :style="{
                    color: win.index === activeWindowIndex ? palette.purple : palette.bgSurface
                  }"
                ></span>
              </div>

              <!-- Slanted Tab -->
              <div
                v-else-if="powerlineStyle === 'slanted'"
                class="power-seg slanted clickable"
                :class="{ active: win.index === activeWindowIndex }"
                @click="activeWindowIndex = win.index"
              >
                <span
                  class="seg-body"
                  :style="{
                    backgroundColor: win.index === activeWindowIndex ? palette.purple : palette.bgSurface,
                    color: win.index === activeWindowIndex ? palette.bgDark : palette.fgMuted,
                    fontWeight: win.index === activeWindowIndex ? 'bold' : 'normal'
                  }"
                >
                  <span v-if="showWindowIcons">{{ win.icon }} </span>{{ win.index }} {{ win.name }}
                  <span v-if="win.index === activeWindowIndex && windowZoomed" :style="{ color: palette.yellow }"> 🔍</span>
                </span>
                <span
                  class="sep-slanted"
                  :style="{
                    color: win.index === activeWindowIndex ? palette.purple : palette.bgSurface
                  }"
                ></span>
              </div>

              <!-- Block Tab -->
              <div
                v-else-if="powerlineStyle === 'block'"
                class="power-seg block clickable"
                :class="{ active: win.index === activeWindowIndex }"
                @click="activeWindowIndex = win.index"
              >
                <span
                  class="seg-body"
                  :style="{
                    backgroundColor: win.index === activeWindowIndex ? palette.purple : palette.bgSurface,
                    color: win.index === activeWindowIndex ? palette.bgDark : palette.fgMuted,
                    fontWeight: win.index === activeWindowIndex ? 'bold' : 'normal'
                  }"
                >
                  <span v-if="showWindowIcons">{{ win.icon }} </span>{{ win.index }}:{{ win.name }}
                  <span v-if="win.index === activeWindowIndex && windowZoomed" :style="{ color: palette.yellow }"> 🔍</span>
                </span>
              </div>

              <!-- Minimalist Tab -->
              <div
                v-else
                class="power-seg minimal clickable"
                :class="{ active: win.index === activeWindowIndex }"
                @click="activeWindowIndex = win.index"
              >
                <span v-if="idx > 0" class="min-sep" :style="{ color: palette.fgMuted }"> • </span>
                <span
                  :style="{
                    color: win.index === activeWindowIndex ? palette.purple : palette.fgMuted,
                    fontWeight: win.index === activeWindowIndex ? 'bold' : 'normal',
                    textDecoration: win.index === activeWindowIndex ? 'underline' : 'none'
                  }"
                >
                  <span v-if="showWindowIcons">{{ win.icon }} </span>{{ win.index }}:{{ win.name }}
                  <span v-if="win.index === activeWindowIndex && windowZoomed" :style="{ color: palette.yellow }"> 🔍</span>
                </span>
              </div>
            </template>
          </div>

          <!-- RIGHT STATUS BAR -->
          <div class="status-right-seg">
            <!-- CPU Load Segment -->
            <template v-if="showCpu">
              <div v-if="powerlineStyle === 'rounded'" class="power-seg rounded">
                <span class="cap-left" :style="{ color: palette.bgSurface }"></span>
                <span class="seg-body" :style="{ backgroundColor: palette.bgSurface, color: palette.teal }">
                  󰍛 0.42
                </span>
                <span class="cap-right" :style="{ color: palette.bgSurface }"></span>
              </div>
              <div v-else-if="powerlineStyle === 'arrow'" class="power-seg arrow">
                <span class="sep-arrow-rev" :style="{ color: palette.bgSurface }"></span>
                <span class="seg-body" :style="{ backgroundColor: palette.bgSurface, color: palette.teal }">
                  󰍛 0.42
                </span>
              </div>
              <div v-else-if="powerlineStyle === 'slanted'" class="power-seg slanted">
                <span class="sep-slanted-rev" :style="{ color: palette.bgSurface }"></span>
                <span class="seg-body" :style="{ backgroundColor: palette.bgSurface, color: palette.teal }">
                  󰍛 0.42
                </span>
              </div>
              <div v-else-if="powerlineStyle === 'block'" class="power-seg block">
                <span class="seg-body" :style="{ backgroundColor: palette.bgSurface, color: palette.teal }">
                  󰍛 0.42
                </span>
              </div>
              <div v-else class="power-seg minimal">
                <span :style="{ color: palette.teal }">󰍛 0.42</span>
              </div>
            </template>

            <!-- Hostname Segment -->
            <template v-if="showHost">
              <div v-if="powerlineStyle === 'rounded'" class="power-seg rounded">
                <span class="cap-left" :style="{ color: palette.bgSurface }"></span>
                <span class="seg-body" :style="{ backgroundColor: palette.bgSurface, color: palette.primary }">
                  󰒋 {{ hostName }}
                </span>
                <span class="cap-right" :style="{ color: palette.bgSurface }"></span>
              </div>
              <div v-else-if="powerlineStyle === 'arrow'" class="power-seg arrow">
                <span class="sep-arrow-rev" :style="{ color: palette.bgSurface }"></span>
                <span class="seg-body" :style="{ backgroundColor: palette.bgSurface, color: palette.primary }">
                  󰒋 {{ hostName }}
                </span>
              </div>
              <div v-else-if="powerlineStyle === 'slanted'" class="power-seg slanted">
                <span class="sep-slanted-rev" :style="{ color: palette.bgSurface }"></span>
                <span class="seg-body" :style="{ backgroundColor: palette.bgSurface, color: palette.primary }">
                  󰒋 {{ hostName }}
                </span>
              </div>
              <div v-else-if="powerlineStyle === 'block'" class="power-seg block">
                <span class="seg-body" :style="{ backgroundColor: palette.bgSurface, color: palette.primary }">
                  󰒋 {{ hostName }}
                </span>
              </div>
              <div v-else class="power-seg minimal">
                <span :style="{ color: palette.primary }">󰒋 {{ hostName }}</span>
              </div>
            </template>

            <!-- Battery Segment -->
            <template v-if="showBattery">
              <div v-if="powerlineStyle === 'rounded'" class="power-seg rounded">
                <span class="cap-left" :style="{ color: palette.green }"></span>
                <span class="seg-body" :style="{ backgroundColor: palette.green, color: palette.bgDark, fontWeight: 'bold' }">
                  󰁹 95%
                </span>
                <span class="cap-right" :style="{ color: palette.green }"></span>
              </div>
              <div v-else-if="powerlineStyle === 'arrow'" class="power-seg arrow">
                <span class="sep-arrow-rev" :style="{ color: palette.green }"></span>
                <span class="seg-body" :style="{ backgroundColor: palette.green, color: palette.bgDark, fontWeight: 'bold' }">
                  󰁹 95%
                </span>
              </div>
              <div v-else-if="powerlineStyle === 'slanted'" class="power-seg slanted">
                <span class="sep-slanted-rev" :style="{ color: palette.green }"></span>
                <span class="seg-body" :style="{ backgroundColor: palette.green, color: palette.bgDark, fontWeight: 'bold' }">
                  󰁹 95%
                </span>
              </div>
              <div v-else-if="powerlineStyle === 'block'" class="power-seg block">
                <span class="seg-body" :style="{ backgroundColor: palette.green, color: palette.bgDark, fontWeight: 'bold' }">
                  󰁹 95%
                </span>
              </div>
              <div v-else class="power-seg minimal">
                <span :style="{ color: palette.green, fontWeight: 'bold' }">󰁹 95%</span>
              </div>
            </template>

            <!-- Window Count Segment -->
            <template v-if="showWindowCount">
              <div v-if="powerlineStyle === 'rounded'" class="power-seg rounded">
                <span class="cap-left" :style="{ color: palette.purple }"></span>
                <span class="seg-body" :style="{ backgroundColor: palette.purple, color: palette.bgDark, fontWeight: 'bold' }">
                  󰖲 4
                </span>
                <span class="cap-right" :style="{ color: palette.purple }"></span>
              </div>
              <div v-else-if="powerlineStyle === 'arrow'" class="power-seg arrow">
                <span class="sep-arrow-rev" :style="{ color: palette.purple }"></span>
                <span class="seg-body" :style="{ backgroundColor: palette.purple, color: palette.bgDark, fontWeight: 'bold' }">
                  󰖲 4
                </span>
              </div>
              <div v-else-if="powerlineStyle === 'slanted'" class="power-seg slanted">
                <span class="sep-slanted-rev" :style="{ color: palette.purple }"></span>
                <span class="seg-body" :style="{ backgroundColor: palette.purple, color: palette.bgDark, fontWeight: 'bold' }">
                  󰖲 4
                </span>
              </div>
              <div v-else-if="powerlineStyle === 'block'" class="power-seg block">
                <span class="seg-body" :style="{ backgroundColor: palette.purple, color: palette.bgDark, fontWeight: 'bold' }">
                  󰖲 4
                </span>
              </div>
              <div v-else class="power-seg minimal">
                <span :style="{ color: palette.purple }">󰖲 4</span>
              </div>
            </template>

            <!-- Time / Clock Segment -->
            <template v-if="showTime">
              <div v-if="powerlineStyle === 'rounded'" class="power-seg rounded">
                <span class="cap-left" :style="{ color: palette.peach }"></span>
                <span class="seg-body" :style="{ backgroundColor: palette.peach, color: palette.bgDark, fontWeight: 'bold' }">
                  󱑂 {{ currentTime }}
                </span>
                <span class="cap-right" :style="{ color: palette.peach }"></span>
              </div>
              <div v-else-if="powerlineStyle === 'arrow'" class="power-seg arrow">
                <span class="sep-arrow-rev" :style="{ color: palette.peach }"></span>
                <span class="seg-body" :style="{ backgroundColor: palette.peach, color: palette.bgDark, fontWeight: 'bold' }">
                  󱑂 {{ currentTime }}
                </span>
              </div>
              <div v-else-if="powerlineStyle === 'slanted'" class="power-seg slanted">
                <span class="sep-slanted-rev" :style="{ color: palette.peach }"></span>
                <span class="seg-body" :style="{ backgroundColor: palette.peach, color: palette.bgDark, fontWeight: 'bold' }">
                  󱑂 {{ currentTime }}
                </span>
              </div>
              <div v-else-if="powerlineStyle === 'block'" class="power-seg block">
                <span class="seg-body" :style="{ backgroundColor: palette.peach, color: palette.bgDark, fontWeight: 'bold' }">
                  󱑂 {{ currentTime }}
                </span>
              </div>
              <div v-else class="power-seg minimal">
                <span :style="{ color: palette.peach, fontWeight: 'bold' }">󱑂 {{ currentTime }}</span>
              </div>
            </template>
          </div>

        </div>
      </div>

      <!-- INTERACTIVE CUSTOMIZER TOOLBAR -->
      <div class="builder-controls-panel">
        
        <!-- Row 1: Style & Theme Pickers -->
        <div class="builder-row">
          <div class="builder-group">
            <label class="builder-label">Powerline Style:</label>
            <div class="style-selector-pills">
              <button
                v-for="st in powerlineStyles"
                :key="st.id"
                class="style-choice-btn"
                :class="{ active: powerlineStyle === st.id }"
                @click="powerlineStyle = st.id"
              >
                <span class="style-icon">{{ st.icon }}</span>
                <span>{{ st.name }}</span>
              </button>
            </div>
          </div>

          <div class="builder-group">
            <label class="builder-label">Theme Palette:</label>
            <select v-model="selectedThemeId" class="theme-dropdown" @change="onThemeChange">
              <option v-for="t in themes" :key="t.id" :value="t.id">
                {{ t.name }} ({{ t.variant }})
              </option>
            </select>
          </div>
        </div>

        <!-- Row 2: Modular Segment Toggles -->
        <div class="builder-row modules-row">
          <div class="builder-group">
            <label class="builder-label">Left Segments:</label>
            <div class="toggle-chips-group">
              <label class="toggle-chip">
                <input type="checkbox" v-model="showSession" />
                <span>Session (󰇄)</span>
              </label>
              <label class="toggle-chip">
                <input type="checkbox" v-model="showPath" />
                <span>Path (󰉋)</span>
              </label>
              <label class="toggle-chip">
                <input type="checkbox" v-model="showGit" />
                <span>Git (󰊢)</span>
              </label>
            </div>
          </div>

          <div class="builder-group">
            <label class="builder-label">Window Tabs:</label>
            <div class="toggle-chips-group">
              <label class="toggle-chip">
                <input type="checkbox" v-model="showWindowIcons" />
                <span>Icons (󰨞/)</span>
              </label>
              <label class="toggle-chip">
                <input type="checkbox" v-model="windowZoomed" />
                <span>Zoomed (🔍)</span>
              </label>
            </div>
          </div>

          <div class="builder-group">
            <label class="builder-label">Right Segments:</label>
            <div class="toggle-chips-group">
              <label class="toggle-chip">
                <input type="checkbox" v-model="showHost" />
                <span>Host (󰒋)</span>
              </label>
              <label class="toggle-chip">
                <input type="checkbox" v-model="showBattery" />
                <span>Battery (󰁹)</span>
              </label>
              <label class="toggle-chip">
                <input type="checkbox" v-model="showCpu" />
                <span>CPU (󰍛)</span>
              </label>
              <label class="toggle-chip">
                <input type="checkbox" v-model="showTime" />
                <span>Clock (󱑂)</span>
              </label>
              <label class="toggle-chip">
                <input type="checkbox" v-model="showWindowCount" />
                <span>Windows (󰖲)</span>
              </label>
            </div>
          </div>
        </div>

        <!-- Row 3: Simulator State & Mock Data -->
        <div class="builder-row live-test-row">
          <div class="builder-group test-toggles">
            <label class="builder-label">Simulate State:</label>
            <label class="state-toggle-btn" :class="{ active: prefixActive }">
              <input type="checkbox" v-model="prefixActive" />
              <span>⌃B Prefix Pressed</span>
            </label>
          </div>

          <div class="builder-group mock-inputs">
            <label class="builder-label">Mock Data:</label>
            <div class="inputs-flex">
              <input
                v-model="sessionName"
                placeholder="Session"
                title="Session Name"
                class="mock-input session"
              />
              <input
                v-model="hostName"
                placeholder="Host"
                title="Host Name"
                class="mock-input host"
              />
              <input
                v-model="currentPath"
                placeholder="Path"
                title="Current Working Path"
                class="mock-input path"
              />
            </div>
          </div>
        </div>

      </div>
    </div>

    <!-- Theme Details, Swatches & Generated Code Box -->
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

        <div style="font-size: 0.8rem; font-weight: 600; margin-bottom: 0.35rem;">Theme Palette (click to copy):</div>
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
          <button class="btn-preview" @click="downloadConfig">
            <span>⬇ Download tmuxship.toml</span>
          </button>
        </div>
      </div>

      <!-- Code Viewer with Shiki -->
      <div class="code-viewer">
        <div class="code-viewer-header">
          <div class="code-tabs-nav">
            <button
              class="code-tab-btn"
              :class="{ active: currentTab === 'tmuxship' }"
              @click="currentTab = 'tmuxship'"
            >
              tmuxship.toml (Unified)
            </button>
            <button
              class="code-tab-btn"
              :class="{ active: currentTab === 'tmux' }"
              @click="currentTab = 'tmux'"
            >
              ~/.tmux.conf
            </button>
            <button
              class="code-tab-btn"
              :class="{ active: currentTab === 'left' }"
              @click="currentTab = 'left'"
            >
              starship.toml (Left)
            </button>
            <button
              class="code-tab-btn"
              :class="{ active: currentTab === 'center' }"
              @click="currentTab = 'center'"
            >
              .center.toml
            </button>
            <button
              class="code-tab-btn"
              :class="{ active: currentTab === 'right' }"
              @click="currentTab = 'right'"
            >
              .right.toml
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

    <!-- Theme Gallery Cards Grid -->
    <div style="margin-top: 2.5rem;">
      <div style="display: flex; justify-content: space-between; align-items: flex-end; margin-bottom: 0.75rem;">
        <div>
          <h3 style="font-size: 1.25rem; font-weight: 700; margin: 0;">Explore All Color Themes</h3>
          <p style="font-size: 0.85rem; color: var(--vp-c-text-2); margin: 0.2rem 0 0;">
            Click any palette to instantly apply its colors to your active powerline setup
          </p>
        </div>
        <span style="font-size: 0.8rem; color: var(--vp-c-text-3);">{{ themes.length }} themes available</span>
      </div>

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
            <div style="display: flex; gap: 4px;">
              <span
                v-for="s in (t.swatches || []).slice(0, 5)"
                :key="s.name"
                style="width: 12px; height: 12px; border-radius: 2px;"
                :style="{ backgroundColor: s.hex }"
              ></span>
            </div>
            <span style="font-size: 0.8rem; color: var(--vp-c-brand-1); font-weight: 600;">Apply Palette &rarr;</span>
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
const selectedThemeId = ref('catppuccin-mocha');
const powerlineStyle = ref('rounded'); // 'rounded' | 'arrow' | 'slanted' | 'minimal' | 'block'

// Modular Toggles
const showSession = ref(true);
const showPath = ref(true);
const showGit = ref(true);
const showWindowIcons = ref(true);
const showHost = ref(true);
const showBattery = ref(true);
const showCpu = ref(false);
const showTime = ref(true);
const showWindowCount = ref(false);

// State Simulator
const prefixActive = ref(false);
const windowZoomed = ref(false);
const activeWindowIndex = ref(2);
const sessionName = ref('dev');
const hostName = ref('laptop');
const currentPath = ref('~/projects/tmuxship');
const gitBranch = ref('main');

const currentTab = ref('tmuxship');
const currentTime = ref('');
const highlightedCode = ref('');

let highlighter = null;
let timer = null;

const powerlineStyles = [
  { id: 'rounded', name: 'Rounded Pills', icon: ' ' },
  { id: 'arrow', name: 'Classic Arrow', icon: ' ' },
  { id: 'slanted', name: 'Slanted / Angle', icon: ' ' },
  { id: 'minimal', name: 'Clean Minimal', icon: '• │' },
  { id: 'block', name: 'Solid Block', icon: '█ █' },
];

const quickPresets = [
  { name: 'Catppuccin Pills', themeId: 'catppuccin-mocha', style: 'rounded', dotColor: '#89b4fa' },
  { name: 'Rosé Pine Bubbles', themeId: 'rose-pine', style: 'rounded', dotColor: '#eb6f92' },
  { name: 'Tokyo Night Slanted', themeId: 'tokyo-night', style: 'slanted', dotColor: '#7aa2f7' },
  { name: 'Nord Classic Arrow', themeId: 'nord', style: 'arrow', dotColor: '#88c0d0' },
  { name: 'Gruvbox Clean', themeId: 'gruvbox-dark', style: 'minimal', dotColor: '#fabd2f' },
];

const windowList = [
  { index: 1, name: 'code', icon: '󰨞' },
  { index: 2, name: 'server', icon: '' },
  { index: 3, name: 'logs', icon: '󰈚' },
  { index: 4, name: 'notes', icon: '󰒋' },
];

const currentTheme = computed(() => {
  return themes.value.find(t => t.id === selectedThemeId.value) || themes.value[0] || {};
});

const currentStyleLabel = computed(() => {
  const match = powerlineStyles.find(s => s.id === powerlineStyle.value);
  return match ? match.name : 'Powerline';
});

const shortPath = computed(() => {
  return currentPath.value.replace(/\/home\/[^\/]+/, '~');
});

// Dynamic Semantic Color Palette derived from theme swatches & variant
const palette = computed(() => {
  const t = currentTheme.value;
  const sw = t.swatches || [];
  const isLightTheme = t.variant === 'light';

  // Base background & surfaces
  const bgDark = isLightTheme ? '#eff1f5' : '#11111b';
  const bgSurface = isLightTheme ? '#dce0e8' : '#313244';
  const bgMute = isLightTheme ? '#ccd0da' : '#1e1e2e';
  const fgMain = isLightTheme ? '#4c4f69' : '#cdd6f4';
  const fgMuted = isLightTheme ? '#8c8fa1' : '#6c7086';

  // Extract swatches or use harmonious defaults
  const findSwatch = (names, fallback) => {
    for (const n of names) {
      const match = sw.find(s => s.name.toLowerCase().includes(n));
      if (match) return match.hex;
    }
    return fallback;
  };

  const primary = findSwatch(['blue', 'pine', 'sapphire', 'cyan'], isLightTheme ? '#1e66f5' : '#89b4fa');
  const prefixColor = findSwatch(['love', 'red', 'rose', 'coral'], isLightTheme ? '#d20f39' : '#f38ba8');
  const green = findSwatch(['green', 'foam', 'emerald'], isLightTheme ? '#40a02b' : '#a6e3a1');
  const purple = findSwatch(['mauve', 'iris', 'magenta', 'purple'], isLightTheme ? '#8839ef' : '#cba6f7');
  const yellow = findSwatch(['gold', 'yellow', 'sun'], isLightTheme ? '#df8e1d' : '#f9e2af');
  const peach = findSwatch(['peach', 'orange', 'salmon'], isLightTheme ? '#fe640b' : '#fab387');
  const teal = findSwatch(['teal', 'sky', 'aqua'], isLightTheme ? '#179299' : '#94e2d5');

  return {
    bgDark,
    bgSurface,
    bgMute,
    fgMain,
    fgMuted,
    primary,
    prefixColor,
    green,
    purple,
    yellow,
    peach,
    teal,
  };
});

function applyQuickPreset(p) {
  selectedThemeId.value = p.themeId;
  powerlineStyle.value = p.style;
  updateHighlight();
}

function selectTheme(themeId) {
  selectedThemeId.value = themeId;
  updateHighlight();
}

function onThemeChange() {
  updateHighlight();
}

// Generate Unified TOML Config dynamically
const generatedUnifiedToml = computed(() => {
  const p = palette.value;
  const t = currentTheme.value;
  const style = powerlineStyle.value;

  let leftToml = `[left]
"$schema" = 'https://starship.rs/config-schema.json'
format = "$custom"
add_newline = false\n`;

  if (showSession.value) {
    if (style === 'rounded') {
      leftToml += `\n[left.custom.prefix_active]
shell = "bash"
when = 'test "\${TMUX_CLIENT_PREFIX:-0}" = "1"'
command = 'printf "#[bg=${p.prefixColor},fg=${p.bgDark},bold] 󰇄 %s #[bg=default,fg=${p.prefixColor}]" "\${TMUX_SESSION_NAME:-tmux}"'
format = "$output "

[left.custom.session_normal]
shell = "bash"
when = 'test "\${TMUX_CLIENT_PREFIX:-0}" != "1"'
command = 'printf "#[bg=${p.primary},fg=${p.bgDark},bold] 󰇄 %s #[bg=default,fg=${p.primary}]" "\${TMUX_SESSION_NAME:-tmux}"'
format = "$output "\n`;
    } else if (style === 'arrow') {
      leftToml += `\n[left.custom.prefix_active]
shell = "bash"
when = 'test "\${TMUX_CLIENT_PREFIX:-0}" = "1"'
command = 'printf "#[bg=${p.prefixColor},fg=${p.bgDark},bold] 󰇄 %s #[bg=default,fg=${p.prefixColor}]" "\${TMUX_SESSION_NAME:-tmux}"'
format = "$output "

[left.custom.session_normal]
shell = "bash"
when = 'test "\${TMUX_CLIENT_PREFIX:-0}" != "1"'
command = 'printf "#[bg=${p.primary},fg=${p.bgDark},bold] 󰇄 %s #[bg=default,fg=${p.primary}]" "\${TMUX_SESSION_NAME:-tmux}"'
format = "$output "\n`;
    } else if (style === 'slanted') {
      leftToml += `\n[left.custom.prefix_active]
shell = "bash"
when = 'test "\${TMUX_CLIENT_PREFIX:-0}" = "1"'
command = 'printf "#[bg=${p.prefixColor},fg=${p.bgDark},bold] 󰇄 %s #[bg=default,fg=${p.prefixColor}]" "\${TMUX_SESSION_NAME:-tmux}"'
format = "$output "

[left.custom.session_normal]
shell = "bash"
when = 'test "\${TMUX_CLIENT_PREFIX:-0}" != "1"'
command = 'printf "#[bg=${p.primary},fg=${p.bgDark},bold] 󰇄 %s #[bg=default,fg=${p.primary}]" "\${TMUX_SESSION_NAME:-tmux}"'
format = "$output "\n`;
    } else if (style === 'block') {
      leftToml += `\n[left.custom.prefix_active]
shell = "bash"
when = 'test "\${TMUX_CLIENT_PREFIX:-0}" = "1"'
command = 'printf " 󰇄 %s " "\${TMUX_SESSION_NAME:-tmux}"'
format = "[$output]($style) "
style = "bg:${p.prefixColor} fg:${p.bgDark} bold"

[left.custom.session_normal]
shell = "bash"
when = 'test "\${TMUX_CLIENT_PREFIX:-0}" != "1"'
command = 'printf " 󰇄 %s " "\${TMUX_SESSION_NAME:-tmux}"'
format = "[$output]($style) "
style = "bg:${p.primary} fg:${p.bgDark} bold"\n`;
    } else {
      leftToml += `\n[left.custom.prefix_active]
shell = "bash"
when = 'test "\${TMUX_CLIENT_PREFIX:-0}" = "1"'
command = 'printf "󰇄 %s" "\${TMUX_SESSION_NAME:-tmux}"'
format = "[$output]($style) "
style = "fg:${p.prefixColor} bold"

[left.custom.session_normal]
shell = "bash"
when = 'test "\${TMUX_CLIENT_PREFIX:-0}" != "1"'
command = 'printf "󰇄 %s" "\${TMUX_SESSION_NAME:-tmux}"'
format = "[$output]($style) "
style = "fg:${p.primary}"\n`;
    }
  }

  if (showPath.value) {
    if (style === 'rounded') {
      leftToml += `\n[left.custom.directory]
shell = "bash"
when = 'test -n "\${TMUX_PANE_CURRENT_PATH}"'
command = 'p="\${TMUX_PANE_CURRENT_PATH}"; p="\${p/#$HOME/~}"; printf "#[bg=${p.bgSurface},fg=${p.teal}] 󰉋 %s #[bg=default,fg=${p.bgSurface}]" "$p"'
format = "$output "\n`;
    } else if (style === 'arrow') {
      leftToml += `\n[left.custom.directory]
shell = "bash"
when = 'test -n "\${TMUX_PANE_CURRENT_PATH}"'
command = 'p="\${TMUX_PANE_CURRENT_PATH}"; p="\${p/#$HOME/~}"; printf "#[bg=${p.bgSurface},fg=${p.teal}] 󰉋 %s #[bg=default,fg=${p.bgSurface}]" "$p"'
format = "$output "\n`;
    } else if (style === 'slanted') {
      leftToml += `\n[left.custom.directory]
shell = "bash"
when = 'test -n "\${TMUX_PANE_CURRENT_PATH}"'
command = 'p="\${TMUX_PANE_CURRENT_PATH}"; p="\${p/#$HOME/~}"; printf "#[bg=${p.bgSurface},fg=${p.teal}] 󰉋 %s #[bg=default,fg=${p.bgSurface}]" "$p"'
format = "$output "\n`;
    } else {
      leftToml += `\n[left.custom.directory]
shell = "bash"
when = 'test -n "\${TMUX_PANE_CURRENT_PATH}"'
command = 'p="\${TMUX_PANE_CURRENT_PATH}"; p="\${p/#$HOME/~}"; printf "󰉋 %s" "$p"'
format = "[$output]($style) "
style = "fg:${p.teal}"\n`;
    }
  }

  if (showGit.value) {
    leftToml += `\n[left.custom.git_branch]
shell = "bash"
when = 'test -n "\${TMUX_PANE_CURRENT_PATH}" && git -C "\${TMUX_PANE_CURRENT_PATH}" rev-parse --is-inside-work-tree >/dev/null 2>&1'
command = 'b=$(git -C "\${TMUX_PANE_CURRENT_PATH}" branch --show-current 2>/dev/null); test -n "$b" && printf "󰊢 %s" "$b"'
format = "[$output]($style) "
style = "fg:${p.green}"\n`;
  }

  let centerToml = `\n[center]
format = "$custom"
add_newline = false\n`;

  if (style === 'rounded') {
    centerToml += `\n[center.custom.window_active]
when = 'test "\${TMUX_WINDOW_ACTIVE:-0}" = "1"'
command = 'printf "#[bg=${p.purple},fg=${p.bgDark},bold] #I #[bg=${p.bgSurface},fg=${p.fgMain},bold] #W #[bg=default,fg=${p.bgSurface}]"'
format = "$output"

[center.custom.window_inactive]
when = 'test "\${TMUX_WINDOW_ACTIVE:-0}" != "1"'
command = 'printf "#[bg=${p.bgSurface},fg=${p.fgMuted}] #I #[bg=${p.bgMute},fg=${p.fgMuted}] #W #[bg=default,fg=${p.bgMute}]"'
format = "$output"\n`;
  } else if (style === 'arrow') {
    centerToml += `\n[center.custom.window_active]
when = 'test "\${TMUX_WINDOW_ACTIVE:-0}" = "1"'
command = 'printf "#[bg=${p.purple},fg=${p.bgDark},bold] #I #W #[bg=default,fg=${p.purple}]"'
format = "$output "

[center.custom.window_inactive]
when = 'test "\${TMUX_WINDOW_ACTIVE:-0}" != "1"'
command = 'printf "#[bg=${p.bgSurface},fg=${p.fgMuted}] #I #W #[bg=default,fg=${p.bgSurface}]"'
format = "$output "\n`;
  } else if (style === 'slanted') {
    centerToml += `\n[center.custom.window_active]
when = 'test "\${TMUX_WINDOW_ACTIVE:-0}" = "1"'
command = 'printf "#[bg=${p.purple},fg=${p.bgDark},bold] #I #W #[bg=default,fg=${p.purple}]"'
format = "$output "

[center.custom.window_inactive]
when = 'test "\${TMUX_WINDOW_ACTIVE:-0}" != "1"'
command = 'printf "#[bg=${p.bgSurface},fg=${p.fgMuted}] #I #W #[bg=default,fg=${p.bgSurface}]"'
format = "$output "\n`;
  } else if (style === 'block') {
    centerToml += `\n[center.custom.window_active]
when = 'test "\${TMUX_WINDOW_ACTIVE:-0}" = "1"'
command = 'printf " #I:#W "'
format = "[$output]($style)"
style = "bg:${p.purple} fg:${p.bgDark} bold"

[center.custom.window_inactive]
when = 'test "\${TMUX_WINDOW_ACTIVE:-0}" != "1"'
command = 'printf " #I:#W "'
format = "[$output]($style)"
style = "bg:${p.bgSurface} fg:${p.fgMuted}"\n`;
  } else {
    centerToml += `\n[center.custom.window_active]
when = 'test "\${TMUX_WINDOW_ACTIVE:-0}" = "1"'
command = 'printf "%s:%s" "\${TMUX_WINDOW_INDEX:-1}" "\${TMUX_WINDOW_NAME:-sh}"'
format = "[$output]($style)"
style = "fg:${p.purple} bold"

[center.custom.window_inactive]
when = 'test "\${TMUX_WINDOW_ACTIVE:-0}" != "1"'
command = 'printf "%s:%s" "\${TMUX_WINDOW_INDEX:-1}" "\${TMUX_WINDOW_NAME:-sh}"'
format = "[$output]($style)"
style = "fg:${p.fgMuted}"\n`;
  }

  centerToml += `\n[center.custom.window_zoom]
when = 'test "\${TMUX_WINDOW_ACTIVE:-0}" = "1" && test "\${TMUX_WINDOW_ZOOMED_FLAG:-0}" = "1"'
command = 'printf " 🔍"'
format = "$output"
style = "fg:${p.yellow}"\n`;

  let rightToml = `\n[right]
"$schema" = 'https://starship.rs/config-schema.json'
format = "$custom"
add_newline = false\n`;

  if (showCpu.value) {
    rightToml += `\n[right.custom.cpu]
when = "true"
shell = "bash"
command = 'load=$(uptime | awk -F "load average:" "{print \\$2}" | cut -d, -f1 | tr -d " "); printf "󰍛 %s" "$load"'
format = "[$output]($style) "
style = "fg:${p.teal}"\n`;
  }

  if (showHost.value) {
    if (style === 'rounded') {
      rightToml += `\n[right.custom.host]
when = "true"
shell = "bash"
command = 'printf "#[bg=${p.bgSurface},fg=${p.primary}] 󰒋 %s #[bg=default,fg=${p.bgSurface}]" "\${TMUX_HOST_SHORT:-$(hostname -s)}"'
format = "$output "\n`;
    } else if (style === 'arrow') {
      rightToml += `\n[right.custom.host]
when = "true"
shell = "bash"
command = 'printf "#[bg=default,fg=${p.bgSurface}]#[bg=${p.bgSurface},fg=${p.primary}] 󰒋 %s " "\${TMUX_HOST_SHORT:-$(hostname -s)}"'
format = "$output"\n`;
    } else if (style === 'slanted') {
      rightToml += `\n[right.custom.host]
when = "true"
shell = "bash"
command = 'printf "#[bg=default,fg=${p.bgSurface}]#[bg=${p.bgSurface},fg=${p.primary}] 󰒋 %s " "\${TMUX_HOST_SHORT:-$(hostname -s)}"'
format = "$output"\n`;
    } else {
      rightToml += `\n[right.custom.host]
when = "true"
shell = "bash"
command = 'printf "󰒋 %s" "\${TMUX_HOST_SHORT:-$(hostname -s)}"'
format = "[$output]($style) "
style = "fg:${p.primary}"\n`;
    }
  }

  if (showBattery.value) {
    if (style === 'rounded') {
      rightToml += `\n[right.custom.battery]
when = "which pmset >/dev/null 2>&1 || test -d /sys/class/power_supply/BAT0"
shell = "bash"
command = 'pct=$(pmset -g batt 2>/dev/null | grep -o "[0-9]\\\\+%" | head -1 || cat /sys/class/power_supply/BAT0/capacity 2>/dev/null); test -n "$pct" && printf "#[bg=${p.green},fg=${p.bgDark},bold] 󰁹 %s #[bg=default,fg=${p.green}]" "$pct"'
format = "$output "\n`;
    } else {
      rightToml += `\n[right.custom.battery]
when = "which pmset >/dev/null 2>&1 || test -d /sys/class/power_supply/BAT0"
shell = "bash"
command = 'pct=$(pmset -g batt 2>/dev/null | grep -o "[0-9]\\\\+%" | head -1 || cat /sys/class/power_supply/BAT0/capacity 2>/dev/null); test -n "$pct" && printf "󰁹 %s" "$pct"'
format = "[$output]($style) "
style = "fg:${p.green}"\n`;
    }
  }

  if (showWindowCount.value) {
    rightToml += `\n[right.custom.window_count]
when = "true"
shell = "bash"
command = 'printf "󰖲 %s" "\${TMUX_SESSION_WINDOWS:-1}"'
format = "[$output]($style) "
style = "fg:${p.purple}"\n`;
  }

  if (showTime.value) {
    if (style === 'rounded') {
      rightToml += `\n[right.custom.time]
when = "true"
shell = "bash"
command = 'printf "#[bg=${p.peach},fg=${p.bgDark},bold] 󱑂 %s #[bg=default,fg=${p.peach}]" "$(date +%H:%M)"'
format = "$output"\n`;
    } else if (style === 'arrow') {
      rightToml += `\n[right.custom.time]
when = "true"
shell = "bash"
command = 'printf "#[bg=default,fg=${p.peach}]#[bg=${p.peach},fg=${p.bgDark},bold] 󱑂 %s " "$(date +%H:%M)"'
format = "$output"\n`;
    } else if (style === 'slanted') {
      rightToml += `\n[right.custom.time]
when = "true"
shell = "bash"
command = 'printf "#[bg=default,fg=${p.peach}]#[bg=${p.peach},fg=${p.bgDark},bold] 󱑂 %s " "$(date +%H:%M)"'
format = "$output"\n`;
    } else {
      rightToml += `\n[right.custom.time]
when = "true"
shell = "bash"
command = 'date +%H:%M:%S'
format = "[󱑂 $output]($style)"
style = "fg:${p.peach}"\n`;
    }
  }

  const header = `# tmuxship.toml — Generated by Theme & Segment Builder
name = "${t.name} (${currentStyleLabel.value})"
window_separator = "${style === 'rounded' || style === 'arrow' || style === 'slanted' ? ' ' : ' • '}"\n\n`;

  return header + leftToml + centerToml + rightToml;
});

const rawCodeToHighlight = computed(() => {
  if (currentTab.value === 'tmux') {
    return `# ~/.tmux.conf — tmuxship integration
# 1. Point to your tmuxship configuration
setenv -g TMUX_SHIP_CONFIG "$HOME/.tmux/tmuxship.toml"

# 2. Apply theme styles and hooks
run-shell 'tmuxship apply'

# 3. Dynamic refresh hooks on switch/focus
set-hook -g client-session-changed 'refresh-client -S'
set-hook -g client-attached        'refresh-client -S'
set-hook -g pane-focus-in          'refresh-client -S'
set -g window-status-style "bg=default,fg=default"`;
  } else if (currentTab.value === 'left') {
    return `# starship.toml (Left Status Segment)
# Place at ~/.tmux/starship.toml
` + generatedUnifiedToml.value.split('[center]')[0].replace(/\[left\./g, '[').replace(/\[left\]/, '');
  } else if (currentTab.value === 'center') {
    const parts = generatedUnifiedToml.value.split('[right]');
    const centerPart = parts[0].split('[center]')[1] || '';
    return `# .center.toml (Window Tabs Segment)\n# Place at ~/.tmux/.center.toml\n[center]` + centerPart.replace(/\[center\./g, '[');
  } else if (currentTab.value === 'right') {
    const rightPart = generatedUnifiedToml.value.split('[right]')[1] || '';
    return `# .right.toml (Right Status Segment)\n# Place at ~/.tmux/.right.toml\n[right]` + rightPart.replace(/\[right\./g, '[');
  }

  return generatedUnifiedToml.value;
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
  alert('Copied configuration to clipboard!');
}

function downloadConfig() {
  const content = generatedUnifiedToml.value;
  const blob = new Blob([content], { type: 'text/plain' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = `tmuxship-${selectedThemeId.value}-${powerlineStyle.value}.toml`;
  document.body.appendChild(a);
  a.click();
  a.remove();
  URL.revokeObjectURL(url);
}

function updateTime() {
  currentTime.value = new Date().toTimeString().split(' ')[0].substring(0, 5);
}

watch([currentTab, isDark, selectedThemeId, powerlineStyle, showSession, showPath, showGit, showWindowIcons, showHost, showBattery, showCpu, showTime, showWindowCount], () => {
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
