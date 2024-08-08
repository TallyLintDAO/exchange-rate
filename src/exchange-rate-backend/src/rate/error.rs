use std::fmt;
use std::error::Error;
use reqwest;
use serde_json;
use std::collections::HashMap;
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub enum FetchError {
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
    Other(String),
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FetchError::Reqwest(e) => write!(f, "Request error: {}", e),
            FetchError::SerdeJson(e) => write!(f, "JSON error: {}", e),
            FetchError::Other(e) => write!(f, "Other error: {}", e),
        }
    }
}

impl Error for FetchError {}

impl From<reqwest::Error> for FetchError {
    fn from(e: reqwest::Error) -> Self {
        FetchError::Reqwest(e)
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(e: serde_json::Error) -> Self {
        FetchError::SerdeJson(e)
    }
}
