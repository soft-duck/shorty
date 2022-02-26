use std::error::Error;
use std::io::Read;

use actix_web::{App, get, HttpRequest, HttpResponse, HttpServer, post, Responder, web};
use tracing::{debug, info, instrument, Level};
use tracing_subscriber::EnvFilter;

use crate::config::Config;
use crate::link::link::{Link, LinkStore};
use crate::util::{generate_random_chars, sanitize_url, uri_to_url};

mod util;
mod link;
mod config;


#[get("/{shortened_url:.*}")]
#[instrument(skip_all)]
async fn get_shortened(params: web::Path<String>, link_store: web::Data<LinkStore>) -> impl Responder {
	let shortened_url = params.into_inner();
	debug!("Got request for {shortened_url}");

	if let Some(link) = link_store.get(shortened_url.as_str()).await {
		info!("Return url for {shortened_url} is {link}");
		HttpResponse::TemporaryRedirect()
			.append_header(("Location", link.redirect_to.as_str()))
			.finish()
	} else {
		HttpResponse::NotFound().finish()
	}
}

#[post("/{url:.*}")]
#[instrument(skip_all)]
async fn create_shortened(
	req: HttpRequest,
	link_store: web::Data<LinkStore>,
	config: web::Data<Config>,
) -> impl Responder {
	let uri = req.uri();
	info!("URI is {uri}");

	let url = uri_to_url(uri);
	let url = sanitize_url(url);

	let link = Link::new(url);
	let shortened_url = format!("http://{}/{}", config.public_url, link.id);
	info!("Shortening URL {} to {}", link.redirect_to, shortened_url);

	link_store.insert(link).await;


	HttpResponse::Ok().body(shortened_url)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let env_filter = EnvFilter::from_default_env()
		.add_directive(Level::INFO.into())
		.add_directive("shorty=debug".parse()?);

	tracing_subscriber::fmt()
		.with_env_filter(env_filter)
		.with_line_number(true)
		.with_file(true)
		.init();

	let config;
	{
		let mut file = std::fs::File::open("./config.toml")?;
		let mut content = String::new();
		file.read_to_string(&mut content)?;

		config = Config::new(content.as_str())?;
	}

	let config = web::Data::new(config);
	let _config = config.clone();

	let links = LinkStore::new();
	let links = web::Data::new(links);

	info!("Starting server at {}:{}", config.base_url, config.port);
	HttpServer::new(move ||
		App::new()
			.app_data(_config.clone())
			.app_data(links.clone())
			.service(get_shortened)
			.service(create_shortened)
	)
		.bind((config.base_url.as_str(), config.port))?
		.run()
		.await?;


	Ok(())
}
