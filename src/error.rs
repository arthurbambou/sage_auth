//! API error and common error

use reqwest::{Error as ReqwestError, Response};
use std::error::Error as StdError;
use std::fmt;
use std::result::Result as StdResult;
use url::ParseError;

use crate::types::ErrorMessage;

pub type Result<T> = StdResult<T, Error>;

/// Error from Mojang API
///
/// The name of the enum means `error` from the message,
/// it's the short description of the error.
///
/// The `String` contained in the enum means `errorMessage` from the message,
/// it's the longer description which can be shown to the user.
#[derive(Debug)]
pub enum ApiError {
    MethodNotAllowed(String),
    NotFound(String),
    ForbiddenOperationException(String),
    IllegalArgumentException(String),
    UnsupportedMediaType(String),

    /// Unknown error
    Unknown {
        error: String,
        message: String,
    },
}

/// Common errors
#[derive(Debug)]
pub enum Error {
    /// An error occurred from reqwest
    Reqwest(ReqwestError),

    /// Url parse error.
    UrlParseError(ParseError),

    /// Missing required fields to generate a request.
    MissingField(&'static str),

    /// API error, from Mojang server
    API(ApiError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Reqwest(reqwest_error) => write!(f, "Reqwest error: {}", reqwest_error),
            Error::UrlParseError(url_parse_error) => {
                write!(f, "URL parse error: {}", url_parse_error)
            }
            Error::MissingField(field) => write!(f, "Missing field: {}", field),
            Error::API(api_error) => match api_error {
                ApiError::MethodNotAllowed(message) => {
                    write!(f, "API error: MethodNotAllowed ({})", message)
                }
                ApiError::NotFound(message) => write!(f, "API error: NotFound ({})", message),
                ApiError::ForbiddenOperationException(message) => {
                    write!(f, "API error: ForbiddenOperationException ({})", message)
                }
                ApiError::IllegalArgumentException(message) => {
                    write!(f, "API error: IllegalArgumentException ({})", message)
                }
                ApiError::UnsupportedMediaType(message) => {
                    write!(f, "API error: UnsupportedMediaType ({})", message)
                }
                ApiError::Unknown { error, message } => {
                    write!(f, "API error: {} ({})", error, message)
                }
            },
        }
    }
}

impl StdError for Error {
    fn cause(&self) -> Option<&dyn StdError> {
        match self {
            Error::Reqwest(reqwest_error) => Some(reqwest_error),
            Error::UrlParseError(url_parse_error) => Some(url_parse_error),
            _ => None,
        }
    }
}

impl From<ReqwestError> for Error {
    fn from(error: ReqwestError) -> Self {
        Error::Reqwest(error)
    }
}

impl From<ParseError> for Error {
    fn from(error: ParseError) -> Self {
        Error::UrlParseError(error)
    }
}

impl Error {
    pub(crate) async fn from_response(error: Response) -> Self {
        let msg = error.json::<ErrorMessage>().await;
        if msg.is_err() {
            return msg.unwrap_err().into();
        }
        let msg = msg.unwrap();

        if msg.error == "ForbiddenOperationException" {
            Error::API(ApiError::ForbiddenOperationException(msg.error_message))
        } else if msg.error == "IllegalArgumentException" {
            Error::API(ApiError::IllegalArgumentException(msg.error_message))
        } else if msg.error == "Method Not Allowed" {
            Error::API(ApiError::MethodNotAllowed(msg.error_message))
        } else if msg.error == "Not Found" {
            Error::API(ApiError::NotFound(msg.error_message))
        } else if msg.error == "Unsupported Media Type" {
            Error::API(ApiError::UnsupportedMediaType(msg.error_message))
        } else {
            Error::API(ApiError::Unknown {
                error: msg.error,
                message: msg.error_message,
            })
        }
    }
}
