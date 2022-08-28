use actix_web::http::Uri;
use base64::CharacterSet;
use chrono::Local;
use rand::RngCore;
use sqlx::{Pool, Sqlite};
use tracing::error;
use crate::link::Link;
use crate::ShortyError;

/// How many random bytes should be generated for the IDs.
/// They are then URL encoded, so a `URL_SIZE` of 4 corresponds to 6 chars without padding.
const URL_SIZE: usize = 4;

const BASE64_CONFIG: base64::Config = base64::Config::new(CharacterSet::UrlSafe, false);

const RANDOM_ID_RETRIES: u32 = 3;

/// Checks if the URL starts with `http` or `https`.
/// If it doesn't it prepends `http`.
/// We have to do this because otherwise the browser will assume we are redirecting
/// to a subpage on the same domain.
#[must_use]
pub fn ensure_http_prefix(url: String) -> String {
	if url.starts_with("http://") || url.starts_with("https://") {
		return url;
	}


	format!("http://{url}")
}

/// This function replaces illegal URL chars with ones that can be used in urls.
/// Currently it just replaces spaces with underscores, additions might happen in the future.
pub fn replace_illegal_url_chars(s: impl AsRef<str>) -> String {
	s.as_ref().replace(" ", "_")
}

/// Generates some random chars.
/// Used for the random ID.
/// We generate a few random bytes (How many is defined by `URL_SIZE`.
#[must_use]
pub fn generate_random_chars() -> String {
	let mut random_bytes: [u8; URL_SIZE] = [0; URL_SIZE];
	rand::thread_rng().fill_bytes(&mut random_bytes);


	base64::encode_config(&random_bytes, BASE64_CONFIG)
}

/// Calls [`generate_random_chars`] and looks if the id already exists in the database.
/// Gives up after [`RANDOM_ID_RETRIES`] tries.
/// Currently, if it generates a random ID and a link with that ID exists in the Database, it
/// considers the ID as "occupied", even if the link in question is already expired.
///
/// # Errors
///
/// Errors if it fails to generate a valid link in [`RANDOM_ID_RETRIES`] tries.
///
/// Errors if there is some problem communicating with the database.
pub async fn get_random_id(pool: &Pool<Sqlite>) -> Result<String, ShortyError> {
	for _ in 0..RANDOM_ID_RETRIES {
		let random_chars = generate_random_chars();
		if !Link::link_exists(random_chars.as_str(), pool).await? {
			return Ok(random_chars);
		}
	}
	error!("Tried {} times to generate a random ID, but failed!", RANDOM_ID_RETRIES);


	Err(ShortyError::RandomIDMaxRetriesExceeded)
}

/// If the URI is longer than 0 chars, it contains a `/` char at the first position.
/// If it is longer than 0 chars, this removes the prepended `/` char.
#[allow(clippy::similar_names)]
#[must_use]
pub fn uri_to_url(uri: &Uri) -> String {
	let mut url = uri.to_string();
	if !url.is_empty() {
		url.remove(0);
	}


	url
}

/// Returns the current local time in milliseconds.
#[must_use]
pub fn time_now() -> i64 {
	Local::now().timestamp_millis()
}
