use serde::Serialize;
use std::{
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
};

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
    email: Option<String>,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct Personalization {
    #[serde(rename = "to")]
    to: Vec<From>,

    #[serde(rename = "cc", skip_serializing_if = "Option::is_none")]
    cc: Option<Vec<From>>,
}

impl<'a> Sendgrid<'a> {
    #[must_use = "Sendgrid::new() returns a Sendgrid instance"]
    /// Create a new sendgrid instance.
    /// # Example
    /// ```
    /// use sendgrid_thin::{Sendgrid, ContentType};
    /// let mut sendgrid = Sendgrid::new("SENDGRID_API_KEY");
    /// // Required
    /// sendgrid
    ///    .set_to_emails(&["to_email_1@example.com", "to_email_2@example.com"])
    ///    .set_from_email("from_email@example.com")
    ///    .set_subject("subject of email")
    ///    .set_body("body of email");
    ///
    /// // Optional
    ///sendgrid
    ///    .set_content_type(ContentType::Text)
    ///    .set_send_at(1668271500)
    ///    .set_cc_emails(&["cc_email_1@example.com", "cc_email_2@example.com"]);
    ///
    /// match sendgrid.send() {
    ///    Ok(message) => println!("{}", message),
    ///    Err(err) => println!("Error sending email: {}", err),
    /// }
    /// ```
    pub fn new<T: AsRef<str>>(api_key: T) -> Self {
        Self {
            api_key: api_key.as_ref().to_owned(),
            sendgrid_email: SendgridEmail {
                personalizations: [Personalization {
                    to: vec![From { email: None }],
                    cc: None,
                }],
                from: From { email: None },
                subject: None,
                content: [Content {
                    content_type: None,
                    value: None,
                }],
                send_at: None,
            },
        }
    }

    /// Sets the recipients that will receive the email.
    ///
    /// # Example
    /// ```
    /// use sendgrid_thin::Sendgrid;
    /// let mut sendgrid = Sendgrid::new("SENDGRID_API_KEY");
    /// // Required
    /// sendgrid
    ///    .set_to_emails(&["to_email_1@example.com", "to_email_2@example.com"])
    ///    .set_from_email("from_email@example.com")
    ///    .set_subject("subject of email")
    ///    .set_body("body of email");
    ///
    /// match sendgrid.send() {
    ///    Ok(message) => println!("{}", message),
    ///    Err(err) => println!("Error sending email: {}", err),
    /// }
    /// ```
    pub fn set_to_emails<T: AsRef<str>>(&mut self, to_email: &[T]) -> &mut Self {
        self.sendgrid_email.get_first_personalization().to = to_email
            .iter()
            .map(|email| From {
                email: Some(email.as_ref().to_owned()),
            })
            .collect();
        self
    }

    /// Sets the sender of the email.
    ///
    /// # Example
    /// ```
    /// use sendgrid_thin::Sendgrid;
    /// let mut sendgrid = Sendgrid::new("SENDGRID_API_KEY");
    /// // Required
    /// sendgrid
    ///    .set_to_emails(&["to_email_1@example.com", "to_email_2@example.com"])
    ///    .set_from_email("from_email@example.com")
    ///    .set_subject("subject of email")
    ///    .set_body("body of email");
    ///
    /// match sendgrid.send() {
    ///    Ok(message) => println!("{}", message),
    ///    Err(err) => println!("Error sending email: {}", err),
    /// }
    /// ```
    pub fn set_from_email<T: AsRef<str>>(&mut self, from_email: T) -> &mut Self {
        self.sendgrid_email.from.email = Some(from_email.as_ref().to_owned());
        self
    }

    /// Sets the subject of the email.
    ///
    /// # Example
    /// ```
    /// use sendgrid_thin::Sendgrid;
    /// let mut sendgrid = Sendgrid::new("SENDGRID_API_KEY");
    /// // Required
    /// sendgrid
    ///    .set_to_emails(&["to_email_1@example.com", "to_email_2@example.com"])
    ///    .set_from_email("from_email@example.com")
    ///    .set_subject("subject of email")
    ///    .set_body("body of email");
    ///
    /// match sendgrid.send() {
    ///    Ok(message) => println!("{}", message),
    ///    Err(err) => println!("Error sending email: {}", err),
    /// }
    /// ```
    pub fn set_subject<T: AsRef<str>>(&mut self, subject: T) -> &mut Self {
        self.sendgrid_email.subject = Some(subject.as_ref().to_owned());
        self
    }

