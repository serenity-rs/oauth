use hyper::Error as HyperError;
use reqwest::Error as ReqwestError;
use serde_json::Error as JsonError;
use serde_urlencoded::ser::Error as UrlEncodeError;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;

/// Result type used throughout the library's public result functions.
pub type Result<T> = StdResult<T, Error>;

/// Standard error enum used to wrap different potential error types.
#[derive(Debug)]
pub enum Error {
    /// An error from the `hyper` crate.
    Hyper(HyperError),
    /// An error from the `reqwest` crate.
    Reqwest(ReqwestError),
    /// An error from the `serde_json` crate.
    Json(JsonError),
    /// An error from the `serde_urlencoded` crate.
    UrlEncode(UrlEncodeError),
}

impl From<HyperError> for Error {
    fn from(err: HyperError) -> Self {
        Error::Hyper(err)
    }
}

impl From<ReqwestError> for Error {
    fn from(err: ReqwestError) -> Self {
        Error::Reqwest(err)
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Error::Json(err)
    }
}

impl From<UrlEncodeError> for Error {
    fn from(err: UrlEncodeError) -> Self {
        Error::UrlEncode(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.to_string().as_str())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Hyper(ref inner) => inner.description(),
            Error::Reqwest(ref inner) => inner.description(),
            Error::Json(ref inner) => inner.description(),
            Error::UrlEncode(ref inner) => inner.description(),
        }
    }
}
