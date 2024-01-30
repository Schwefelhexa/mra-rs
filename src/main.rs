use clap::Parser;
use cli::Cli;
use confique::Config;

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

    println!("{:#?}", config);
}
