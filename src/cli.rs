use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to configuration file(s).
    /// Supports JSON, TOML and YAML.
    /// Higher priority files are loaded first.
    #[clap(short, long)]
    pub config: Vec<PathBuf>,
}
