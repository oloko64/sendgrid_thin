use std::error::Error;

/// The content type of the email.
/// # Example
/// ```
/// sendgrid_thin::ContentType::Html; // Equals to "text/html"
/// sendgrid_thin::ContentType::Text; // Equals to "text/plain"
/// ```
pub enum ContentType {
    Text,
    Html,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Sendgrid<'a> {
    to_email: &'a str,
    from_email: &'a str,
    api_key: &'a str,
    content_type: Option<&'a str>,
}

impl<'a> Sendgrid<'a> {
    /// Create a new sendgrid instance.
    pub fn new(api_key: &'a str, to_email: &'a str, from_email: &'a str) -> Self {
        Self {
            api_key,
            to_email,
            from_email,
            content_type: None,
        }
    }

    /// Sends an email using Sendgrid API.
    /// # Example
    /// ```
    /// use sendgrid_thin::Sendgrid;
    /// let sendgrid = Sendgrid::new("SENDGRID_API_KEY", "to_email@example.com", "from_email@example.com");
    /// match sendgrid.send_mail("subject", "body") {
    ///    Ok(_) => println!("Email sent successfully"),
    ///    Err(err) => println!("Error sending email: {}", err),
    /// }
    /// ```
    pub fn send_mail(&self, subject: &str, body: &str) -> Result<(), Box<dyn Error>> {
        ureq::post("https://api.sendgrid.com/v3/mail/send")
            .set("Authorization", &format!("Bearer {}", self.api_key))
            .set("Content-Type", "application/json")
            .send_string(&self.set_body_email(subject, body))?;
        Ok(())
    }

    /// Set the content type of the email.
    /// # Example
    /// ```
    /// use sendgrid_thin::{Sendgrid, ContentType};
    /// let mut sendgrid = Sendgrid::new("SENDGRID_API_KEY", "to_email@example.com", "from_email@example.com");
    /// sendgrid.set_content_type(ContentType::Html);
    /// ```
    pub fn set_content_type(&mut self, content_type: ContentType) {
        self.content_type = match content_type {
            ContentType::Text => Some("text/plain"),
            ContentType::Html => Some("text/html"),
        };
    }

    fn set_body_email(&self, subject: &str, body: &str) -> String {
        let content_type = self.content_type.unwrap_or("text/plain");
        format!("{{\"personalizations\": [{{\"to\": [{{\"email\": \"{}\"}}], \"subject\": \"{}\"}}], \"from\": {{\"email\": \"{}\"}}, \"content\": [{{\"type\": \"{}\", \"value\": \"{}\"}}]}}", self.to_email, subject, self.from_email, content_type , body)
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
            sendgrid,
            Sendgrid {
                api_key: "SENDGRID_API_KEY",
                to_email: "to_email@example.com",
                from_email: "from_email@example.com",
                content_type: None,
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
        assert_eq!(sendgrid.content_type, None);
        sendgrid.set_content_type(ContentType::Html);
        assert_eq!(sendgrid.content_type, Some("text/html"));
        sendgrid.set_content_type(ContentType::Text);
        assert_eq!(sendgrid.content_type, Some("text/plain"));
    }

    #[test]
    fn test_set_body_email() {
        let sendgrid = Sendgrid::new(
            "SENDGRID_API_KEY",
            "to_email@example.com",
            "from_email@example.com",
        );
        assert_eq!(sendgrid.set_body_email("subject_test", "body_test"), "{\"personalizations\": [{\"to\": [{\"email\": \"to_email@example.com\"}], \"subject\": \"subject_test\"}], \"from\": {\"email\": \"from_email@example.com\"}, \"content\": [{\"type\": \"text/plain\", \"value\": \"body_test\"}]}");
    }
}
