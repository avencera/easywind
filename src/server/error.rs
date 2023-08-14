use std::path::PathBuf;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use log::error;
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

    #[error("Unable to get metadata for file, path: {0}")]
    FileMetadataError(PathBuf),

    #[error("File larger than metadata reported, path: {0}")]
    FileReadBufferOverflow(PathBuf),

    #[error(transparent)]
    Unknown(#[from] eyre::Report),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        error!("{:?}", self);

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {:?}", self),
        )
            .into_response()
    }
}
