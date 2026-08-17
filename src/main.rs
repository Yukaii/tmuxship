mod config;
mod render;
mod theme;
mod tmux_conf;

use anyhow::{anyhow, Result};
use clap::{ArgAction, Parser, Subcommand, ValueEnum};
use config::{resolve_config_with_theme, Side};
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use theme::catalog::{all_themes_with_env, find_theme_with_env};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "A Starship-to-tmux adapter and legendary theme suite for beautiful tmux status bars"
)]
struct Cli {
    #[command(subcommand)]
    command: CliCommand,
}

#[derive(Subcommand, Debug)]
enum CliCommand {
    /// Render left status
    Left(RenderArgs),

    /// Render right status
    Right(RenderArgs),

    /// Render center/window status
    Center(RenderArgs),

    /// Render all sides
    Full(RenderArgs),

    /// Print tmux config that delegates tmux-native values to tmux
    EmitTmuxConf(ConfArgs),

    /// Apply generated tmux config to the current tmux server
    Apply(ConfArgs),

    /// Explore, preview, export, and install built-in themes
    #[command(subcommand)]
    Theme(ThemeCommand),

    /// Print tmux.conf initialization snippet for a theme
    Init(InitArgs),
}

#[derive(Parser, Debug, Clone, Default)]
struct RenderArgs {
    /// Force a specific Starship config file for this invocation only
    #[arg(long, value_name = "PATH")]
    config: Option<PathBuf>,

    /// Use a specific built-in theme (e.g. rose-pine, catppuccin-mocha, nord)
    #[arg(long, value_name = "THEME")]
    theme: Option<String>,

    /// Print the resolved config path without rendering (useful for debugging)
    #[arg(long, action = ArgAction::SetTrue)]
    show_config: bool,
}

#[derive(Parser, Debug, Clone, Default)]
struct ConfArgs {
    /// Use a specific built-in theme (e.g. rose-pine, catppuccin-mocha, nord)
    #[arg(long, value_name = "THEME")]
    theme: Option<String>,
}

#[derive(Parser, Debug)]
struct InitArgs {
    /// Theme name or ID to generate configuration for (e.g. rose-pine, catppuccin-mocha)
    #[arg(default_value = "rose-pine")]
    theme: String,
}

#[derive(Subcommand, Debug)]
enum ThemeCommand {
    /// List all available built-in themes
    #[command(name = "list", alias = "ls")]
    List {
        /// Output in JSON format
        #[arg(long)]
        json: bool,

        /// Filter themes by name, style, or variant (e.g. 'rose', 'dark', 'light', 'catppuccin')
        #[arg(short, long)]
        filter: Option<String>,
    },

    /// Preview themes with realistic terminal status bar mockups
    Preview {
        /// Specific theme to preview (previews all if omitted)
        theme: Option<String>,

        /// Filter themes to preview (e.g. 'dark', 'light', 'nord')
        #[arg(short, long)]
        filter: Option<String>,
    },

    /// Show TOML configuration for a theme
    Show {
        /// Theme name or ID (e.g. rose-pine, catppuccin-mocha)
        theme: String,

        /// Specific side to display
        #[arg(short, long, value_enum, default_value = "all")]
        side: ShowSide,
    },

    /// Export theme TOML configuration files to a directory
    Export {
        /// Theme name or ID
        theme: String,

        /// Target destination directory (defaults to current directory)
        #[arg(short, long, default_value = ".")]
        dir: PathBuf,
    },

    /// Install a theme into ~/.tmux/ or target directory
    Install {
        /// Theme name or ID
        theme: String,

        /// Target destination directory (defaults to ~/.tmux)
        #[arg(short, long)]
        dir: Option<PathBuf>,

        /// Overwrite existing files
        #[arg(short, long)]
        force: bool,
    },

    /// Print tmux.conf snippet for quick activation of a theme
    Init {
        /// Theme name or ID
        #[arg(default_value = "rose-pine")]
        theme: String,
    },
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
enum ShowSide {
    Left,
    Center,
    Right,
    Full,
    All,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let env: HashMap<String, String> = env::vars().collect();

