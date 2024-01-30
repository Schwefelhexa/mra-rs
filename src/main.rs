use confique::Config;

use crate::config::Conf;

mod config;

fn main() {
    let config = Conf::builder()
        .env()
        .file(std::env::current_dir().unwrap().join("config.example.toml"))
        .load()
        .unwrap();

    println!("{:#?}", config);
}
