use crate::sendgrid::ContentType;
use std::error::Error;

pub mod sendgrid;

impl<'a> sendgrid::SendgridEmail<'a> {
    #[must_use]
    /// Create a new sendgrid instance.
    pub fn new(api_key: &'a str, to_email: &'a str, from_email: &'a str) -> Self {
        Self {
            api_key,
            personalizations: vec![sendgrid::Personalization {
                to: vec![sendgrid::From { email: to_email }],
                cc: None,
            }],
            from: sendgrid::From { email: from_email },
            subject: "",
            content: vec![sendgrid::Content {
                content_type: None,
                value: None,
            }],
        }
    }

    /// Sends an email using Sendgrid API.
    /// # Example
    /// ```
    /// use sendgrid_thin::sendgrid::SendgridEmail;
    /// let mut sendgrid = SendgridEmail::new("SENDGRID_API_KEY", "to_email@example.com", "from_email@example.com");
    /// match sendgrid.send("subject", "body") {
    ///    Ok(_) => println!("Email sent successfully"),
    ///    Err(err) => println!("Error sending email: {}", err),
    /// }
    /// ```
    pub fn send(&mut self, subject: &'a str, body: &'a str) -> Result<(), Box<dyn Error>> {
        ureq::post("https://api.sendgrid.com/v3/mail/send")
            .set("Authorization", &format!("Bearer {}", self.api_key))
            .set("Content-Type", "application/json")
            .send_string(&self.set_body_email(subject, body))?;
        Ok(())
    }

    /// Add a CC email to the email.
    ///
    /// Allow to send the email to multiple recipients.
    /// # Example
    /// ```
    /// use sendgrid_thin::sendgrid::SendgridEmail;
    /// let mut sendgrid = SendgridEmail::new("SENDGRID_API_KEY", "to_email@example.com", "from_email@example.com");
    /// sendgrid.add_cc_emails(&["cc_email1@example.com", "cc_email2@example.com"]);
    /// match sendgrid.send("subject", "body") {
    ///    Ok(_) => println!("Email sent successfully"),
    ///    Err(err) => println!("Error sending email: {}", err),
    /// }
    /// ```
    pub fn add_cc_emails(&mut self, cc_emails: &[&'a str]) {
        match self.personalizations[0].cc.as_mut() {
            Some(cc) => {
                for email in cc_emails {
                    cc.push(sendgrid::From { email: *email });
                }
            }
            None => {
                self.personalizations[0].cc = Some(
                    cc_emails
                        .iter()
                        .map(|email| sendgrid::From { email: *email })
                        .collect(),
                );
            }
        }
    }

    /// Set the content type of the email.
    /// # Example
    /// ```
    /// use sendgrid_thin::sendgrid::{SendgridEmail, ContentType};
    /// let mut sendgrid = SendgridEmail::new("SENDGRID_API_KEY", "to_email@example.com", "from_email@example.com");
    /// sendgrid.set_content_type(ContentType::Html);
    /// ```
    pub fn set_content_type(&mut self, content_type: ContentType) {
        self.content[0].content_type = match content_type {
            ContentType::Text => Some("text/plain"),
            ContentType::Html => Some("text/html"),
        };
    }

    fn set_body_email(&mut self, subject: &'a str, body: &'a str) -> String {
        if let None = self.content[0].content_type {
            self.content[0].content_type = Some("text/plain");
        }
        self.subject = subject;
        self.content[0].value = Some(body);
        serde_json::to_string(self).expect("Error serializing email")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sendgrid_instance() {
        let sendgrid = sendgrid::SendgridEmail::new(
            "SENDGRID_API_KEY",
            "to_email@example.com",
            "from_email@example.com",
        );
        assert_eq!(
            sendgrid,
            sendgrid::SendgridEmail {
                api_key: "SENDGRID_API_KEY",
                personalizations: vec![sendgrid::Personalization {
                    to: vec![sendgrid::From {
                        email: "to_email@example.com"
                    }],
                    cc: None,
                }],
                from: sendgrid::From {
                    email: "from_email@example.com"
                },
                subject: "",
                content: vec![sendgrid::Content {
                    content_type: None,
                    value: None,
                }],
            }
        );
    }

    #[test]
    fn test_content_type() {
        let mut sendgrid = sendgrid::SendgridEmail::new(
            "SENDGRID_API_KEY",
            "to_email@example.com",
            "from_email@example.com",
        );
        assert_eq!(sendgrid.content[0].content_type, None);
        sendgrid.set_content_type(ContentType::Html);
        assert_eq!(sendgrid.content[0].content_type, Some("text/html"));
        sendgrid.set_content_type(ContentType::Text);
        assert_eq!(sendgrid.content[0].content_type, Some("text/plain"));
    }

    #[test]
    fn test_set_body_email() {
        let mut sendgrid = sendgrid::SendgridEmail::new(
            "SENDGRID_API_KEY",
            "to_email@example.com",
            "from_email@example.com",
        );
        assert_eq!(sendgrid.set_body_email("subject_test", "body_test"), "{\"personalizations\":[{\"to\":[{\"email\":\"to_email@example.com\"}]}],\"from\":{\"email\":\"from_email@example.com\"},\"subject\":\"subject_test\",\"content\":[{\"type\":\"text/plain\",\"value\":\"body_test\"}]}");
    }

    #[test]
    fn test_set_body_email_with_cc_emails() {
        let mut sendgrid = sendgrid::SendgridEmail::new(
            "SENDGRID_API_KEY",
            "to_email@example.com",
            "from_email@example.com",
        );
        sendgrid.add_cc_emails(&["cc_email1@example.com", "cc_email2@example.com"]);
        assert_eq!(sendgrid.set_body_email("subject_test", "body_test"), "{\"personalizations\":[{\"to\":[{\"email\":\"to_email@example.com\"}],\"cc\":[{\"email\":\"cc_email1@example.com\"},{\"email\":\"cc_email2@example.com\"}]}],\"from\":{\"email\":\"from_email@example.com\"},\"subject\":\"subject_test\",\"content\":[{\"type\":\"text/plain\",\"value\":\"body_test\"}]}");
    }
}
