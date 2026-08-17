use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeVariant {
    Dark,
    Light,
}

impl std::fmt::Display for ThemeVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThemeVariant::Dark => write!(f, "dark"),
            ThemeVariant::Light => write!(f, "light"),
        }
    }
}

fn default_variant() -> ThemeVariant {
    ThemeVariant::Dark
}

fn default_separator() -> String {
    " • ".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorSwatch {
    pub name: String,
    pub hex: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeMeta {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub author: String,
    #[serde(default = "default_variant")]
    pub variant: ThemeVariant,
    #[serde(default = "default_separator")]
    pub window_separator: String,
    #[serde(default)]
    pub swatches: Vec<ColorSwatch>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Theme {
    pub id: String,
    pub name: String,
    pub description: String,
    pub author: String,
    pub variant: ThemeVariant,
    pub window_separator: String,
    pub swatches: Vec<ColorSwatch>,
    pub left_toml: String,
    pub center_toml: String,
    pub right_toml: String,
    pub full_toml: String,
    #[serde(skip)]
    pub is_custom: bool,
}

macro_rules! builtin_theme {
    ($id:expr, $name:expr, $desc:expr, $author:expr, $variant:expr, $sep:expr, [ $( ($sname:expr, $shex:expr) ),* ], $left:expr, $center:expr, $right:expr, $full:expr) => {
        Theme {
            id: $id.to_string(),
            name: $name.to_string(),
            description: $desc.to_string(),
            author: $author.to_string(),
            variant: $variant,
            window_separator: $sep.to_string(),
            swatches: vec![
                $(
                    ColorSwatch {
                        name: $sname.to_string(),
                        hex: $shex.to_string(),
                    },
                )*
            ],
            left_toml: $left.to_string(),
            center_toml: $center.to_string(),
            right_toml: $right.to_string(),
            full_toml: $full.to_string(),
            is_custom: false,
        }
    };
}

pub fn builtin_themes() -> Vec<Theme> {
    vec![
        builtin_theme!(
            "rose-pine",
            "Rosé Pine",
            "All natural pine, faux fur and a bit of soho vibes for the classy minimalist",
            "Rose Pine",
            ThemeVariant::Dark,
            " • ",
            [("Love", "#eb6f92"), ("Gold", "#f6c177"), ("Rose", "#ebbcba"), ("Pine", "#31748f"), ("Foam", "#9ccfd8"), ("Iris", "#c4a7e7")],
            include_str!("../../themes/rose-pine/starship.toml"),
            include_str!("../../themes/rose-pine/.center.toml"),
            include_str!("../../themes/rose-pine/.right.toml"),
            include_str!("../../themes/rose-pine/.full.toml")
        ),
        builtin_theme!(
            "rose-pine-moon",
            "Rosé Pine Moon",
            "Cozy blend of warm and cool night hues for low-light focus",
            "Rose Pine",
            ThemeVariant::Dark,
            " • ",
            [("Love", "#eb6f92"), ("Gold", "#f6c177"), ("Rose", "#ea9a97"), ("Pine", "#3e8fb0"), ("Foam", "#9ccfd8"), ("Iris", "#c4a7e7")],
            include_str!("../../themes/rose-pine-moon/starship.toml"),
            include_str!("../../themes/rose-pine-moon/.center.toml"),
            include_str!("../../themes/rose-pine-moon/.right.toml"),
            include_str!("../../themes/rose-pine-moon/.full.toml")
        ),
        builtin_theme!(
            "rose-pine-dawn",
            "Rosé Pine Dawn",
            "Delicate sunrise hues on a warm paper background",
            "Rose Pine",
            ThemeVariant::Light,
            " • ",
            [("Love", "#b4637a"), ("Gold", "#ea9d34"), ("Rose", "#d7827e"), ("Pine", "#286983"), ("Foam", "#56949f"), ("Iris", "#907aa9")],
            include_str!("../../themes/rose-pine-dawn/starship.toml"),
            include_str!("../../themes/rose-pine-dawn/.center.toml"),
            include_str!("../../themes/rose-pine-dawn/.right.toml"),
            include_str!("../../themes/rose-pine-dawn/.full.toml")
        ),
        builtin_theme!(
            "catppuccin-mocha",
            "Catppuccin Mocha",
            "Soothing pastel dark theme with deep contrast and rich vibrant accents",
            "Catppuccin Org",
            ThemeVariant::Dark,
            " • ",
            [("Teal", "#94e2d5"), ("Blue", "#89b4fa"), ("Green", "#a6e3a1"), ("Yellow", "#f9e2af"), ("Mauve", "#cba6f7"), ("Surface", "#313244")],
            include_str!("../../themes/catppuccin-mocha/starship.toml"),
            include_str!("../../themes/catppuccin-mocha/.center.toml"),
            include_str!("../../themes/catppuccin-mocha/.right.toml"),
            include_str!("../../themes/catppuccin-mocha/.full.toml")
        ),
        builtin_theme!(
            "catppuccin-macchiato",
            "Catppuccin Macchiato",
            "Medium-contrast dark flavor with smooth pastel undertones",
            "Catppuccin Org",
            ThemeVariant::Dark,
            " • ",
            [("Teal", "#8bd5ca"), ("Blue", "#8aadf4"), ("Green", "#a6da95"), ("Peach", "#f5a97f"), ("Mauve", "#c6a0f6")],
            include_str!("../../themes/catppuccin-macchiato/starship.toml"),
            include_str!("../../themes/catppuccin-macchiato/.center.toml"),
            include_str!("../../themes/catppuccin-macchiato/.right.toml"),
            include_str!("../../themes/catppuccin-macchiato/.full.toml")
        ),
        builtin_theme!(
            "catppuccin-frappe",
            "Catppuccin Frappé",
            "Soft muted dark theme balancing warmth and cool undertones",
            "Catppuccin Org",
            ThemeVariant::Dark,
            " • ",
            [("Teal", "#81c8be"), ("Blue", "#8caaee"), ("Green", "#a6d189"), ("Peach", "#ef9f76"), ("Mauve", "#ca9ee6")],
            include_str!("../../themes/catppuccin-frappe/starship.toml"),
            include_str!("../../themes/catppuccin-frappe/.center.toml"),
            include_str!("../../themes/catppuccin-frappe/.right.toml"),
            include_str!("../../themes/catppuccin-frappe/.full.toml")
        ),
        builtin_theme!(
            "catppuccin-latte",
            "Catppuccin Latte",
            "Crisp, bright light theme with cheerful and soft contrast",
            "Catppuccin Org",
            ThemeVariant::Light,
            " • ",
            [("Teal", "#179299"), ("Blue", "#1e66f5"), ("Green", "#40a02b"), ("Peach", "#fe640b"), ("Mauve", "#8839ef")],
            include_str!("../../themes/catppuccin-latte/starship.toml"),
            include_str!("../../themes/catppuccin-latte/.center.toml"),
            include_str!("../../themes/catppuccin-latte/.right.toml"),
            include_str!("../../themes/catppuccin-latte/.full.toml")
        ),
        builtin_theme!(
            "tokyo-night",
            "Tokyo Night",
            "A clean, dark Neovim-inspired theme that celebrates the lights of Downtown Tokyo",
            "Tokyo Night",
            ThemeVariant::Dark,
            " • ",
            [("Blue", "#7aa2f7"), ("Cyan", "#7dcfff"), ("Green", "#73daca"), ("Magenta", "#bb9af7"), ("Yellow", "#e0af68")],
            include_str!("../../themes/tokyo-night/starship.toml"),
            include_str!("../../themes/tokyo-night/.center.toml"),
            include_str!("../../themes/tokyo-night/.right.toml"),
            include_str!("../../themes/tokyo-night/.full.toml")
        ),
        builtin_theme!(
            "tokyo-night-moon",
            "Tokyo Night Moon",
            "Dark blue hue variant of Tokyo Night with midnight aesthetics",
            "Tokyo Night",
            ThemeVariant::Dark,
            " • ",
            [("Blue", "#82aaff"), ("Teal", "#4fd6be"), ("Green", "#c3e88d"), ("Pink", "#fca7ea"), ("Yellow", "#ffc777")],
            include_str!("../../themes/tokyo-night-moon/starship.toml"),
            include_str!("../../themes/tokyo-night-moon/.center.toml"),
            include_str!("../../themes/tokyo-night-moon/.right.toml"),
            include_str!("../../themes/tokyo-night-moon/.full.toml")
        ),
        builtin_theme!(
            "nord",
            "Nord",
            "An arctic, north-bluish clean color palette for an optimal visual workflow",
            "Arctic Ice Studio",
            ThemeVariant::Dark,
            " • ",
            [("Frost Blue", "#88c0d0"), ("Dark Frost", "#81a1c1"), ("Aurora Green", "#a3be8c"), ("Aurora Yellow", "#ebcb8b"), ("Aurora Purple", "#b48ead")],
            include_str!("../../themes/nord/starship.toml"),
            include_str!("../../themes/nord/.center.toml"),
            include_str!("../../themes/nord/.right.toml"),
            include_str!("../../themes/nord/.full.toml")
        ),
        builtin_theme!(
            "gruvbox-dark",
            "Gruvbox Dark",
            "Retro groove warm color scheme with earthy autumn tones",
            "morhetz",
            ThemeVariant::Dark,
            " • ",
            [("Yellow", "#fabd2f"), ("Green", "#b8bb26"), ("Aqua", "#8ec07c"), ("Blue", "#83a598"), ("Orange", "#fe8019")],
            include_str!("../../themes/gruvbox-dark/starship.toml"),
            include_str!("../../themes/gruvbox-dark/.center.toml"),
            include_str!("../../themes/gruvbox-dark/.right.toml"),
            include_str!("../../themes/gruvbox-dark/.full.toml")
        ),
        builtin_theme!(
            "gruvbox-light",
            "Gruvbox Light",
            "Light variant of retro groove warm palette with mellow parchment tones",
            "morhetz",
            ThemeVariant::Light,
            " • ",
            [("Yellow", "#d79921"), ("Green", "#98971a"), ("Aqua", "#689d6a"), ("Blue", "#458588"), ("Orange", "#af3a03")],
            include_str!("../../themes/gruvbox-light/starship.toml"),
            include_str!("../../themes/gruvbox-light/.center.toml"),
            include_str!("../../themes/gruvbox-light/.right.toml"),
            include_str!("../../themes/gruvbox-light/.full.toml")
        ),
        builtin_theme!(
            "dracula",
            "Dracula",
            "Famous dark theme for hackers with iconic purple, cyan and neon accents",
            "Zeno Rocha",
            ThemeVariant::Dark,
            " • ",
            [("Purple", "#bd93f9"), ("Cyan", "#8be9fd"), ("Green", "#50fa7b"), ("Pink", "#ff79c6"), ("Yellow", "#f1fa8c")],
            include_str!("../../themes/dracula/starship.toml"),
            include_str!("../../themes/dracula/.center.toml"),
            include_str!("../../themes/dracula/.right.toml"),
            include_str!("../../themes/dracula/.full.toml")
        ),
        builtin_theme!(
            "kanagawa",
            "Kanagawa",
            "Elegant dark theme inspired by the colors of Katsushika Hokusai's woodblock print",
            "rebelot",
            ThemeVariant::Dark,
            " • ",
            [("Wave Blue", "#7e9cd8"), ("Spring Green", "#98bb6c"), ("Surimi Orange", "#ffa066"), ("Sakura Pink", "#d27e99"), ("Oni Violet", "#957fb8")],
            include_str!("../../themes/kanagawa/starship.toml"),
            include_str!("../../themes/kanagawa/.center.toml"),
            include_str!("../../themes/kanagawa/.right.toml"),
            include_str!("../../themes/kanagawa/.full.toml")
        ),
        builtin_theme!(
            "onedark",
            "One Dark",
            "Atom's iconic dark theme with perfectly balanced syntax colors",
            "Atom / GitHub",
            ThemeVariant::Dark,
            " • ",
            [("Blue", "#61afef"), ("Green", "#98c379"), ("Purple", "#c678dd"), ("Yellow", "#e5c07b"), ("Cyan", "#56b6c2")],
            include_str!("../../themes/onedark/starship.toml"),
            include_str!("../../themes/onedark/.center.toml"),
            include_str!("../../themes/onedark/.right.toml"),
            include_str!("../../themes/onedark/.full.toml")
        ),
        builtin_theme!(
            "solarized-dark",
            "Solarized Dark",
            "Precision color scheme designed for prolonged eye comfort in terminal workstations",
            "Ethan Schoonover",
            ThemeVariant::Dark,
            " • ",
            [("Blue", "#268bd2"), ("Cyan", "#2aa198"), ("Green", "#859900"), ("Yellow", "#b58900"), ("Violet", "#6c71c4")],
            include_str!("../../themes/solarized-dark/starship.toml"),
            include_str!("../../themes/solarized-dark/.center.toml"),
            include_str!("../../themes/solarized-dark/.right.toml"),
            include_str!("../../themes/solarized-dark/.full.toml")
        ),
        builtin_theme!(
            "solarized-light",
            "Solarized Light",
            "Scientific light color scheme carefully balanced for optimal readability",
            "Ethan Schoonover",
            ThemeVariant::Light,
            " • ",
            [("Blue", "#268bd2"), ("Cyan", "#2aa198"), ("Green", "#859900"), ("Yellow", "#b58900"), ("Violet", "#6c71c4")],
            include_str!("../../themes/solarized-light/starship.toml"),
            include_str!("../../themes/solarized-light/.center.toml"),
            include_str!("../../themes/solarized-light/.right.toml"),
            include_str!("../../themes/solarized-light/.full.toml")
        ),
    ]
}

fn load_theme_from_dir(dir: &Path, id: &str) -> Option<Theme> {
    if !dir.is_dir() {
        return None;
    }

    let meta_path = dir.join("theme.toml");
    let meta: ThemeMeta = if meta_path.is_file() {
        let content = fs::read_to_string(meta_path).ok()?;
        toml::from_str(&content).ok()?
    } else {
        ThemeMeta {
            name: id.to_string(),
            description: String::new(),
            author: "Custom".to_string(),
            variant: ThemeVariant::Dark,
            window_separator: " • ".to_string(),
            swatches: Vec::new(),
        }
    };

    let read_side = |names: &[&str]| -> String {
        for name in names {
            let p = dir.join(name);
            if p.is_file() {
                if let Ok(c) = fs::read_to_string(p) {
                    return c;
                }
            }
        }
        String::new()
    };

    let left = read_side(&["starship.toml", ".left.toml", "left.toml"]);
    let center = read_side(&[".center.toml", "center.toml"]);
    let right = read_side(&[".right.toml", "right.toml"]);
    let full = read_side(&[".full.toml", "full.toml"]);

    Some(Theme {
        id: id.to_string(),
        name: meta.name,
        description: meta.description,
        author: meta.author,
        variant: meta.variant,
        window_separator: meta.window_separator,
        swatches: meta.swatches,
        left_toml: left,
        center_toml: center,
        right_toml: right,
        full_toml: full,
        is_custom: true,
    })
}

pub fn custom_theme_dirs(env: &HashMap<String, String>) -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Some(custom_dir) = env.get("TMUX_SHIP_THEMES_DIR") {
        dirs.push(PathBuf::from(custom_dir));
    }
    if let Some(xdg) = env.get("XDG_CONFIG_HOME") {
        dirs.push(PathBuf::from(xdg).join("tmuxship/themes"));
        dirs.push(PathBuf::from(xdg).join("tmux/themes"));
    }
    if let Some(home) = env.get("HOME") {
        let h = PathBuf::from(home);
        dirs.push(h.join(".config/tmuxship/themes"));
        dirs.push(h.join(".tmux/themes"));
    } else if let Some(home_dir) = dirs::home_dir() {
        dirs.push(home_dir.join(".config/tmuxship/themes"));
        dirs.push(home_dir.join(".tmux/themes"));
    }
    dirs
}

#[allow(dead_code)]
pub fn find_theme(name_or_id: &str) -> Option<Theme> {
    find_theme_with_env(name_or_id, &HashMap::new())
}

pub fn find_theme_with_env(name_or_id: &str, env: &HashMap<String, String>) -> Option<Theme> {
    let query = name_or_id.trim().to_lowercase().replace(' ', "-");

    // 1. Check custom theme directories first
    for base in custom_theme_dirs(env) {
        if base.is_dir() {
            let direct = base.join(&query);
            if let Some(theme) = load_theme_from_dir(&direct, &query) {
                return Some(theme);
            }
        }
    }

    // 2. Fall back to built-in themes
    let builtins = builtin_themes();
    builtins.into_iter().find(|t| {
        t.id == query || t.id.replace('-', "") == query.replace('-', "") || t.name.to_lowercase() == query
    })
}

pub fn all_themes() -> Vec<Theme> {
    all_themes_with_env(&HashMap::new())
}

pub fn all_themes_with_env(env: &HashMap<String, String>) -> Vec<Theme> {
    let mut map: HashMap<String, Theme> = HashMap::new();

    // 1. Add built-in themes
    for t in builtin_themes() {
        map.insert(t.id.clone(), t);
    }

    // 2. Add custom themes from disk (overrides or adds to list)
    for base in custom_theme_dirs(env) {
        if let Ok(entries) = fs::read_dir(base) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(id) = path.file_name().and_then(|n| n.to_str()) {
                        if let Some(custom) = load_theme_from_dir(&path, id) {
                            map.insert(custom.id.clone(), custom);
                        }
                    }
                }
            }
        }
    }

    let mut result: Vec<Theme> = map.into_values().collect();
    result.sort_by(|a, b| a.id.cmp(&b.id));
    result
}
