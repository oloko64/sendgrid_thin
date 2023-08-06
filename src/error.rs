#[derive(Debug, Clone, PartialEq)]
pub enum SendgridError {
    RequestError(String),
    SerdeError(String),
}

impl SendgridError {
    pub fn new_request_error(msg: &str) -> Self {
        SendgridError::RequestError(msg.to_string())
    }
}

impl std::fmt::Display for SendgridError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SendgridError::RequestError(msg) | SendgridError::SerdeError(msg) => {
                write!(f, "{msg}")
            }
        }
    }
}

impl From<serde_json::Error> for SendgridError {
    fn from(err: serde_json::Error) -> Self {
        SendgridError::SerdeError(err.to_string())
    }
}

impl From<reqwest::Error> for SendgridError {
    fn from(err: reqwest::Error) -> Self {
        SendgridError::RequestError(err.to_string())
    }
}

impl std::error::Error for SendgridError {}
