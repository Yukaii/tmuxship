mod config;
mod render;
mod tmux_conf;

use anyhow::Result;
use clap::{ArgAction, Parser, Subcommand};
use config::{resolve_config, Side};
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
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
    EmitTmuxConf,

    /// Apply generated tmux config to the current tmux server
    Apply,
}

#[derive(Parser, Debug)]
struct RenderArgs {
    /// Force a specific Starship config file for this invocation only
    #[arg(long, value_name = "PATH")]
    config: Option<PathBuf>,

    /// Print the resolved config path without rendering (useful for debugging)
    #[arg(long, action = ArgAction::SetTrue)]
    show_config: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let env: HashMap<String, String> = env::vars().collect();

    match cli.command {
        CliCommand::Left(args) => render_side(Side::Left, args, &env),
        CliCommand::Right(args) => render_side(Side::Right, args, &env),
        CliCommand::Center(args) => render_side(Side::Center, args, &env),
        CliCommand::Full(args) => render_side(Side::Full, args, &env),
        CliCommand::EmitTmuxConf => {
            let options = tmux_conf::emit_tmux_conf(&env)?;
            println!("{}", tmux_conf::format_tmux_conf(&options));
            Ok(())
        }
        CliCommand::Apply => {
            let options = tmux_conf::emit_tmux_conf(&env)?;
            tmux_conf::apply_tmux_conf(&options)
        }
    }
}

fn render_side(side: Side, args: RenderArgs, env: &HashMap<String, String>) -> Result<()> {
    let config = resolve_config(side, args.config, env)?;

    if args.show_config {
        println!("{}", config.config_path.display());
        return Ok(());
    }

    let ansi = render::run_starship(&config, env)?;
    let rendered = render::render_from_ansi(&ansi);
    print!("{}", rendered);
    Ok(())
}
