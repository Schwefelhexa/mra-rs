use itertools::Itertools;
use maildir::Maildir;
use native_tls::TlsConnector;

use crate::config::{ImapSource, MaildirDestination, MraDestination, MraSource};

pub trait Source {
    /// Returns a Vec of Strings, each containing an email in RFC 5322 format.
    fn pull(&self) -> Vec<String>;
}
impl Source for MraSource {
    fn pull(&self) -> Vec<String> {
        match self {
            MraSource::Imap(source) => source.pull(),
        }
    }
}
impl Source for ImapSource {
    fn pull(&self) -> Vec<String> {
        let tls = TlsConnector::new().unwrap();
        let mut session =
            imap::connect((self.host(), *self.port()), self.host(), &tls)
                .unwrap()
                .login(self.username(), self.password())
                .unwrap();

        session.select("INBOX").unwrap();
        let unread_message_ids = session.search("UNSEEN").unwrap().iter().join(" ");
        let unread_messages = session
            .fetch(&unread_message_ids, "RFC822")
            .unwrap()
            .iter()
            .map(|message| message.body().unwrap())
            .map(|body| String::from_utf8(body.to_vec()).unwrap())
            .collect();

        unread_messages
    }
}

pub trait Destination {
    fn push(&self, mails: Vec<&str>);
}
impl Destination for MraDestination {
    fn push(&self, mails: Vec<&str>) {
        match self {
            MraDestination::Maildir(destination) => destination.push(mails),
        }
    }
}
impl Destination for MaildirDestination {
    fn push(&self, mails: Vec<&str>) {
        let maildir = Maildir::from(self.path.clone());
        maildir.create_dirs().unwrap();

        for mail in mails {
            maildir.store_new(mail.as_bytes()).unwrap();
        }
    }
}

pub fn pull<S: Source, D: Destination>(source: &S, destination: &D) {
    let mails = source.pull();

    destination.push(mails.iter().map(|mail| mail.as_str()).collect());
}
