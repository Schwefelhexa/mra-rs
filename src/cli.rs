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

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Parser, Debug)]
pub enum Command {
    /// Pull emails from source(s) to destination(s).
    Pull(PullCommand)
}

#[derive(Parser, Debug)]
pub struct PullCommand {
    /// Which pairs to pull.
    /// If not specified, all pairs are pulled.
    #[clap(short, long)]
    pub pairs: Vec<String>,
}
