#!/usr/bin/env bash
# Generate SVG screenshots from Starship ANSI output using ansisvg.
# Prerequisites: starship, ansisvg
#
# Usage:
#   ./scripts/generate-screenshots.sh

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUTPUT_DIR="$REPO_ROOT/screenshots"
EXAMPLES_DIR="$REPO_ROOT/examples"

mkdir -p "$OUTPUT_DIR"

RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

pass()  { printf "${GREEN}OK${NC}  %s\n" "$1"; }
fail()  { printf "${RED}FAIL${NC} %s\n" "$1" >&2; }

check_dep() {
  if ! command -v "$1" &>/dev/null; then
    echo "ERROR: $1 is required but not found in PATH" >&2
    exit 1
  fi
}

# starship_ansi <config> [env_vars...] — prints ANSI from starship to stdout
starship_ansi() {
  local config="$1"; shift
  env STARSHIP_CONFIG="$config" STARSHIP_SHELL=sh CLICOLOR_FORCE=1 "$@" \
    starship prompt 2>/dev/null
}

# render_svg <output.svg> <width> [extra_ansisvg_args...]
# Reads ANSI from stdin, writes SVG to OUTPUT_DIR.
render_svg() {
  local out="$1" width="$2"; shift 2
  ansisvg --width "$width" --fontsize 13 --lineheight 1.3 "$@" > "$OUTPUT_DIR/$out"
}

# scene <basename> <config> <width> [env_vars...]
scene() {
  local name="$1" config="$2" width="$3"; shift 3
  local out="$name.svg"
  printf '  %-32s' "$name"
  if starship_ansi "$config" "$@" | render_svg "$out" "$width"; then
    pass "$out"
  else
    fail "$out"
  fi
}

# ------------------------------------------------------------------

check_dep starship
check_dep ansisvg

echo "==> Generating screenshots..."

echo ""
echo "--- Left Status ---"

scene "left-normal" \
  "$EXAMPLES_DIR/starship.toml" 16 \
  TMUX_CLIENT_PREFIX=0 TMUX_SESSION_NAME=dev

scene "left-prefix" \
  "$EXAMPLES_DIR/starship.toml" 16 \
  TMUX_CLIENT_PREFIX=1 TMUX_SESSION_NAME=dev

scene "left-advanced" \
  "$EXAMPLES_DIR/advanced-left.toml" 60 \
  TMUX_CLIENT_PREFIX=0 TMUX_SESSION_NAME=dev \
  TMUX_PANE_CURRENT_PATH="$REPO_ROOT"

echo ""
echo "--- Center / Window Status ---"

scene "window-active" \
  "$EXAMPLES_DIR/.center.toml" 20 \
  TMUX_WINDOW_ACTIVE=1 TMUX_WINDOW_INDEX=0 TMUX_WINDOW_NAME=editor \
  TMUX_WINDOW_ZOOMED_FLAG=0

scene "window-inactive" \
  "$EXAMPLES_DIR/.center.toml" 20 \
  TMUX_WINDOW_ACTIVE=0 TMUX_WINDOW_INDEX=1 TMUX_WINDOW_NAME=server \
  TMUX_WINDOW_ZOOMED_FLAG=0

scene "window-zoom" \
  "$EXAMPLES_DIR/.center.toml" 20 \
  TMUX_WINDOW_ACTIVE=1 TMUX_WINDOW_INDEX=0 TMUX_WINDOW_NAME=editor \
  TMUX_WINDOW_ZOOMED_FLAG=1

echo ""
echo "--- Right Status ---"

scene "right" \
  "$EXAMPLES_DIR/.right.toml" 52 \
  TMUX_HOST_SHORT=makina TMUX_SESSION_WINDOWS=4

# --- Full Status Bar (Composite) -----------------------------------

echo ""
echo "--- Full Status Bar ---"

ESC=$(printf '\033')
BAR_BG="${ESC}[48;2;30;30;46m"
BAR_SEP="${ESC}[38;2;108;112;134m"
BAR_RST="${ESC}[0m"

# Strip trailing \e[0m + whitespace from starship output so the bar bg
# stays continuous.  Also re-apply bar bg after any internal \e[0m.
bar_segment() {
  tr -d '\n' | sed \
    -e "s/${ESC}\[0m[[:space:]]*$//" \
    -e "s/${ESC}\[0m/${ESC}[0m${BAR_BG}/g"
}

L=$(starship_ansi "$EXAMPLES_DIR/starship.toml" \
    TMUX_CLIENT_PREFIX=0 TMUX_SESSION_NAME=dev | bar_segment)
W0=$(starship_ansi "$EXAMPLES_DIR/.center.toml" \
    TMUX_WINDOW_ACTIVE=1 TMUX_WINDOW_INDEX=0 TMUX_WINDOW_NAME=editor \
    TMUX_WINDOW_ZOOMED_FLAG=0 | bar_segment)
W1=$(starship_ansi "$EXAMPLES_DIR/.center.toml" \
    TMUX_WINDOW_ACTIVE=0 TMUX_WINDOW_INDEX=1 TMUX_WINDOW_NAME=server \
    TMUX_WINDOW_ZOOMED_FLAG=0 | bar_segment)
W2=$(starship_ansi "$EXAMPLES_DIR/.center.toml" \
    TMUX_WINDOW_ACTIVE=0 TMUX_WINDOW_INDEX=2 TMUX_WINDOW_NAME=shell \
    TMUX_WINDOW_ZOOMED_FLAG=0 | bar_segment)
W3=$(starship_ansi "$EXAMPLES_DIR/.center.toml" \
    TMUX_WINDOW_ACTIVE=0 TMUX_WINDOW_INDEX=3 TMUX_WINDOW_NAME=logs \
    TMUX_WINDOW_ZOOMED_FLAG=0 | bar_segment)
R=$(starship_ansi "$EXAMPLES_DIR/.right.toml" \
    TMUX_HOST_SHORT=makina TMUX_SESSION_WINDOWS=4 | bar_segment)

# Layout (120 columns):
#   left (columns 0..), windows (centred around col 60), right (right edge)

PAD28='                            '  # 28 literal space chars (left → windows gap)
PAD18='                  '            # 18 literal space chars (windows → right gap)
# Separator resets any leaking styles from the previous window segment,
# then re-applies bar bg + muted fg.
SEP="${BAR_RST}${BAR_BG}${BAR_SEP} • ${BAR_RST}${BAR_BG}"

printf "%b" \
  "${BAR_BG}${L}${PAD28}" \
  "${W0}${SEP}${W1}${SEP}${W2}${SEP}${W3}${PAD18}" \
  "${R}${BAR_RST}" \
  | render_svg "full-bar.svg" 120

pass "full-bar.svg"

# --- Summary -------------------------------------------------------

echo ""
echo "Generated $(find "$OUTPUT_DIR" -name '*.svg' | wc -l | tr -d ' ') SVGs in $OUTPUT_DIR/:"
find "$OUTPUT_DIR" -name '*.svg' -exec basename {} \; | sort | while read -r f; do
  size=$(wc -c < "$OUTPUT_DIR/$f" | tr -d ' ')
  printf '  %-28s %s bytes\n' "$f" "$size"
done
echo ""
echo "Done."
