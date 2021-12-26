use log::{log, Level};
use rocket::{http::Status, response::Responder, Request};
use std::{backtrace::Backtrace, error::Error};
use thiserror::Error;

pub type ApiResult<T> = Result<T, ApiError>;

/// Type to capture all errors that can happen throughout the app.
#[derive(Debug, Error)]
pub enum ApiError {
    /// CSV parsing error
    #[error("{source}")]
    Csv {
        #[from]
        source: csv::Error,
        backtrace: Backtrace,
    },

    /// Reqwest error
    #[error("{source}")]
    Reqwest {
        #[from]
        source: reqwest::Error,
        backtrace: Backtrace,
    },
}

// Ripped from https://github.com/LucasPickering/laulud/blob/master/api/src/error.rs
impl ApiError {
    /// Convert this error to an HTTP status code. For most types the exact
    /// code doesn't matter because we don't actually return HTTP errors in
    /// GraphQL. So in most cases, the only thing that matters is 4xx vs 5xx to
    /// determine the logging level. For error types that can be returned
    /// from oauth routes though, the exact code matters because those are pure
    /// HTTP (no GraphQL).
    pub fn to_status(&self) -> Status {
        match self {
            // 500
            Self::Csv { .. } => Status::InternalServerError,

            // Forward whatever status we got from Reqwest
            Self::Reqwest { source, .. } => {
                source
                    .status()
                    .and_then(|status| Status::from_code(status.as_u16()))
                    // No status code from reqwest, so that's a 500
                    .unwrap_or(Status::InternalServerError)
            }
        }
    }

    /// Log this error. Logging level will be based on the status code
    pub fn log(&self) {
        let log_level = if self.to_status().code >= 500 {
            Level::Error
        } else {
            Level::Debug
        };

        log!(
            log_level,
            "API Error: {}\n{}",
            self,
            self.backtrace()
                .map(|bt| bt.to_string())
                .unwrap_or_default()
        );
    }
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(
        self,
        _: &'r Request<'_>,
    ) -> rocket::response::Result<'static> {
        self.log();
        Err(self.to_status())
    }
}
