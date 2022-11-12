use serde::Serialize;

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

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct SendgridEmail<'a> {
    #[serde(skip)]
    pub api_key: &'a str,

    #[serde(rename = "personalizations")]
    pub personalizations: Vec<Personalization<'a>>,

    #[serde(rename = "from")]
    pub from: From<'a>,

    #[serde(rename = "subject")]
    pub subject: &'a str,

    #[serde(rename = "content")]
    pub content: Vec<Content<'a>>,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct Content<'a> {
    #[serde(rename = "type")]
    pub content_type: Option<&'a str>,

    #[serde(rename = "value")]
    pub value: Option<&'a str>,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct From<'a> {
    #[serde(rename = "email")]
    pub email: &'a str,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct Personalization<'a> {
    #[serde(rename = "to")]
    pub to: Vec<From<'a>>,

    #[serde(rename = "cc", skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<From<'a>>>,
}
