use clap::Parser;
use cli::{Cli, Command};
use color_eyre::eyre::Result;
use confique::Config;
use itertools::Itertools;

use crate::config::Conf;

mod cli;
mod config;
mod mra;

fn main() -> Result<()> {
    let args = Cli::parse();

    let mut config = Conf::builder().env();
    for path in args.config {
        config = config.file(path);
    }
    let config = config.load()?;

    match args.command {
        Command::Pull(cmd) => {
            let invalid_pairs = cmd
                .pairs
                .iter()
                .filter(|pair| !config.pairs.contains_key(*pair))
                .collect::<Vec<_>>();
            if !invalid_pairs.is_empty() {
                println!("Invalid pairs: {}", invalid_pairs.iter().join(", "));
                return Ok(());
            }

            let pairs = if cmd.pairs.is_empty() {
                config.pairs.keys().cloned().collect()
            } else {
                cmd.pairs
            };

            for pair in pairs {
                let config_pair = &config.pairs[&pair];

                mra::pull(&config_pair.source, &config_pair.destination)?;
            }
        }
    }

    Ok(())
}
