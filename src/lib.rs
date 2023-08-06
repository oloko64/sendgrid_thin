mod error;

use error::SendgridError;
use serde::Serialize;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub enum ContentType {
    Text,
    Html,
}

impl AsRef<ContentType> for ContentType {
    fn as_ref(&self) -> &ContentType {
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
#[must_use]
pub struct Sendgrid {
    api_key: String,
    send_at: Option<u64>,
    request_timeout: Option<Duration>,
    sendgrid_request_body: String,
}

#[derive(Debug, Clone, PartialEq)]
#[must_use]
pub struct SendgridBuilder {
    api_key: String,
    request_timeout: Option<Duration>,
    sendgrid_email: SendgridEmail,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct SendgridEmail {
    #[serde(rename = "personalizations")]
    personalizations: [Personalization; 1],

    #[serde(rename = "from")]
    from: From,

    #[serde(rename = "subject")]
    subject: String,

    #[serde(rename = "content")]
    content: [Content; 1],

    #[serde(rename = "send_at", skip_serializing_if = "Option::is_none")]
    send_at: Option<u64>,
}

impl Default for SendgridEmail {
    fn default() -> Self {
        SendgridEmail {
            personalizations: [Personalization {
                to: Vec::from([From {
                    email: String::new(),
                }]),
                cc: None,
            }],
            from: From {
                email: String::new(),
            },
            subject: String::new(),
            content: [Content {
                content_type: Some(String::from("text/plain")),
                value: String::new(),
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

#[derive(Serialize, Debug, Clone, PartialEq)]
struct Content {
    #[serde(rename = "type")]
    content_type: Option<String>,

    #[serde(rename = "value")]
    value: String,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
struct From {
    #[serde(rename = "email")]
    email: String,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
struct Personalization {
    #[serde(rename = "to")]
    to: Vec<From>,

    #[serde(rename = "cc", skip_serializing_if = "Option::is_none")]
    cc: Option<Vec<From>>,
}

impl SendgridBuilder {
    /// Create a new sendgrid builder.
    /// # Example
    /// ```
    /// use sendgrid_thin::{SendgridBuilder, ContentType};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let sendgrid = SendgridBuilder::new(
    ///         "SENDGRID_API_KEY",
    ///         "from_email@example.com",
    ///         ["to_email_1@example.com","to_email_2@example.com"],
    ///         "subject of email",
    ///         "body of email",
    ///      )
    ///     .set_content_type(ContentType::Text)
    ///     .set_send_at(1668271500)
    ///     .set_cc_emails(&["cc_email_1@example.com", "cc_email_2@example.com"])
    ///     .build()
    ///     .unwrap();
    ///
    ///     match sendgrid.send().await {
    ///         Ok(message) => println!("{message}"),
    ///         Err(err) => println!("Error sending email: {err}"),
    ///     }
    /// }
    /// ```
    pub fn new<T, U>(
        api_key: impl Into<String>,
        from_email: impl Into<String>,
        to_emails: U,
        email_subject: impl Into<String>,
        email_body: impl Into<String>,
    ) -> SendgridBuilder
    where
        T: Into<String>,
        U: IntoIterator<Item = T>,
    {
        SendgridBuilder {
            api_key: api_key.into(),
            request_timeout: None,
            sendgrid_email: {
                let mut sendgrid_email = SendgridEmail::default();
                sendgrid_email.get_first_personalization().to = to_emails
                    .into_iter()
                    .map(|email| From {
                        email: email.into(),
                    })
                    .collect();
                sendgrid_email.from.email = from_email.into();
                sendgrid_email.subject = email_subject.into();
                sendgrid_email.get_first_content().value = email_body.into();
                sendgrid_email
            },
        }
    }

    /// Add a CC email to the email.
    ///
    /// Allow to send the email to multiple recipients.
    /// # Example
    /// ```
    /// use sendgrid_thin::Sendgrid;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let sendgrid = Sendgrid::builder(
    ///         "SENDGRID_API_KEY",
    ///         "from_email@example.com",
    ///         ["to_email_1@example.com","to_email_2@example.com"],
    ///         "subject of email",
    ///         "body of email",
    ///      )
    ///     .set_cc_emails(&["cc_email1@example.com", "cc_email2@example.com"])
    ///     .build()
    ///     .unwrap();
    ///
    ///     match sendgrid.send().await {
    ///         Ok(message) => println!("{message}"),
    ///         Err(err) => println!("Error sending email: {err}"),
    ///     }
    /// }
    /// ```
    pub fn set_cc_emails<T>(mut self, cc_emails: impl IntoIterator<Item = T>) -> SendgridBuilder
    where
        T: AsRef<str>,
    {
        self.sendgrid_email.get_first_personalization().cc = Some(
            cc_emails
                .into_iter()
                .map(|email| From {
                    email: email.as_ref().to_owned(),
                })
                .collect(),
        );
        self
    }

    /// Set the content type of the email.
    /// # Example
    /// ```
    /// use sendgrid_thin::{Sendgrid, ContentType};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let sendgrid = Sendgrid::builder(
    ///         "SENDGRID_API_KEY",
    ///         "from_email@example.com",
    ///         ["to_email_1@example.com","to_email_2@example.com"],
    ///         "subject of email",
    ///         "body of email",
    ///      )
    ///     .set_content_type(ContentType::Html)
    ///     .build()
    ///     .unwrap();
    ///
    ///     match sendgrid.send().await {
    ///         Ok(message) => println!("{message}"),
    ///         Err(err) => println!("Error sending email: {err}"),
    ///     }
    /// }
    /// ```
    pub fn set_content_type<T>(mut self, content_type: T) -> SendgridBuilder
    where
        T: AsRef<ContentType>,
    {
        self.sendgrid_email.get_first_content().content_type = match content_type.as_ref() {
            ContentType::Text => Some(String::from("text/plain")),
            ContentType::Html => Some(String::from("text/html")),
        };
        self
    }

    /// Set the time in unix timestamp when the email should be sent.
    /// # Example
    /// ```
    /// use sendgrid_thin::Sendgrid;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let sendgrid = Sendgrid::builder(
    ///         "SENDGRID_API_KEY",
    ///         "from_email@example.com",
    ///         ["to_email_1@example.com","to_email_2@example.com"],
    ///         "subject of email",
    ///         "body of email",
    ///      )
    ///     .set_send_at(1668271500)
    ///     .build()
    ///     .unwrap();
    ///
    ///     match sendgrid.send().await {
    ///         Ok(message) => println!("{message}"),
    ///         Err(err) => println!("Error sending email: {err}"),
    ///     }
    /// }
    /// ```
    pub fn set_send_at(mut self, send_at: u64) -> SendgridBuilder {
        self.sendgrid_email.send_at = Some(send_at);
        self
    }

    /// Enables a request timeout.
    ///
    /// The timeout is applied from when the request starts connecting until the response body has finished.
    ///
    /// Default is no timeout for async requests and 30 seconds for blocking requests.
    ///
    /// # Example
    /// ```
    /// use sendgrid_thin::Sendgrid;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let sendgrid = Sendgrid::builder(
    ///         "SENDGRID_API_KEY",
    ///         "from_email@example.com",
    ///         ["to_email_1@example.com","to_email_2@example.com"],
    ///         "subject of email",
    ///         "body of email",
    ///      )
    ///     .set_request_timeout(std::time::Duration::from_secs(10))
    ///     .build()
    ///     .unwrap();
    ///
    ///     match sendgrid.send().await {
    ///         Ok(message) => println!("{message}"),
    ///         Err(err) => println!("Error sending email: {err}"),
    ///     }
    /// }
    /// ```
    pub fn set_request_timeout(mut self, request_timeout: Duration) -> SendgridBuilder {
        self.request_timeout = Some(request_timeout);
        self
    }

    /// Builds the Sendgrid struct.
    /// # Example
    /// ```
    /// use sendgrid_thin::{SendgridBuilder, ContentType};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let sendgrid = SendgridBuilder::new(
    ///         "SENDGRID_API_KEY",
    ///         "from_email@example.com",
    ///         ["to_email_1@example.com","to_email_2@example.com"],
    ///         "subject of email",
    ///         "body of email",
    ///      )
    ///     .build()
    ///     .unwrap();
    ///
    ///     match sendgrid.send().await {
    ///         Ok(message) => println!("{message}"),
    ///         Err(err) => println!("Error sending email: {err}"),
    ///     }
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if the Sendgrid struct is not valid.
    pub fn build(self) -> Result<Sendgrid, SendgridError> {
        Ok(Sendgrid {
            api_key: self.api_key,
            sendgrid_request_body: serde_json::to_string(&self.sendgrid_email)?,
            request_timeout: self.request_timeout,
            send_at: self.sendgrid_email.send_at,
        })
    }
}

impl Sendgrid {
    /// Create a new sendgrid builder.
    /// # Example
    /// ```
    /// use sendgrid_thin::{Sendgrid, ContentType};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let sendgrid = Sendgrid::builder(
    ///         "SENDGRID_API_KEY",
    ///         "from_email@example.com",
    ///         ["to_email_1@example.com","to_email_2@example.com"],
    ///         "subject of email",
    ///         "body of email",
    ///      )
    ///     .set_content_type(ContentType::Text)
    ///     .set_send_at(1668271500)
    ///     .set_cc_emails(&["cc_email_1@example.com", "cc_email_2@example.com"])
    ///     .build()
    ///     .unwrap();
    ///
    ///     match sendgrid.send().await {
    ///         Ok(message) => println!("{message}"),
    ///         Err(err) => println!("Error sending email: {err}"),
    ///     }
    /// }
    /// ```
    pub fn builder<T, U>(
        api_key: impl Into<String>,
        from_email: impl Into<String>,
        to_emails: U,
        email_subject: impl Into<String>,
        email_body: impl Into<String>,
    ) -> SendgridBuilder
    where
        T: Into<String>,
        U: IntoIterator<Item = T>,
    {
        SendgridBuilder::new(api_key, from_email, to_emails, email_subject, email_body)
    }

    fn is_scheduled(&self) -> Option<String> {
        if let Some(send_at) = self.send_at {
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Error getting current time")
                .as_secs();
            if current_time < send_at {
                return Some(format!(
                    "Email successfully scheduled to be sent at {send_at}."
                ));
            }
        }
        None
    }

    /// Sends an email using Sendgrid API with a blocking client.
    /// # Example
    /// ```
    /// use sendgrid_thin::{Sendgrid, ContentType};
    ///
    /// fn main() {
    ///     let sendgrid = Sendgrid::builder(
    ///         "SENDGRID_API_KEY",
    ///         "from_email@example.com",
    ///         ["to_email_1@example.com","to_email_2@example.com"],
    ///         "subject of email",
    ///         "body of email",
    ///      )
    ///     .set_content_type(ContentType::Text)
    ///     .set_send_at(1668271500)
    ///     .set_cc_emails(&["cc_email_1@example.com", "cc_email_2@example.com"])
    ///     .build()
    ///     .unwrap();
    ///
    ///     match sendgrid.send_blocking() {
    ///         Ok(message) => println!("{message}"),
    ///         Err(err) => println!("Error sending email: {err}"),
    ///     }
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if the request fails.
    #[cfg(feature = "blocking")]
    pub fn send_blocking(&self) -> Result<String, SendgridError> {
        let client;
        if let Some(request_timeout) = self.request_timeout {
            client = reqwest::blocking::Client::builder()
                .timeout(request_timeout)
                .build()?;
        } else {
            client = reqwest::blocking::Client::new();
        }

        let response = client
            .post("https://api.sendgrid.com/v3/mail/send")
            .bearer_auth(&self.api_key)
            .header("Content-Type", "application/json")
            .body(self.sendgrid_request_body.clone())
            .send()?;

        if !response.status().is_success() {
            return Err(SendgridError::new_request_error(
                &response
                    .text()
                    .unwrap_or(String::from("Error getting response text")),
            ));
        }

        let message = self
            .is_scheduled()
            .unwrap_or(String::from("Email successfully sent."));

        Ok(message)
    }

    /// Sends an email using Sendgrid API with a non-blocking client.
    /// # Example
    /// ```
    /// use sendgrid_thin::{Sendgrid, ContentType};
    /// #[tokio::main]
    /// async fn main() {
    ///     let sendgrid = Sendgrid::builder(
    ///         "SENDGRID_API_KEY",
    ///         "from_email@example.com",
    ///         ["to_email_1@example.com","to_email_2@example.com"],
    ///         "subject of email",
    ///         "body of email",
    ///      )
    ///     .set_content_type(ContentType::Text)
    ///     .set_send_at(1668271500)
    ///     .set_cc_emails(&["cc_email_1@example.com", "cc_email_2@example.com"])
    ///     .build()
    ///     .unwrap();
    ///
    ///     match sendgrid.send().await {
    ///         Ok(message) => println!("{message}"),
    ///         Err(err) => println!("Error sending email: {err}"),
    ///     }
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if the request fails.
    pub async fn send(&self) -> Result<String, SendgridError> {
        let client;
        if let Some(request_timeout) = self.request_timeout {
            client = reqwest::Client::builder()
                .timeout(request_timeout)
                .build()?;
        } else {
            client = reqwest::Client::new();
        }

        let response = client
            .post("https://api.sendgrid.com/v3/mail/send")
            .bearer_auth(&self.api_key)
            .header("Content-Type", "application/json")
            .body(self.sendgrid_request_body.clone())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(SendgridError::new_request_error(
                &response
                    .text()
                    .await
                    .unwrap_or(String::from("Error getting response text")),
            ));
        }

        let message = self
            .is_scheduled()
            .unwrap_or(String::from("Email successfully sent."));

        Ok(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sendgrid_instance() {
        let sendgrid = Sendgrid::builder(
            "SENDGRID_API_KEY",
            "test_from@test.com",
            ["test_to@test.com"],
            "subject",
            "body",
        );
        assert_eq!(
            sendgrid.sendgrid_email,
            SendgridEmail {
                personalizations: [Personalization {
                    to: Vec::from([From {
                        email: String::from("test_to@test.com")
                    }]),
                    cc: None,
                }],
                from: From {
                    email: String::from("test_from@test.com")
                },
                subject: String::from("subject"),
                content: [Content {
                    content_type: Some(String::from("text/plain")),
                    value: String::from("body"),
                }],
                send_at: None,
            }
        );
    }

    #[test]
    fn test_sendgrid_instance_builder() {
        let sendgrid = Sendgrid::builder(
            "SENDGRID_API_KEY",
            "test_from@test.com",
            ["test_to@test.com"],
            "subject",
            "body",
        );
        assert_eq!(
            sendgrid.sendgrid_email,
            SendgridEmail {
                personalizations: [Personalization {
                    to: Vec::from([From {
                        email: String::from("test_to@test.com")
                    }]),
                    cc: None,
                }],
                from: From {
                    email: String::from("test_from@test.com")
                },
                subject: String::from("subject"),
                content: [Content {
                    content_type: Some(String::from("text/plain")),
                    value: String::from("body"),
                }],
                send_at: None,
            }
        );
    }

    #[test]
    fn test_content_type() {
        let sendgrid = Sendgrid::builder(
            "SENDGRID_API_KEY",
            "test_from@test.com",
            ["test_to@test.com"],
            "subject",
            "body",
        );
        assert_eq!(
            sendgrid.sendgrid_email.content[0].content_type,
            Some(String::from("text/plain"))
        );

        let sendgrid = sendgrid.set_content_type(ContentType::Html);
        assert_eq!(
            sendgrid.sendgrid_email.content[0].content_type,
            Some(String::from("text/html"))
        );

        let sendgrid = sendgrid.set_content_type(ContentType::Text);
        assert_eq!(
            sendgrid.sendgrid_email.content[0].content_type,
            Some(String::from("text/plain"))
        );
    }

    #[test]
    fn test_set_body_and_subject_email() {
        let sendgrid = Sendgrid::builder(
            "SENDGRID_API_KEY",
            "from_email@example.com",
            ["to_email@example.com"],
            "subject_test",
            "body_test",
        )
        .set_content_type(ContentType::Text)
        .build()
        .unwrap();
        assert_eq!(sendgrid.sendgrid_request_body, "{\"personalizations\":[{\"to\":[{\"email\":\"to_email@example.com\"}]}],\"from\":{\"email\":\"from_email@example.com\"},\"subject\":\"subject_test\",\"content\":[{\"type\":\"text/plain\",\"value\":\"body_test\"}]}");
    }

    #[test]
    fn test_set_body_email_with_cc_emails() {
        let sendgrid = Sendgrid::builder(
            "SENDGRID_API_KEY",
            "from_email@example.com",
            ["to_email@example.com"],
            "subject_test",
            "body_test",
        )
        .set_content_type(ContentType::Text)
        .set_cc_emails(&["cc_email1@example.com", "cc_email2@example.com"])
        .build()
        .unwrap();
        assert_eq!(sendgrid.sendgrid_request_body, "{\"personalizations\":[{\"to\":[{\"email\":\"to_email@example.com\"}],\"cc\":[{\"email\":\"cc_email1@example.com\"},{\"email\":\"cc_email2@example.com\"}]}],\"from\":{\"email\":\"from_email@example.com\"},\"subject\":\"subject_test\",\"content\":[{\"type\":\"text/plain\",\"value\":\"body_test\"}]}");
    }

    #[test]
    fn test_set_send_at() {
        let sendgrid = Sendgrid::builder(
            "SENDGRID_API_KEY",
            "test_from@test.com",
            ["test_to@test.com"],
            "subject",
            "body",
        );
        assert_eq!(sendgrid.sendgrid_email.send_at, None);
        let sendgrid = sendgrid.set_send_at(1668271500);
        assert_eq!(sendgrid.sendgrid_email.send_at, Some(1668271500));
    }

    #[test]
    fn test_set_request_timeout() {
        let sendgrid = Sendgrid::builder(
            "SENDGRID_API_KEY",
            "test_from@test.com",
            ["test_to@test.com"],
            "subject",
            "body",
        );
        assert_eq!(sendgrid.request_timeout, None);
        let sendgrid = sendgrid.set_request_timeout(Duration::from_secs(10));
        assert_eq!(sendgrid.request_timeout, Some(Duration::from_secs(10)));
    }

    #[test]
    fn test_traits() {
        fn assert_sync_traits<T: Sized + Send + Sync + Unpin>() {}
        assert_sync_traits::<Sendgrid>();
        assert_sync_traits::<SendgridBuilder>();

        fn assert_derived_traits<T: Clone + std::fmt::Debug + PartialEq>() {}
        assert_derived_traits::<Sendgrid>();
        assert_derived_traits::<SendgridBuilder>();

        fn assert_error_traits<T: std::error::Error + Clone + PartialEq>() {}
        assert_error_traits::<SendgridError>();
    }
}
