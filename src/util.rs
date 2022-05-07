use actix_web::http::Uri;
use base64::CharacterSet;
use chrono::Local;
use rand::RngCore;

/// Size of the random
const URL_SIZE: usize = 4;

const BASE64_CONFIG: base64::Config = base64::Config::new(CharacterSet::UrlSafe, false);


/// Checks if the URL starts with `http` or `https`.
/// If it doesn't it prepends `http`.
/// We have to do this because otherwise the browser will assume we are redirecting
/// to a subpage on the same domain.
pub fn check_url_http(url: String) -> String {
	if url.starts_with("http://") || url.starts_with("https://") {
		return url;
	}


	format!("http://{url}")
}

/// Generates some random chars.
/// Used for the random ID.
/// We generate a few random bytes (How many is defined by `URL_SIZE`.
pub fn generate_random_chars() -> String {
	let mut random_bytes: [u8; URL_SIZE] = [0; URL_SIZE];
	rand::thread_rng().fill_bytes(&mut random_bytes);


	base64::encode_config(&random_bytes, BASE64_CONFIG)
}

/// If the URI is longer than 0 chars, it contains a `/` char at the first position.
/// If it is longer than 0 chars, this removes the prepended `/` char.
#[allow(clippy::similar_names)]
pub fn uri_to_url(uri: &Uri) -> String {
	let mut url = uri.to_string();
	if url.len() > 1 {
		url.remove(0);
	}


	url
}

/// Returns the current local time in milliseconds.
pub fn time_now() -> i64 {
	Local::now().timestamp_millis()
}