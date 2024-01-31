use std::{collections::HashMap, path::PathBuf, process::Command};

use confique::Config;
use getset::Getters;
use serde::Deserialize;

#[derive(Config, Debug)]
pub struct Conf {
    pub pairs: HashMap<String, MraPair>,
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
enum OptionalHost {
    #[serde(rename = "host")]
    Raw(String),
    #[serde(rename = "host_cmd")]
    FromCommand(String),
}

#[derive(Debug, Deserialize, Getters)]
pub struct ImapSource {
    #[serde(flatten)]
    host: OptionalHost,
    #[getset(get = "pub")]
    port: u16,
    pub username: String,
    pub password: String,
}
impl ImapSource {
    pub fn host(&self) -> String {
        match &self.host {
            OptionalHost::Raw(v) => v.clone(),
            OptionalHost::FromCommand(cmd) => {
                let result = Command::new("sh")
                    .arg("-c")
                    .arg(cmd)
                    .output()
                    .unwrap()
                    .stdout;
                String::from_utf8(result).unwrap().trim().to_owned()
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct MaildirDestination {
    pub path: PathBuf,
}