    /// Sets the body of the email.
    ///
    /// # Example
    /// ```
    /// use sendgrid_thin::Sendgrid;
    /// let mut sendgrid = Sendgrid::new("SENDGRID_API_KEY");
    /// // Required
    /// sendgrid
    ///    .set_to_emails(&["to_email_1@example.com", "to_email_2@example.com"])
    ///    .set_from_email("from_email@example.com")
    ///    .set_subject("subject of email")
    ///    .set_body("body of email");
    ///
    /// match sendgrid.send() {
    ///    Ok(message) => println!("{}", message),
    ///    Err(err) => println!("Error sending email: {}", err),
    /// }
    /// ```
    pub fn set_body<T: AsRef<str>>(&mut self, body: T) -> &mut Self {
        self.sendgrid_email.get_first_content().value = Some(body.as_ref().to_owned());
        self
    }

    /// Add a CC email to the email.
    ///
    /// Allow to send the email to multiple recipients.
    /// # Example
    /// ```
    /// use sendgrid_thin::Sendgrid;
    /// let mut sendgrid = Sendgrid::new("SENDGRID_API_KEY");
    /// // Required
    /// sendgrid
    ///    .set_to_emails(&["to_email_1@example.com", "to_email_2@example.com"])
    ///    .set_from_email("from_email@example.com")
    ///    .set_subject("subject of email")
    ///    .set_body("body of email");
    ///
    /// sendgrid.set_cc_emails(&["cc_email1@example.com", "cc_email2@example.com"]);
    ///
    /// match sendgrid.send() {
    ///    Ok(message) => println!("{}", message),
    ///    Err(err) => println!("Error sending email: {}", err),
    /// }
    /// ```
    pub fn set_cc_emails<T: AsRef<str>>(&mut self, cc_emails: &[T]) -> &mut Self {
        self.sendgrid_email.get_first_personalization().cc = Some(
            cc_emails
                .iter()
                .map(|email| From {
                    email: Some((*email).as_ref().to_owned()),
                })
                .collect(),
        );
        self
    }

    /// Set the content type of the email.
    /// # Example
    /// ```
    /// use sendgrid_thin::{Sendgrid, ContentType};
    /// let mut sendgrid = Sendgrid::new("SENDGRID_API_KEY");
    /// // Required
    /// sendgrid
    ///    .set_to_emails(&["to_email_1@example.com", "to_email_2@example.com"])
    ///    .set_from_email("from_email@example.com")
    ///    .set_subject("subject of email")
    ///    .set_body("body of email");
    ///
    /// sendgrid.set_content_type(ContentType::Html);
    ///
    /// match sendgrid.send() {
    ///    Ok(message) => println!("{}", message),
    ///    Err(err) => println!("Error sending email: {}", err),
    /// }
    /// ```
    pub fn set_content_type<T: AsRef<ContentType>>(&mut self, content_type: T) -> &mut Self {
        self.sendgrid_email.get_first_content().content_type = match content_type.as_ref() {
            ContentType::Text => Some("text/plain"),
            ContentType::Html => Some("text/html"),
        };
        self
    }

    /// Set the time to send the email.
    /// # Example
    /// ```
    /// use sendgrid_thin::Sendgrid;
    /// let mut sendgrid = Sendgrid::new("SENDGRID_API_KEY");
    /// // Required
    /// sendgrid
    ///    .set_to_emails(&["to_email_1@example.com", "to_email_2@example.com"])
    ///    .set_from_email("from_email@example.com")
    ///    .set_subject("subject of email")
    ///    .set_body("body of email");
    ///
    /// sendgrid.set_send_at(1668271500);
    ///
    /// match sendgrid.send() {
    ///    Ok(message) => println!("{}", message),
    ///    Err(err) => println!("Error sending email: {}", err),
    /// }
    /// ```
    pub fn set_send_at(&mut self, send_at: u64) -> &mut Self {
        self.sendgrid_email.send_at = Some(send_at);
        self
    }

    fn check_required_parameters(&mut self) -> Result<(), Box<dyn Error>> {
        if self.sendgrid_email.get_first_content().value.is_none() {
            return Err(
                "Email body is required. Use set_body() to set the body of the email.".into(),
            );
        }
        if self.sendgrid_email.subject.is_none() {
            return Err(
                "Email subject is required. Use set_subject() to set the subject of the email."
                    .into(),
            );
        };
        Ok(())
    }

