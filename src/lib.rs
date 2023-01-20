use anyhow::{bail, Result};
use serde::Serialize;
use std::{
    mem::take,
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

#[must_use]
pub struct SendgridRequest(Sendgrid);

impl SendgridRequest {
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
    ///let request = sendgrid
    ///    .set_content_type(ContentType::Text)
    ///    .set_send_at(1668271500)
    ///    .set_cc_emails(&["cc_email_1@example.com", "cc_email_2@example.com"]).build().unwrap();
    ///
    /// match request.send() {
    ///    Ok(message) => println!("{}", message),
    ///    Err(err) => println!("Error sending email: {}", err),
    /// }
    /// ```
    pub fn send(&self) -> Result<String> {
        ureq::post("https://api.sendgrid.com/v3/mail/send")
            .set("Authorization", &format!("Bearer {}", self.0.api_key))
            .set("Content-Type", "application/json")
            .send_string(
                &serde_json::to_string(&self.0.sendgrid_email).expect("Error serializing email"),
            )?;

        if let Some(send_at) = self.0.sendgrid_email.send_at {
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Error getting current time")
                .as_secs();
            if current_time < send_at {
                return Ok(format!(
                    "Email successfully scheduled to be sent at {}.",
                    send_at
                ));
            }
        }
        Ok("Email sent successfully.".to_owned())
    }
}

#[derive(Default)]
pub struct Sendgrid {
    api_key: String,
    sendgrid_email: SendgridEmail,
}

#[derive(Serialize)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
struct SendgridEmail {
    #[serde(rename = "personalizations")]
    personalizations: [Personalization; 1],

    #[serde(rename = "from")]
    from: From,

    #[serde(rename = "subject")]
    subject: Option<String>,

    #[serde(rename = "content")]
    content: [Content; 1],

    #[serde(rename = "send_at", skip_serializing_if = "Option::is_none")]
    send_at: Option<u64>,
}

impl Default for SendgridEmail {
    fn default() -> Self {
        SendgridEmail {
            personalizations: [Personalization {
                to: Vec::from([From { email: None }]),
                cc: None,
            }],
            from: From { email: None },
            subject: None,
            content: [Content {
                content_type: Some(String::from("text/plain")),
                value: None,
            }],
            send_at: None,
        }
    }
}

trait SendgridEmailFirstItem {
    fn get_first_personalization(&mut self) -> &mut Personalization;
    fn get_first_content(&mut self) -> &mut Content;
}

impl SendgridEmailFirstItem for SendgridEmail {
    fn get_first_personalization(&mut self) -> &mut Personalization {
        &mut *self
            .personalizations
            .first_mut()
            .expect("Failed to get personalizations at index 0")
    }

    fn get_first_content(&mut self) -> &mut Content {
        &mut *self
            .content
            .first_mut()
            .expect("Failed to get content at index 0")
    }
}

#[derive(Serialize)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
struct Content {
    #[serde(rename = "type")]
    content_type: Option<String>,

    #[serde(rename = "value")]
    value: Option<String>,
}

#[derive(Serialize)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
struct From {
    #[serde(rename = "email")]
    email: Option<String>,
}

#[derive(Serialize)]
#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
struct Personalization {
    #[serde(rename = "to")]
    to: Vec<From>,

    #[serde(rename = "cc", skip_serializing_if = "Option::is_none")]
    cc: Option<Vec<From>>,
}

impl Sendgrid {
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
    ///let request = sendgrid
    ///    .set_content_type(ContentType::Text)
    ///    .set_send_at(1668271500)
    ///    .set_cc_emails(&["cc_email_1@example.com", "cc_email_2@example.com"]).build().unwrap();
    ///
    /// match request.send() {
    ///    Ok(message) => println!("{}", message),
    ///    Err(err) => println!("Error sending email: {}", err),
    /// }
    /// ```
    #[must_use = "Sendgrid::new() returns a Sendgrid instance"]
    pub fn new(api_key: impl Into<String>) -> Sendgrid {
        Self {
            api_key: api_key.into(),
            ..Default::default()
        }
    }

    /// Sets the recipients that will receive the email.
    ///
    /// # Example
    /// ```
    /// use sendgrid_thin::Sendgrid;
    /// let mut sendgrid = Sendgrid::new("SENDGRID_API_KEY");
    /// // Required
    /// let request = sendgrid
    ///    .set_to_emails(&["to_email_1@example.com", "to_email_2@example.com"])
    ///    .set_from_email("from_email@example.com")
    ///    .set_subject("subject of email")
    ///    .set_body("body of email").build().unwrap();
    ///
    /// match request.send() {
    ///    Ok(message) => println!("{}", message),
    ///    Err(err) => println!("Error sending email: {}", err),
    /// }
    /// ```
    pub fn set_to_emails<T>(&mut self, to_email: impl IntoIterator<Item = T>) -> &mut Sendgrid
    where
        T: AsRef<str>,
    {
        self.sendgrid_email.get_first_personalization().to = to_email
            .into_iter()
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
    /// let request = sendgrid
    ///    .set_to_emails(&["to_email_1@example.com", "to_email_2@example.com"])
    ///    .set_from_email("from_email@example.com")
    ///    .set_subject("subject of email")
    ///    .set_body("body of email").build().unwrap();
    ///
    /// match request.send() {
    ///    Ok(message) => println!("{}", message),
    ///    Err(err) => println!("Error sending email: {}", err),
    /// }
    /// ```
    pub fn set_from_email(&mut self, from_email: impl Into<String>) -> &mut Sendgrid {
        self.sendgrid_email.from.email = Some(from_email.into());
        self
    }

    /// Sets the subject of the email.
    ///
    /// # Example
    /// ```
    /// use sendgrid_thin::Sendgrid;
    /// let mut sendgrid = Sendgrid::new("SENDGRID_API_KEY");
    /// // Required
    /// let request = sendgrid
    ///    .set_to_emails(&["to_email_1@example.com", "to_email_2@example.com"])
    ///    .set_from_email("from_email@example.com")
    ///    .set_subject("subject of email")
    ///    .set_body("body of email").build().unwrap();
    ///
    /// match request.send() {
    ///    Ok(message) => println!("{}", message),
    ///    Err(err) => println!("Error sending email: {}", err),
    /// }
    /// ```
    pub fn set_subject(&mut self, subject: impl Into<String>) -> &mut Sendgrid {
        self.sendgrid_email.subject = Some(subject.into());
        self
    }

    /// Sets the body of the email.
    ///
    /// # Example
    /// ```
    /// use sendgrid_thin::Sendgrid;
    /// let mut sendgrid = Sendgrid::new("SENDGRID_API_KEY");
    /// // Required
    /// let request = sendgrid
    ///    .set_to_emails(&["to_email_1@example.com", "to_email_2@example.com"])
    ///    .set_from_email("from_email@example.com")
    ///    .set_subject("subject of email")
    ///    .set_body("body of email").build().unwrap();
    ///
    /// match request.send() {
    ///    Ok(message) => println!("{}", message),
    ///    Err(err) => println!("Error sending email: {}", err),
    /// }
    /// ```
    pub fn set_body(&mut self, body: impl Into<String>) -> &mut Sendgrid {
        self.sendgrid_email.get_first_content().value = Some(body.into());
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
    /// let request = sendgrid.set_cc_emails(&["cc_email1@example.com", "cc_email2@example.com"]).build().unwrap();
    ///
    /// match request.send() {
    ///    Ok(message) => println!("{}", message),
    ///    Err(err) => println!("Error sending email: {}", err),
    /// }
    /// ```
    pub fn set_cc_emails<T>(&mut self, cc_emails: impl IntoIterator<Item = T>) -> &mut Sendgrid
    where
        T: AsRef<str>,
    {
        self.sendgrid_email.get_first_personalization().cc = Some(
            cc_emails
                .into_iter()
                .map(|email| From {
                    email: Some(email.as_ref().to_owned()),
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
    /// let request = sendgrid.set_content_type(ContentType::Html).build().unwrap();
    ///
    /// match request.send() {
    ///    Ok(message) => println!("{}", message),
    ///    Err(err) => println!("Error sending email: {}", err),
    /// }
    /// ```
    pub fn set_content_type<T>(&mut self, content_type: T) -> &mut Sendgrid
    where
        T: AsRef<ContentType>,
    {
        self.sendgrid_email.get_first_content().content_type = match content_type.as_ref() {
            ContentType::Text => Some(String::from("text/plain")),
            ContentType::Html => Some(String::from("text/html")),
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
    /// let request = sendgrid.set_send_at(1668271500).build().unwrap();
    ///
    /// match request.send() {
    ///    Ok(message) => println!("{}", message),
    ///    Err(err) => println!("Error sending email: {}", err),
    /// }
    /// ```
    pub fn set_send_at(&mut self, send_at: u64) -> &mut Sendgrid {
        self.sendgrid_email.send_at = Some(send_at);
        self
    }

    fn check_required_parameters(&self) -> Result<()> {
        if self.sendgrid_email.content[0].value.is_none() {
            bail!("Email body is required. Use set_body() to set the body of the email.");
        }
        if self.sendgrid_email.subject.is_none() {
            bail!("Email subject is required. Use set_subject() to set the subject of the email.");
        };
        if self.sendgrid_email.personalizations[0].to[0]
            .email
            .is_none()
        {
            bail!("Email to is required. Use set_to_emails() to set the to of the email.");
        };
        if self.sendgrid_email.from.email.is_none() {
            bail!("Email from is required. Use set_from_email() to set the from of the email.");
        };
        Ok(())
    }

    /// Build the request to send the email.
    /// # Example
    /// ```
    /// use sendgrid_thin::Sendgrid;
    /// let mut sendgrid = Sendgrid::new("SENDGRID_API_KEY");
    /// 
    /// // Required
    /// sendgrid
    ///    .set_to_emails(&["to_email_1@example.com", "to_email_2@example.com"])
    ///    .set_from_email("from_email@example.com")
    ///    .set_subject("subject of email")
    ///    .set_body("body of email");
    /// 
    /// let request = sendgrid.build().unwrap();
    /// 
    /// match request.send() {
    ///   Ok(message) => println!("{}", message),
    ///  Err(err) => println!("Error sending email: {}", err),
    /// }
    /// ```
    pub fn build(&mut self) -> Result<SendgridRequest> {
        self.check_required_parameters()?;
        let request = take(self);
        Ok(SendgridRequest(request))
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
                    to: Vec::from([From { email: None }]),
                    cc: None,
                }],
                from: From { email: None },
                subject: None,
                content: [Content {
                    content_type: Some(String::from("text/plain")),
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
            Some(String::from("text/plain"))
        );
        sendgrid.set_content_type(ContentType::Html);
        assert_eq!(
            sendgrid.sendgrid_email.get_first_content().content_type,
            Some(String::from("text/html"))
        );
        sendgrid.set_content_type(ContentType::Text);
        assert_eq!(
            sendgrid.sendgrid_email.get_first_content().content_type,
            Some(String::from("text/plain"))
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
