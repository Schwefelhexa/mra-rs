use clap::Parser;
use cli::{Cli, Command};
use confique::Config;
use itertools::Itertools;

use crate::config::Conf;

mod cli;
mod config;

fn main() {
    let args = Cli::parse();

    let mut config = Conf::builder().env();
    for path in args.config {
        config = config.file(path);
    }
    let config = config.load().unwrap();

    match args.command {
        Command::Pull(cmd) => {
            let invalid_pairs = cmd
                .pairs
                .iter()
                .filter(|pair| !config.pairs.contains_key(*pair))
                .collect::<Vec<_>>();
            if !invalid_pairs.is_empty() {
                println!("Invalid pairs: {}", invalid_pairs.iter().join(", "));
                return;
            }

            println!("Pulling emails from source(s) to destination(s).");

            println!("{:#?}", cmd);
            println!("{:#?}", config);
        }
    }
}
