use actix_files::NamedFile;
use actix_web::{get, HttpRequest, HttpResponse, Responder, web};
use tracing::debug;
use crate::CONFIG;

const INDEX_HTML: &str = include_str!("../../website/index.html");
const MAIN_JS: &str = include_str!("../../website/main.js");
const STYLE_CSS: &str = include_str!("../../website/style.css");
const ROBOTO_MONO_TTF: &[u8] = include_bytes!("../../website/roboto_mono.ttf");

// The function is async because the actix-web macro requires it.
#[allow(clippy::unused_async)]
#[get("/")]
pub async fn index(req: HttpRequest) -> Result<impl Responder, Box<dyn std::error::Error>> {
	debug!("Got request for Index");
	if let Some(ref path) = CONFIG.frontend_location {
		let path = format!("{path}/index.html");
		return Ok(NamedFile::open(path)?.into_response(&req));
	}

	let response = get_embedded_file("index.html").unwrap();
	Ok(
		HttpResponse::Ok()
			.content_type(response.0)
			.body(response.1)
	)
}

// The function is async because the actix-web macro requires it.
#[allow(clippy::unused_async)]
#[get("/assets/{asset:.*}")]
pub async fn serve_file(asset: web::Path<String>, req: HttpRequest) -> Result<impl Responder, Box<dyn std::error::Error>> {
	let asset = asset.into_inner();
	debug!("Got request for file: {asset}");

	if let Some(ref path) = CONFIG.frontend_location {
		let path = format!("{path}/{asset}");
		return Ok(NamedFile::open(path)?.into_response(&req));
	}

	// Tuple of MIME Type and Content.
	let response_opt: Option<(&str, &[u8])> = get_embedded_file(asset.as_str());


	if let Some(response) = response_opt {
		Ok(
			HttpResponse::Ok()
				.content_type(response.0)
				.body(response.1)
		)
	} else {
		Ok(HttpResponse::NotFound().finish())
	}
}

/// Returns a Tuple of Mime Type (as &str) and file content (as &[u8]).
fn get_embedded_file(file: &str) -> Option<(&'static str, &'static [u8])> {
	debug!("Getting embedded file: {file}");
	match file {
		"index.html" => { Some(("text/html", INDEX_HTML.as_bytes())) }
		"main.js" => { Some(("text/javascript", MAIN_JS.as_bytes())) }
		"style.css" => { Some(("text/css", STYLE_CSS.as_bytes())) }
		"roboto_mono.ttf" => { Some(("font/ttf", ROBOTO_MONO_TTF)) }
		_ => { None }
	}
}

