mod config;
mod render;

use anyhow::Result;
use clap::{ArgAction, Parser, ValueEnum};
use config::{resolve_config, Side};
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Which side to render (left, right, full)
    side: CliSide,

    /// Force a specific Starship config file for this invocation only
    #[arg(long, value_name = "PATH")]
    config: Option<PathBuf>,

    /// Print the resolved config path without rendering (useful for debugging)
    #[arg(long, action = ArgAction::SetTrue)]
    show_config: bool,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum CliSide {
    Left,
    Right,
    Full,
}

impl From<CliSide> for Side {
    fn from(value: CliSide) -> Self {
        match value {
            CliSide::Left => Side::Left,
            CliSide::Right => Side::Right,
            CliSide::Full => Side::Full,
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let env: HashMap<String, String> = env::vars().collect();
    let side: Side = cli.side.into();
    let config = resolve_config(side, cli.config, &env)?;

    if cli.show_config {
        println!("{}", config.config_path.display());
        return Ok(());
    }

    let json = render::run_starship(&config, &env)?;
    let rendered = render::render_from_json(&json)?;
    print!("{}", rendered);
    Ok(())
}
