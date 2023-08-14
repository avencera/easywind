use std::path::PathBuf;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid root dir path: {0}")]
    InvalidRootDir(PathBuf),

    #[error("Unable to render template: {0}")]
    TemplateError(#[from] sailfish::RenderError),

    #[error(transparent)]
    HttpError(#[from] axum::http::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    Unknown(#[from] eyre::Report),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {:?}", self),
        )
            .into_response()
    }
}
