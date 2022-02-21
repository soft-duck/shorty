use std::collections::HashMap;
use std::error::Error;

use actix_web::{App, get, HttpRequest, HttpResponse, HttpServer, middleware, post, Responder, web};
use tokio::sync::RwLock;
use tracing::{info, Level, instrument};
use tracing_subscriber::EnvFilter;

use crate::util::{generate_random_chars, uri_to_url};

mod util;
mod link;

type LinkMap = RwLock<HashMap<String, String>>;

const BASE_URL: &str = "localhost:7999";

#[get("/{shortened_url:.*}")]
#[instrument(skip_all)]
async fn get_shortened(params: web::Path<String>, map: web::Data<LinkMap>) -> impl Responder {
	let shortened_url = params.into_inner();

	info!("Retrieving {shortened_url} from the map");
	let map = map.read().await;
	let redirect_url = map.get(shortened_url.as_str());


	if let Some(url) = redirect_url {
		info!("Return url for {shortened_url} is {url}");
		HttpResponse::TemporaryRedirect()
			.append_header(("Location", url.as_str()))
			.finish()
	} else {
		HttpResponse::NotFound().finish()
	}
}

#[post("/{url:.*}")]
#[instrument(skip_all)]
async fn create_shortened(req: HttpRequest, map: web::Data<LinkMap>) -> impl Responder {
	let uri = req.uri();
	info!("URI is {uri}");
	let url = uri_to_url(uri);
	let random_chars = generate_random_chars();
	let shortened_url = format!("http://{}/{}", BASE_URL, random_chars);
	info!("Shortening URL {url} to {shortened_url}");

	{
		let mut map = map.write().await;
		map.insert(random_chars, url);
	}


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

	let links: LinkMap = RwLock::new(HashMap::new());
	let links = web::Data::new(links);

	HttpServer::new(move ||
		App::new()
			.wrap(middleware::NormalizePath::new(middleware::TrailingSlash::MergeOnly))
			.app_data(links.clone())
			.service(get_shortened)
			.service(create_shortened)
	)
		.bind(("127.0.0.1", 7999))?
		.run()
		.await?;


	Ok(())
}
