use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ImageError {
    #[error("Image decoding error: {0}")]
    ImageDecodeError(String),

    #[error("Image manipulation error: {0}")]
    ImageManipulationError(String),
}

impl IntoResponse for ImageError {
    fn into_response(self) -> Response {
        let status = match &self {
            ImageError::ImageDecodeError(_) => StatusCode::BAD_REQUEST,
            ImageError::ImageManipulationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, format!("{}", self)).into_response()
    }
}

impl From<image::ImageError> for ImageError {
    fn from(err: image::ImageError) -> Self {
        ImageError::ImageDecodeError(err.to_string())
    }
}

impl From<std::io::Error> for ImageError {
    fn from(err: std::io::Error) -> Self {
        ImageError::ImageManipulationError(err.to_string())
    }
}
