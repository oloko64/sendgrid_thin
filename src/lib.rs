use serde::Serialize;
use std::error::Error;

/// The content type of the email.
/// # Example
/// ```
/// // ContentType::Html; Equals to "text/html"
/// // ContentType::Text; Equals to "text/plain"
/// ```
pub enum ContentType {
    Text,
    Html,
}

impl AsRef<ContentType> for ContentType {
    fn as_ref(&self) -> &ContentType {
        self
    }
}

pub struct Sendgrid<'a> {
    api_key: String,
    sendgrid_email: SendgridEmail<'a>,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
struct SendgridEmail<'a> {
    #[serde(rename = "personalizations")]
    personalizations: [Personalization; 1],

    #[serde(rename = "from")]
    from: From,

    #[serde(rename = "subject")]
    subject: Option<String>,

    #[serde(rename = "content")]
    content: [Content<'a>; 1],

    #[serde(rename = "send_at", skip_serializing_if = "Option::is_none")]
    send_at: Option<u64>,
}

trait SendgridEmailFirstItem<'a> {
    fn get_first_personalization(&mut self) -> &mut Personalization;
    fn get_first_content(&mut self) -> &mut Content<'a>;
}

impl<'a> SendgridEmailFirstItem<'a> for SendgridEmail<'a> {
    fn get_first_personalization(&mut self) -> &mut Personalization {
        &mut *self
            .personalizations
            .first_mut()
            .expect("Failed to get personalizations at index 0")
    }

    fn get_first_content(&mut self) -> &mut Content<'a> {
        &mut *self
            .content
            .first_mut()
            .expect("Failed to get content at index 0")
    }
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct Content<'a> {
    #[serde(rename = "type")]
    content_type: Option<&'a str>,

    #[serde(rename = "value")]
    value: Option<String>,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct From {
    #[serde(rename = "email")]
    email: String,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct Personalization {
    #[serde(rename = "to")]
    to: Vec<From>,

    #[serde(rename = "cc", skip_serializing_if = "Option::is_none")]
    cc: Option<Vec<From>>,
}

impl<'a> Sendgrid<'a> {
    #[must_use]
    /// Create a new sendgrid instance.
    pub fn new<T: AsRef<str>, U: AsRef<str>, V: AsRef<str>>(
        api_key: T,
        to_email: U,
        from_email: V,
    ) -> Self {
        Self {
            api_key: api_key.as_ref().to_owned(),
            sendgrid_email: SendgridEmail {
                personalizations: [Personalization {
                    to: vec![From {
                        email: to_email.as_ref().to_owned(),
                    }],
                    cc: None,
                }],
                from: From {
                    email: from_email.as_ref().to_owned(),
                },
                subject: None,
                content: [Content {
                    content_type: None,
                    value: None,
                }],
                send_at: None,
            },
        }
    }

    /// Add a CC email to the email.
    ///
    /// Allow to send the email to multiple recipients.
    /// # Example
    /// ```
    /// use sendgrid_thin::Sendgrid;
    /// let mut sendgrid = Sendgrid::new("SENDGRID_API_KEY", "to_email@example.com", "from_email@example.com");
    /// sendgrid.add_cc_emails(&["cc_email1@example.com", "cc_email2@example.com"]);
    /// match sendgrid.send("subject", "body") {
    ///    Ok(_) => println!("Email sent successfully"),
    ///    Err(err) => println!("Error sending email: {}", err),
    /// }
    /// ```
    pub fn add_cc_emails<T: AsRef<str>>(&mut self, cc_emails: &[T]) {
        match self.sendgrid_email.get_first_personalization().cc.as_mut() {
            Some(cc) => {
                for email in cc_emails {
                    cc.push(From {
                        email: (*email).as_ref().to_owned(),
                    });
                }
            }
            None => {
                self.sendgrid_email.get_first_personalization().cc = Some(
                    cc_emails
                        .iter()
                        .map(|email| From {
                            email: (*email).as_ref().to_owned(),
                        })
                        .collect(),
                );
            }
        }
    }

    /// Set the content type of the email.
    /// # Example
    /// ```
    /// use sendgrid_thin::{Sendgrid, ContentType};
    /// let mut sendgrid = Sendgrid::new("SENDGRID_API_KEY", "to_email@example.com", "from_email@example.com");
    /// sendgrid.set_content_type(ContentType::Html);
    /// ```
    pub fn set_content_type<T: AsRef<ContentType>>(&mut self, content_type: T) {
        self.sendgrid_email.get_first_content().content_type = match content_type.as_ref() {
            ContentType::Text => Some("text/plain"),
            ContentType::Html => Some("text/html"),
        }
    }

    fn set_body_email(&mut self, subject: &str, body: &str) -> String {
        if self
            .sendgrid_email
            .get_first_content()
            .content_type
            .is_none()
        {
            self.sendgrid_email.get_first_content().content_type = Some("text/plain");
        }
        self.sendgrid_email.subject = Some(subject.to_owned());
        self.sendgrid_email.get_first_content().value = Some(body.to_owned());
        serde_json::to_string(&self.sendgrid_email).expect("Error serializing email")
    }

    /// Set the time to send the email.
    /// # Example
    /// ```
    /// use sendgrid_thin::Sendgrid;
    /// let mut sendgrid = Sendgrid::new("SENDGRID_API_KEY", "to_email@example.com", "from_email@example.com");
    /// sendgrid.set_send_at(1668271500);
    /// ```
    pub fn set_send_at(&mut self, send_at: u64) {
        self.sendgrid_email.send_at = Some(send_at);
    }

    /// Sends an email using Sendgrid API.
    /// # Example
    /// ```
    /// use sendgrid_thin::Sendgrid;
    /// let mut sendgrid = Sendgrid::new("SENDGRID_API_KEY", "to_email@example.com", "from_email@example.com");
    /// match sendgrid.send("subject", "body") {
    ///    Ok(_) => println!("Email sent successfully"),
    ///    Err(err) => println!("Error sending email: {}", err),
    /// }
    /// ```
    pub fn send<T: AsRef<str>, U: AsRef<str>>(
        &mut self,
        subject: T,
        body: U,
    ) -> Result<String, Box<dyn Error>> {
        ureq::post("https://api.sendgrid.com/v3/mail/send")
            .set("Authorization", &format!("Bearer {}", self.api_key))
            .set("Content-Type", "application/json")
            .send_string(&self.set_body_email(subject.as_ref(), body.as_ref()))?;

        if let Some(send_at) = self.sendgrid_email.send_at {
            Ok(format!("Email successfully scheduled to be sent at {}.", send_at))
        } else {
            Ok("Email sent successfully.".to_owned())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sendgrid_instance() {
        let sendgrid = Sendgrid::new(
            "SENDGRID_API_KEY",
            "to_email@example.com",
            "from_email@example.com",
        );
        assert_eq!(
            sendgrid.sendgrid_email,
            SendgridEmail {
                personalizations: [Personalization {
                    to: vec![From {
                        email: "to_email@example.com".to_owned()
                    }],
                    cc: None,
                }],
                from: From {
                    email: "from_email@example.com".to_owned()
                },
                subject: None,
                content: [Content {
                    content_type: None,
                    value: None,
                }],
                send_at: None,
            }
        );
    }

    #[test]
    fn test_content_type() {
        let mut sendgrid = Sendgrid::new(
            "SENDGRID_API_KEY",
            "to_email@example.com",
            "from_email@example.com",
        );
        assert_eq!(
            sendgrid.sendgrid_email.get_first_content().content_type,
            None
        );
        sendgrid.set_content_type(ContentType::Html);
        assert_eq!(
            sendgrid.sendgrid_email.get_first_content().content_type,
            Some("text/html")
        );
        sendgrid.set_content_type(ContentType::Text);
        assert_eq!(
            sendgrid.sendgrid_email.get_first_content().content_type,
            Some("text/plain")
        );
    }

    #[test]
    fn test_set_body_email() {
        let mut sendgrid = Sendgrid::new(
            "SENDGRID_API_KEY",
            "to_email@example.com",
            "from_email@example.com",
        );
        assert_eq!(sendgrid.set_body_email("subject_test", "body_test"), "{\"personalizations\":[{\"to\":[{\"email\":\"to_email@example.com\"}]}],\"from\":{\"email\":\"from_email@example.com\"},\"subject\":\"subject_test\",\"content\":[{\"type\":\"text/plain\",\"value\":\"body_test\"}]}");
    }

    #[test]
    fn test_set_body_email_with_cc_emails() {
        let mut sendgrid = Sendgrid::new(
            "SENDGRID_API_KEY",
            "to_email@example.com",
            "from_email@example.com",
        );
        sendgrid.add_cc_emails(&["cc_email1@example.com", "cc_email2@example.com"]);
        assert_eq!(sendgrid.set_body_email("subject_test", "body_test"), "{\"personalizations\":[{\"to\":[{\"email\":\"to_email@example.com\"}],\"cc\":[{\"email\":\"cc_email1@example.com\"},{\"email\":\"cc_email2@example.com\"}]}],\"from\":{\"email\":\"from_email@example.com\"},\"subject\":\"subject_test\",\"content\":[{\"type\":\"text/plain\",\"value\":\"body_test\"}]}");
    }

    #[test]
    fn test_set_send_at() {
        let mut sendgrid = Sendgrid::new(
            "SENDGRID_API_KEY",
            "to_email@example.com",
            "from_email@example.com",
        );
        assert_eq!(sendgrid.sendgrid_email.send_at, None);
        sendgrid.set_send_at(1668271500);
        assert_eq!(sendgrid.sendgrid_email.send_at, Some(1668271500));
    }
}
