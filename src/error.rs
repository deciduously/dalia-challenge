// Collection type for all the errors this app can throw

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Connection pool error")]
    ConnError(#[from] r2d2::Error),
    #[error("Datetime error")]
    DateTimeError(#[from] chrono::ParseError),
    #[error("Database error")]
    DbError(#[from] diesel::result::Error),
    #[error("Environment variable error")]
    EnvVarError(#[from] std::env::VarError),
    #[error("IO Error")]
    IoError(#[from] std::io::Error),
    #[error("HTTP error")]
    HttpError(#[from] hyper::error::Error),
    #[error("Migration error")]
    MigrationError(#[from] diesel_migrations::RunMigrationsError),
    #[error("Scrape Error")]
    ScrapeError(#[from] reqwest::Error),
    #[error("URI error")]
    URIError(#[from] hyper::http::uri::InvalidUri),
    #[error("Could not parse UTF-8")]
    UTF8Error(#[from] std::str::Utf8Error),
    #[error("Infallible")]
    Infallible(#[from] std::convert::Infallible),
}

pub type AppResult<T> = Result<T, AppError>;
