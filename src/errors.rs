use reqwest::StatusCode;
use url::ParseError;

use std::fmt;

#[derive(Debug)]
pub enum BoticordError {
    Reqwest(reqwest::Error),
    Url(ParseError),
}

impl BoticordError {
    pub fn status(&self) -> Option<StatusCode> {
        match self {
            BoticordError::Reqwest(e) => e.status(),
            BoticordError::Url(_) => None,
        }
    }
}

impl std::error::Error for BoticordError {}

impl fmt::Display for BoticordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BoticordError::Reqwest(e) => e.fmt(f),
            BoticordError::Url(e) => e.fmt(f),
        }
    }
}

pub fn from(e: reqwest::Error) -> BoticordError {
    BoticordError::Reqwest(e)
}