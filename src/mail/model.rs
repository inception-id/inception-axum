use lettre::message::{header::ContentType, Mailbox};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Address, Message, SmtpTransport, Transport};

use std::env;
use std::io::ErrorKind;
pub struct Mail;

impl Mail {
    fn build_message(
        email: &str,
        subject: &str,
        content: &str,
    ) -> Result<Message, lettre::error::Error> {
        let smtp_from = env::var("SMTP_FROM").expect("Missing SMTP_FROM");
        let smtp_from_address =
            Address::new(&smtp_from, &smtp_from).expect("Missing SMTP_FROM_ADDRESS");
        let smtp_to_address = Address::new(email, email).expect("Missing SMTP_TO_ADDRESS");
        Message::builder()
            .from(Mailbox::new(Some(smtp_from), smtp_from_address))
            .to(Mailbox::new(None, smtp_to_address))
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(String::from(content))
    }

    fn build_smtp() -> Result<SmtpTransport, lettre::transport::smtp::Error> {
        let smtp_username = env::var("SMTP_USERNAME").expect("Missing SMTP_USERNAME");
        let smtp_password = env::var("SMTP_PASSWORD").expect("Missing SMTP_PASSWORD");
        let smtp_host = env::var("SMTP_HOST").expect("Missing SMTP_HOST");
        let smtp_port = env::var("SMTP_PORT")
            .expect("Missing SMTP_PORT")
            .parse::<u16>()
            .expect("Invalid SMTP_PORT");
        let creds = Credentials::new(smtp_username, smtp_password);

        match SmtpTransport::relay(&smtp_host) {
            Ok(relay) => Ok(relay.credentials(creds).port(smtp_port).build()),
            Err(err) => return Err(err),
        }
    }
    pub fn send(
        email: &str,
        subject: &str,
        content: &str,
    ) -> Result<lettre::transport::smtp::response::Response, lettre::error::Error> {
        let message = Self::build_message(email, subject, content)?;

        let smtp = match Self::build_smtp() {
            Ok(transport) => transport,
            Err(err) => {
                let new_error = std::io::Error::new(ErrorKind::Other, err.to_string());
                return Err(lettre::error::Error::Io(new_error));
            }
        };

        match smtp.send(&message) {
            Ok(res) => Ok(res),
            Err(err) => {
                let new_error = std::io::Error::new(ErrorKind::Other, err.to_string());
                return Err(lettre::error::Error::Io(new_error));
            }
        }
    }
}
