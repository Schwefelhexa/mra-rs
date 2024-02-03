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

macro_rules! cmd {
    ($id:ident, $name:expr, $name_cmd:expr) => {
        #[derive(Debug, Deserialize)]
        enum $id {
            #[serde(rename = $name)]
            Raw(String),
            #[serde(rename = $name_cmd)]
            FromCommand(String),
        }
        impl $id {
            pub fn eval(&self) -> String {
                match self {
                    $id::Raw(v) => v.clone(),
                    $id::FromCommand(cmd) => {
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
    };
}

macro_rules! get_cmd {
    ($enum:ident, $field:ident) => {
        pub fn $field(&self) -> String {
            self.$field.eval()
        }
    };
}

cmd!(Host, "host", "host_cmd");
cmd!(Username, "username", "username_cmd");
cmd!(Password, "password", "password_cmd");

#[derive(Debug, Deserialize, Getters)]
pub struct ImapSource {
    #[serde(flatten)]
    host: Host,
    #[getset(get = "pub")]
    port: u16,
    #[serde(flatten)]
    username: Username,
    #[serde(flatten)]
    password: Password,
}
impl ImapSource {
    get_cmd!(Host, host);
    get_cmd!(Username, username);
    get_cmd!(Password, password);
}

#[derive(Debug, Deserialize)]
pub struct MaildirDestination {
    path: String,
}
impl MaildirDestination {
    pub fn path(&self) -> PathBuf {
        let string = shellexpand::full(&self.path).unwrap_or_else(|_| self.path.clone().into());
        string.into_owned().into()
    }
}
