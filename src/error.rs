use std::time::SystemTimeError;

#[derive(Debug)]
pub enum SendgridError {
    RequestError(reqwest::Error),
    SerdeError(serde_json::Error),
    SystemTimeError(SystemTimeError),
    CustomError(String),
}

impl SendgridError {
    pub fn new_custom_error(msg: &str) -> Self {
        SendgridError::CustomError(msg.to_string())
    }
}

impl std::fmt::Display for SendgridError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SendgridError::RequestError(msg) => {
                write!(f, "{msg}")
            }
            SendgridError::SerdeError(err) => {
                write!(f, "{err}")
            }
            SendgridError::SystemTimeError(err) => {
                write!(f, "{err}")
            }
            SendgridError::CustomError(msg) => {
                write!(f, "{msg}")
            }
        }
    }
}

impl From<std::time::SystemTimeError> for SendgridError {
    fn from(err: std::time::SystemTimeError) -> Self {
        SendgridError::SystemTimeError(err)
    }
}

impl From<serde_json::Error> for SendgridError {
    fn from(err: serde_json::Error) -> Self {
        SendgridError::SerdeError(err)
    }
}

impl From<reqwest::Error> for SendgridError {
    fn from(err: reqwest::Error) -> Self {
        SendgridError::RequestError(err)
    }
}

impl std::error::Error for SendgridError {}
