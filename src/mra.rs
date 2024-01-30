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
        vec!["Date: Mon, 30 Jan 2024 12:34:56 +0000
From: John Doe <john.doe@example.com>
To: Jane Smith <jane.smith@example.net>
Subject: Hello, Jane!

Hi Jane,

I hope this email finds you well. Just wanted to say hello and see how you're doing.

Best regards,
John"
            .into()]
    }
}

pub trait Destination {
    fn push(&self, mail: &str);
}
impl Destination for MraDestination {
    fn push(&self, mail: &str) {
        match self {
            MraDestination::Maildir(destination) => destination.push(mail),
        }
    }
}
impl Destination for MaildirDestination {
    fn push(&self, mail: &str) {
        println!("Pushing email to Maildir at {}\n", self.path);
        println!("{}\n\n", mail);
    }
}

pub fn pull<S: Source, D: Destination>(source: &S, destination: &D) {
    let mails = source.pull();

    for mail in mails {
        destination.push(&mail);
    }
}
