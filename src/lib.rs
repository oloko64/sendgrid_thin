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

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct SendgridEmail<'a> {
    #[serde(skip)]
    api_key: String,

    #[serde(rename = "personalizations")]
    personalizations: [Personalization; 1],

    #[serde(rename = "from")]
    from: From,

    #[serde(rename = "subject")]
    subject: Option<String>,

    #[serde(rename = "content")]
    content: [Content<'a>; 1],
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

impl<'a> SendgridEmail<'a> {
    #[must_use]
    /// Create a new sendgrid instance.
    pub fn new<T: AsRef<str>, U: AsRef<str>, V: AsRef<str>>(
        api_key: T,
        to_email: U,
        from_email: V,
    ) -> Self {
        Self {
            api_key: api_key.as_ref().to_owned(),
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
        }
    }

    /// Sends an email using Sendgrid API.
    /// # Example
    /// ```
    /// use sendgrid_thin::SendgridEmail;
    /// let mut sendgrid = SendgridEmail::new("SENDGRID_API_KEY", "to_email@example.com", "from_email@example.com");
    /// match sendgrid.send("subject", "body") {
    ///    Ok(_) => println!("Email sent successfully"),
    ///    Err(err) => println!("Error sending email: {}", err),
    /// }
    /// ```
    pub fn send<T: AsRef<str>, U: AsRef<str>>(
        &mut self,
        subject: T,
        body: U,
    ) -> Result<(), Box<dyn Error>> {
        ureq::post("https://api.sendgrid.com/v3/mail/send")
            .set("Authorization", &format!("Bearer {}", self.api_key))
            .set("Content-Type", "application/json")
            .send_string(&self.set_body_email(subject.as_ref(), body.as_ref()))?;
        Ok(())
    }

    /// Add a CC email to the email.
    ///
    /// Allow to send the email to multiple recipients.
    /// # Example
    /// ```
    /// use sendgrid_thin::SendgridEmail;
    /// let mut sendgrid = SendgridEmail::new("SENDGRID_API_KEY", "to_email@example.com", "from_email@example.com");
    /// sendgrid.add_cc_emails(&["cc_email1@example.com", "cc_email2@example.com"]);
    /// match sendgrid.send("subject", "body") {
    ///    Ok(_) => println!("Email sent successfully"),
    ///    Err(err) => println!("Error sending email: {}", err),
    /// }
    /// ```
    pub fn add_cc_emails<T: AsRef<str>>(&mut self, cc_emails: &[T]) {
        match self.personalizations[0].cc.as_mut() {
            Some(cc) => {
                for email in cc_emails {
                    cc.push(From {
                        email: (*email).as_ref().to_owned(),
                    });
                }
            }
            None => {
                self.personalizations[0].cc = Some(
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
    /// use sendgrid_thin::{SendgridEmail, ContentType};
    /// let mut sendgrid = SendgridEmail::new("SENDGRID_API_KEY", "to_email@example.com", "from_email@example.com");
    /// sendgrid.set_content_type(ContentType::Html);
    /// ```
    pub fn set_content_type<T: AsRef<ContentType>>(&mut self, content_type: T) {
        self.content[0].content_type = match content_type.as_ref() {
            ContentType::Text => Some("text/plain"),
            ContentType::Html => Some("text/html"),
        }
    }

    fn set_body_email(&mut self, subject: &str, body: &str) -> String {
        if self.content[0].content_type.is_none() {
            self.content[0].content_type = Some("text/plain");
        }
        self.subject = Some(subject.to_owned());
        self.content[0].value = Some(body.to_owned());
        serde_json::to_string(self).expect("Error serializing email")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sendgrid_instance() {
        let sendgrid = SendgridEmail::new(
            "SENDGRID_API_KEY",
            "to_email@example.com",
            "from_email@example.com",
        );
        assert_eq!(
            sendgrid,
            SendgridEmail {
                api_key: "SENDGRID_API_KEY".to_owned(),
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
            }
        );
    }

    #[test]
    fn test_content_type() {
        let mut sendgrid = SendgridEmail::new(
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
        let mut sendgrid = SendgridEmail::new(
            "SENDGRID_API_KEY",
            "to_email@example.com",
            "from_email@example.com",
        );
        assert_eq!(sendgrid.set_body_email("subject_test", "body_test"), "{\"personalizations\":[{\"to\":[{\"email\":\"to_email@example.com\"}]}],\"from\":{\"email\":\"from_email@example.com\"},\"subject\":\"subject_test\",\"content\":[{\"type\":\"text/plain\",\"value\":\"body_test\"}]}");
    }

    #[test]
    fn test_set_body_email_with_cc_emails() {
        let mut sendgrid = SendgridEmail::new(
            "SENDGRID_API_KEY",
            "to_email@example.com",
            "from_email@example.com",
        );
        sendgrid.add_cc_emails(&["cc_email1@example.com", "cc_email2@example.com"]);
        assert_eq!(sendgrid.set_body_email("subject_test", "body_test"), "{\"personalizations\":[{\"to\":[{\"email\":\"to_email@example.com\"}],\"cc\":[{\"email\":\"cc_email1@example.com\"},{\"email\":\"cc_email2@example.com\"}]}],\"from\":{\"email\":\"from_email@example.com\"},\"subject\":\"subject_test\",\"content\":[{\"type\":\"text/plain\",\"value\":\"body_test\"}]}");
    }
}
