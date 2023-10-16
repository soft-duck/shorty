use actix_files::NamedFile;
use actix_web::{get, HttpRequest, HttpResponse, post, Responder, web};
use tracing::{debug, info};
use utoipa::OpenApi;

use crate::CONFIG;
use crate::config::Config;
use crate::error::ShortyError;
use crate::LinkConfig;
use crate::LinkStore;
use crate::util::uri_to_url;

#[derive(OpenApi)]
#[openapi(
	paths(
		get_shortened,
		get_config,
		create_shortened,
		create_shortened_custom,
	),
	tags(
		(name = "/", description = "Simple shortening"),
		(name = "/custom", description = "Advanced shortening"),
		(name = "/config", description = "Server configuration"),
	)
)]
pub struct ApiDoc;

// The function is async because the actix-web macro requires it.
#[allow(clippy::unused_async)]
#[get("/")]
pub async fn index(req: HttpRequest) -> Result<impl Responder, Box<dyn std::error::Error>> {
	debug!("Got request for Index");
	if let Some(ref path) = CONFIG.frontend_location {
		let path = format!("{path}/index.html");
		return Ok(NamedFile::open(path)?.into_response(&req));
	}

	#[cfg(feature = "integrated-frontend")]
	{
		let response = get_embedded_file("index.html").unwrap();

		return Ok(
			HttpResponse::Ok()
				.content_type(response.0)
				.body(response.1)
		);
	}

	#[allow(unreachable_code)]
	{ unreachable!("If this is encountered, the `frontend_location` config key was not ensured to be present"); }
}

/// Redirect to the aliased url
#[utoipa::path(
	tag = "/",
	params((
		"link_id" = inline(String),
		Path,
		description = "The id of the aliased url",
	)),
	responses(
		(status = 307, description = "Redirection to aliased url"),
		(status = 404, description = "Shortened ID couldn't be found or was expired"),
	),
)]
#[get("/{link_id:.*}")]
async fn get_shortened(
	params: web::Path<String>,
	link_store: web::Data<LinkStore>,
) -> Result<impl Responder, ShortyError> {
	let link_id = params.into_inner();
	debug!("Got request for {link_id}");


	if let Some(link) = link_store.get(link_id.as_str()).await {
		info!("Return url for {link_id} is {link}");
		Ok(
			HttpResponse::TemporaryRedirect()
				.append_header(("Location", link.redirect_to.as_str()))
				.finish()
		)
	} else {
		Ok(HttpResponse::NotFound().finish())
	}
}

/// Retrieves the servers configuration details
#[utoipa::path(
	tag = "/config",
	responses(
		(status = 200, body = inline(Config), description = "The server config as json"),
	),
)]
// The function is async because the actix-web macro requires it.
#[allow(clippy::unused_async)]
#[get("/config")]
async fn get_config() -> impl Responder {
	HttpResponse::Ok()
		.content_type("application/json; charset=utf-8")
		.body(CONFIG.json_string())
}

/// Create a simple, unconfigured shortened link.
#[utoipa::path(
	tag = "/",
	params((
		"url" = inline(String),
	Path,
		description = "The url to shorten",
	)),
	responses(
		(status = 200, description = "The url was successfully shortened"),
	),
)]
#[post("/{url:.*}")]
#[allow(clippy::similar_names)]
async fn create_shortened(
	req: HttpRequest,
	link_store: web::Data<LinkStore>,
) -> Result<impl Responder, ShortyError> {
	let uri = req.uri();
	debug!("URI is {uri}");
	let url = uri_to_url(uri);

	let link = link_store.create_link(url).await?;
	let formatted = link.formatted();
	info!("Shortening URL {} to {}", link.redirect_to, formatted);


	Ok(
		HttpResponse::Ok()
			.content_type("text/plain; charset=utf-8")
			.body(formatted)
	)
}

/// Advanced url shortening
///
/// Shortens a URL, allowing for advanced configuration.
#[utoipa::path(
	tag = "/custom",
	request_body(content = inline(LinkConfig), description = "The settings for the url to alias"),
	responses(
		(status = 200, description = "The url was successfully registered as an alias and is now retrievable with at the get endpoint"),
		(status = 400, description = "Json is malformed, the link exceeds the max length allowed by the server or the link was empty"),
		(status = 409, description = "The specified ID is already in use"),
	),
)]
#[post("/custom")]
async fn create_shortened_custom(
	link_store: web::Data<LinkStore>,
	link_config: web::Json<LinkConfig>,
) -> Result<impl Responder, ShortyError> {
	let link_config = link_config.into_inner();

	let link = link_store.create_link_with_config(link_config).await?;
	let formatted = link.formatted();
	info!("Shortening URL {} to {}", link.redirect_to, formatted);


	Ok(
		HttpResponse::Ok()
			.content_type("text/plain; charset=utf-8")
			.body(formatted)
	)
}

#[allow(clippy::unused_async)]
#[get("/favicon.ico")]
async fn get_favicon() -> Result<impl Responder, ShortyError> {
	debug!("Got request for favicon");
	Ok(HttpResponse::NotFound().finish())
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

	#[cfg(feature = "integrated-frontend")]
	{
		// Tuple of MIME Type and Content.
		let response_opt: Option<(&str, &[u8])> = get_embedded_file(asset.as_str());


		return if let Some(response) = response_opt {
			Ok(
				HttpResponse::Ok()
					.content_type(response.0)
					.body(response.1)
			)
		} else {
			Ok(HttpResponse::NotFound().finish())
		};
	}

	#[allow(unreachable_code)]
	{ unreachable!("If this is encountered, the `frontend_location` config key was not ensured to be present"); }
}

#[cfg(feature = "integrated-frontend")]
include!(concat!(env!("OUT_DIR"), "/generated.rs"));

/// Returns a Tuple of Mime Type (as &str) and file content (as &[u8]).
#[cfg(feature = "integrated-frontend")]
fn get_embedded_file(file: &str) -> Option<(&'static str, &'static [u8])> {
	use std::collections::HashMap;

	let resources: HashMap<&str, static_files::Resource> = generate();

	debug!("Getting embedded file: {file}");

	resources.get(file).map(|file| {
		(file.mime_type, file.data)
	}).or_else(|| {
		tracing::warn!("Got request for {file} but couldn't find embedded asset.");
		None
	})
}

