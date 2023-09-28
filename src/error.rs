use crate::futures_usd::enums::events::Event;
use std::sync::mpsc::SendError;
use thiserror::Error;
use url::ParseError;

/// Holder for the various errors that can occur when interacting with the futures_usd crate.
#[derive(Error, Debug)]
pub enum BinanceConnectError {
    #[error("Url Parse error: {0}")]
    UrlParseError(ParseError),
    #[error("Socket error: {0}")]
    SocketError(tungstenite::Error),
    #[error("Mpsc send error: {0}")]
    MpscSendError(SendError<Event>),
    #[error("JSON error: {0}")]
    JsonError(serde_json::Error),
    #[error("HTTP error: {0}")]
    HttpError(reqwest::Error),
    #[error("HTTP Response error: {0}")]
    HttpResponseError(String),
    #[error("Other error: {0}")]
    Other(String),
}

impl From<ParseError> for BinanceConnectError {
    fn from(err: ParseError) -> Self {
        BinanceConnectError::UrlParseError(err)
    }
}

impl From<tungstenite::Error> for BinanceConnectError {
    fn from(err: tungstenite::Error) -> Self {
        BinanceConnectError::SocketError(err)
    }
}

impl From<serde_json::Error> for BinanceConnectError {
    fn from(err: serde_json::Error) -> Self {
        BinanceConnectError::JsonError(err)
    }
}

impl From<reqwest::Error> for BinanceConnectError {
    fn from(err: reqwest::Error) -> Self {
        BinanceConnectError::HttpError(err)
    }
}

impl From<SendError<Event>> for BinanceConnectError {
    fn from(err: SendError<Event>) -> Self {
        BinanceConnectError::MpscSendError(err)
    }
}