    match cli.command {
        CliCommand::Left(args) => render_side(Side::Left, args, &env),
        CliCommand::Right(args) => render_side(Side::Right, args, &env),
        CliCommand::Center(args) => render_side(Side::Center, args, &env),
        CliCommand::Full(args) => render_side(Side::Full, args, &env),
        CliCommand::EmitTmuxConf(args) => {
            let options = tmux_conf::emit_tmux_conf_with_theme(args.theme.as_deref(), &env)?;
            println!("{}", tmux_conf::format_tmux_conf(&options));
            Ok(())
        }
        CliCommand::Apply(args) => {
            let options = tmux_conf::emit_tmux_conf_with_theme(args.theme.as_deref(), &env)?;
            tmux_conf::apply_tmux_conf(&options)
        }
        CliCommand::Init(args) => {
            let theme = find_theme_with_env(&args.theme, &env)
                .ok_or_else(|| anyhow!("Theme '{}' not found. Run `tmuxship theme list` to see available themes.", args.theme))?;
            print!("{}", theme::generate_init_snippet(&theme));
            Ok(())
        }
        CliCommand::Theme(cmd) => handle_theme_command(cmd, &env),
    }
}

fn render_side(side: Side, args: RenderArgs, env: &HashMap<String, String>) -> Result<()> {
    let config = resolve_config_with_theme(side, args.config, args.theme.as_deref(), env)?;

    if args.show_config {
        println!("{}", config.config_path.display());
        return Ok(());
    }

    let ansi = render::run_starship(&config, env)?;
    let rendered = render::render_from_ansi(&ansi);
    print!("{}", rendered);
    Ok(())
}

fn handle_theme_command(cmd: ThemeCommand, env: &HashMap<String, String>) -> Result<()> {
    match cmd {
        ThemeCommand::List { json, filter } => {
            let themes = all_themes_with_env(env);
            let filter_lower = filter.as_ref().map(|f| f.to_lowercase());

            let filtered: Vec<_> = themes
                .iter()
                .filter(|t| {
                    if let Some(ref q) = filter_lower {
                        t.id.to_lowercase().contains(q)
                            || t.name.to_lowercase().contains(q)
                            || t.variant.to_string().contains(q)
                            || t.author.to_lowercase().contains(q)
                    } else {
                        true
                    }
                })
                .collect();

            if json {
                let serialized = serde_json::to_string_pretty(&filtered)?;
                println!("{}", serialized);
                return Ok(());
            }

            println!("\x1b[1m\x1b[38;2;137;180;250mAvailable tmuxship Themes ({})\x1b[0m\n", filtered.len());
            println!(
                "  {:<22} {:<24} {:<8} {:<18}",
                "ID", "NAME", "VARIANT", "AUTHOR"
            );
            println!("  {}", "─".repeat(74));

            for t in &filtered {
                let variant_str = match t.variant {
                    theme::ThemeVariant::Dark => "\x1b[38;2;108;112;134mdark\x1b[0m",
                    theme::ThemeVariant::Light => "\x1b[38;2;223;142;29mlight\x1b[0m",
                };
                let custom_marker = if t.is_custom { " *" } else { "" };
                println!(
                    "  \x1b[1m{:<22}\x1b[0m {:<24} {:<17} {}{}",
                    t.id, t.name, variant_str, t.author, custom_marker
                );
            }

            println!(
                "\n\x1b[2mUse `tmuxship theme preview [ID]` to view rich previews.\x1b[0m"
            );
            println!(
                "\x1b[2mUse `tmuxship apply --theme [ID]` to apply immediately.\x1b[0m"
            );
            Ok(())
        }
        ThemeCommand::Preview { theme, filter } => {
            if let Some(ref name) = theme {
                let t = find_theme_with_env(name, env).ok_or_else(|| {
                    anyhow!(
                        "Theme '{}' not found. Run `tmuxship theme list` to see available themes.",
                        name
                    )
                })?;
                theme::preview::display_theme_preview(&t);
                Ok(())
            } else {
                theme::preview::preview_themes(filter.as_deref())?;
                Ok(())
            }
        }
        ThemeCommand::Show { theme, side } => {
            let t = find_theme_with_env(&theme, env).ok_or_else(|| {
                anyhow!(
                    "Theme '{}' not found. Run `tmuxship theme list` to see available themes.",
                    theme
                )
            })?;

            match side {
                ShowSide::Left => print!("{}", t.left_toml),
                ShowSide::Center => print!("{}", t.center_toml),
                ShowSide::Right => print!("{}", t.right_toml),
                ShowSide::Full => print!("{}", t.full_toml),
                ShowSide::All => {
                    println!("# === Left Status (starship.toml) ===\n{}", t.left_toml);
                    println!("# === Center / Window Status (.center.toml) ===\n{}", t.center_toml);
                    println!("# === Right Status (.right.toml) ===\n{}", t.right_toml);
                }
            }
            Ok(())
        }
        ThemeCommand::Export { theme, dir } => {
            let t = find_theme_with_env(&theme, env).ok_or_else(|| {
                anyhow!(
                    "Theme '{}' not found. Run `tmuxship theme list` to see available themes.",
                    theme
                )
            })?;
            theme::export_theme(&t, &dir)?;
            println!(
                "\x1b[32m✔\x1b[0m Exported '{}' config files to {}",
                t.name,
                dir.display()
            );
            Ok(())
        }
        ThemeCommand::Install { theme, dir, force } => {
            let t = find_theme_with_env(&theme, env).ok_or_else(|| {
                anyhow!(
                    "Theme '{}' not found. Run `tmuxship theme list` to see available themes.",
                    theme
                )
            })?;
            let dest = theme::install_theme(&t, dir.as_deref(), force)?;
            println!(
                "\x1b[32m✔\x1b[0m Installed '{}' theme to {}",
                t.name,
                dest.display()
            );
            println!("\nAdd to your ~/.tmux.conf:");
            println!("  setenv -g TMUX_SHIP_LEFT_CONFIG   \"{}/starship.toml\"", dest.display());
            println!("  setenv -g TMUX_SHIP_RIGHT_CONFIG  \"{}/.right.toml\"", dest.display());
            println!("  setenv -g TMUX_SHIP_CENTER_CONFIG \"{}/.center.toml\"", dest.display());
            println!("  run-shell 'tmuxship apply'\n");
            Ok(())
        }
        ThemeCommand::Init { theme } => {
            let t = find_theme_with_env(&theme, env).ok_or_else(|| {
                anyhow!(
                    "Theme '{}' not found. Run `tmuxship theme list` to see available themes.",
                    theme
                )
            })?;
            print!("{}", theme::generate_init_snippet(&t));
            Ok(())
        }
    }
}
