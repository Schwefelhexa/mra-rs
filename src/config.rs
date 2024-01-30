use std::collections::HashMap;

use confique::Config;
use serde::Deserialize;

#[derive(Config, Debug)]
pub struct Conf {
    pairs: HashMap<String, MraPair>
}

#[derive(Debug, Deserialize)]
pub struct MraPair {
    source: MraSource,
    destination: MraDestination,
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
    host: String,
    port: u16,
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
pub struct MaildirDestination {
    path: String,
}
