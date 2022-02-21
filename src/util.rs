use actix_web::http::Uri;
use base64::CharacterSet;
use rand::RngCore;

const URL_SIZE: usize = 6;
const BASE64_CONFIG: base64::Config = base64::Config::new(CharacterSet::UrlSafe, false);


pub fn generate_random_chars() -> String {
	let mut random_bytes: [u8; URL_SIZE] = [0; URL_SIZE];
	rand::thread_rng().fill_bytes(&mut random_bytes);


	base64::encode_config(&random_bytes, BASE64_CONFIG)
}

pub fn uri_to_url(uri: &Uri) -> String {
	let mut url = uri.to_string();
	if url.len() > 1 {
		url.remove(0);
	}


	url
}

pub fn sanitize_url(url: String) -> String {
	if url.starts_with("http://") || url.starts_with("https://") {
		return url;
	}


	format!("http://{url}")
}