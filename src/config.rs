use std::{collections::HashMap, path::PathBuf};

use confique::Config;
use serde::Deserialize;

#[derive(Config, Debug)]
pub struct Conf {
    pub pairs: HashMap<String, MraPair>
}

#[derive(Debug, Deserialize)]
pub struct MraPair {
    pub source: MraSource,
    pub destination: MraDestination,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum MraSource {
    #[serde(rename = "imap")]
    Imap(ImapSource),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum MraDestination {
    #[serde(rename = "maildir")]
    Maildir(MaildirDestination),
}

#[derive(Debug, Deserialize)]
pub struct ImapSource {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct MaildirDestination {
    pub path: PathBuf,
}
