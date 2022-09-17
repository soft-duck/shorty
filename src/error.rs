use actix_web::{HttpResponse, HttpResponseBuilder, ResponseError};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShortyError {
	#[error("Link with provided ID already exists")]
	LinkConflict,
	#[error("Link exceeds maximum length allowed.")]
	LinkExceedsMaxLength,
	#[error("Custom ID exceeds maximum length allowed.")]
	CustomIDExceedsMaxLength,
	#[error("Link is empty.")]
	LinkEmpty,
	#[error("Maximum retries to generate a random link ID were exceeded.")]
	RandomIDMaxRetriesExceeded,
	#[error(transparent)]
	Database(#[from] sqlx::Error),
	#[error(transparent)]
	Dotenv(#[from] dotenv::Error),
	#[error(transparent)]
	Other(#[from] anyhow::Error),
}

impl ResponseError for ShortyError {
	fn status_code(&self) -> StatusCode {
		match self {
			ShortyError::LinkConflict => StatusCode::CONFLICT,
			ShortyError::LinkExceedsMaxLength
			| ShortyError::LinkEmpty
			| ShortyError::CustomIDExceedsMaxLength => StatusCode::BAD_REQUEST,
			_ => StatusCode::INTERNAL_SERVER_ERROR,
		}
	}

	fn error_response(&self) -> HttpResponse<BoxBody> {
		HttpResponseBuilder::new(self.status_code())
			.body(self.to_string())
	}
}