    /// Sends an email using Sendgrid API.
    /// # Example
    /// ```
    /// use sendgrid_thin::{Sendgrid, ContentType};
    /// let mut sendgrid = Sendgrid::new("SENDGRID_API_KEY");
    /// // Required
    /// sendgrid
    ///    .set_to_emails(&["to_email_1@example.com", "to_email_2@example.com"])
    ///    .set_from_email("from_email@example.com")
    ///    .set_subject("subject of email")
    ///    .set_body("body of email");
    ///
    /// // Optional
    ///sendgrid
    ///    .set_content_type(ContentType::Text)
    ///    .set_send_at(1668271500)
    ///    .set_cc_emails(&["cc_email_1@example.com", "cc_email_2@example.com"]);
    ///
    /// match sendgrid.send() {
    ///    Ok(message) => println!("{}", message),
    ///    Err(err) => println!("Error sending email: {}", err),
    /// }
    /// ```
    pub fn send(&mut self) -> Result<String, Box<dyn Error>> {
        if self
            .sendgrid_email
            .get_first_content()
            .content_type
            .is_none()
        {
            self.sendgrid_email.get_first_content().content_type = Some("text/plain");
        }

        self.check_required_parameters()?;

        ureq::post("https://api.sendgrid.com/v3/mail/send")
            .set("Authorization", &format!("Bearer {}", self.api_key))
            .set("Content-Type", "application/json")
            .send_string(
                &serde_json::to_string(&self.sendgrid_email).expect("Error serializing email"),
            )?;

        if let Some(send_at) = self.sendgrid_email.send_at {
            if SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Error getting current time")
                .as_secs()
                < send_at
            {
                return Ok(format!(
                    "Email successfully scheduled to be sent at {}.",
                    send_at
                ));
            }
        }
        Ok("Email sent successfully.".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sendgrid_instance() {
        let sendgrid = Sendgrid::new("SENDGRID_API_KEY");
        assert_eq!(
            sendgrid.sendgrid_email,
            SendgridEmail {
                personalizations: [Personalization {
                    to: vec![From { email: None }],
                    cc: None,
                }],
                from: From { email: None },
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
        let mut sendgrid = Sendgrid::new("SENDGRID_API_KEY");
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
    fn test_set_body_and_subject_email() {
        let mut sendgrid = Sendgrid::new("SENDGRID_API_KEY");
        sendgrid
            .set_to_emails(&["to_email@example.com"])
            .set_from_email("from_email@example.com");
        sendgrid
            .set_body("body_test")
            .set_subject("subject_test")
            .set_content_type(ContentType::Text);
        assert_eq!(serde_json::to_string(&sendgrid.sendgrid_email).unwrap(), "{\"personalizations\":[{\"to\":[{\"email\":\"to_email@example.com\"}]}],\"from\":{\"email\":\"from_email@example.com\"},\"subject\":\"subject_test\",\"content\":[{\"type\":\"text/plain\",\"value\":\"body_test\"}]}");
    }

    #[test]
    fn test_set_body_email_with_cc_emails() {
        let mut sendgrid = Sendgrid::new("SENDGRID_API_KEY");
        sendgrid
            .set_to_emails(&["to_email@example.com"])
            .set_from_email("from_email@example.com");
        sendgrid
            .set_body("body_test")
            .set_subject("subject_test")
            .set_content_type(ContentType::Text);
        sendgrid.set_cc_emails(&["cc_email1@example.com", "cc_email2@example.com"]);
        assert_eq!(serde_json::to_string(&sendgrid.sendgrid_email).unwrap(), "{\"personalizations\":[{\"to\":[{\"email\":\"to_email@example.com\"}],\"cc\":[{\"email\":\"cc_email1@example.com\"},{\"email\":\"cc_email2@example.com\"}]}],\"from\":{\"email\":\"from_email@example.com\"},\"subject\":\"subject_test\",\"content\":[{\"type\":\"text/plain\",\"value\":\"body_test\"}]}");
    }

    #[test]
    fn test_set_send_at() {
        let mut sendgrid = Sendgrid::new("SENDGRID_API_KEY");
        assert_eq!(sendgrid.sendgrid_email.send_at, None);
        sendgrid.set_send_at(1668271500);
        assert_eq!(sendgrid.sendgrid_email.send_at, Some(1668271500));
    }
}
