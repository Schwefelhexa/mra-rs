use color_eyre::eyre::Result;
use itertools::Itertools;
use maildir::Maildir;
use native_tls::TlsConnector;

use crate::config::{ImapSource, MaildirDestination, MraDestination, MraSource};

pub trait Source {
    /// Returns a Vec of Strings, each containing an email in RFC 5322 format.
    fn pull(&self) -> Result<Vec<String>>;
}
impl Source for MraSource {
    fn pull(&self) -> Result<Vec<String>> {
        match self {
            Self::Imap(source) => source.pull(),
        }
    }
}
impl Source for ImapSource {
    fn pull(&self) -> Result<Vec<String>> {
        let tls = TlsConnector::new()?;
        let mut session = imap::connect((self.host()?, *self.port()), self.host()?, &tls)?
            .login(self.username()?, self.password()?)
            .map_err(|e| e.0)?;

        session.select("INBOX")?;
        let unread_message_ids = session.search("UNSEEN")?.iter().join(",");
        println!("Unread message IDs: {unread_message_ids}");
        let unread_messages = session
            .fetch(&unread_message_ids, "RFC822")?
            .iter()
            .filter_map(|message| message.body())
            .map(|body| String::from_utf8(body.to_vec()))
            .filter_map(Result::ok)
            .collect_vec();

        Ok(unread_messages)
    }
}

pub trait Destination {
    fn push(&self, mails: Vec<&str>) -> Result<()>;
}
impl Destination for MraDestination {
    fn push(&self, mails: Vec<&str>) -> Result<()> {
        match self {
            Self::Maildir(destination) => destination.push(mails),
        }
    }
}
impl Destination for MaildirDestination {
    fn push(&self, mails: Vec<&str>) -> Result<()> {
        let maildir = Maildir::from(self.path());
        maildir.create_dirs()?;

        for mail in mails {
            maildir.store_new(mail.as_bytes())?;
        }

        Ok(())
    }
}

pub fn pull<S: Source, D: Destination>(source: &S, destination: &D) -> Result<()> {
    let mails = source.pull()?;

    destination.push(mails.iter().map(String::as_str).collect())
}
