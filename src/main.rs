use std::error::Error;
use std::io::Read;
use std::time::Duration;

use actix_web::{App, get, HttpRequest, HttpResponse, HttpServer, post, Responder, web};
use sqlx::sqlite::SqlitePoolOptions;
use tracing::{debug, error, info, instrument, Level};
use tracing_subscriber::EnvFilter;

use crate::config::Config;
use crate::link::link::LinkStore;
use crate::util::{generate_random_chars, check_url_http, uri_to_url};

mod util;
mod link;
mod config;

const CLEAN_SLEEP_DURATION: Duration = Duration::from_secs(60 * 60);


#[get("/{shortened_url:.*}")]
#[instrument(skip_all)]
async fn get_shortened(params: web::Path<String>, link_store: web::Data<LinkStore>) -> Result<impl Responder, Box<dyn std::error::Error>> {
	let shortened_url = params.into_inner();
	debug!("Got request for {shortened_url}");

	Ok(
		if let Some(link) = link_store.get(shortened_url.as_str()).await {
			info!("Return url for {shortened_url} is {link}");
			HttpResponse::TemporaryRedirect()
				.append_header(("Location", link.redirect_to.as_str()))
				.finish()
		} else {
			HttpResponse::NotFound().finish()
		}
	)
}

#[post("/{url:.*}")]
#[instrument(skip_all)]
async fn create_shortened(
	req: HttpRequest,
	link_store: web::Data<LinkStore>,
	config: web::Data<Config>
) -> Result<impl Responder, Box<dyn std::error::Error>> {
	let uri = req.uri();
	info!("URI is {uri}");

	let url = uri_to_url(uri);
	let url = check_url_http(url);

	let link = link_store.create_link(url).await?;
	let shortened_url = format!("{}/{}", config.public_url, link.id);
	info!("Shortening URL {} to {}", link.redirect_to, shortened_url);


	Ok(HttpResponse::Ok().body(shortened_url))
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
		let file = std::fs::File::open("./config.toml");
		let mut file = match file {
			Ok(file) => file,
			Err(why) => {
				error!("Error opening the config file: {why}");
				std::process::exit(1);
			}
		};

		let mut content = String::new();
		file.read_to_string(&mut content)?;

		config = Config::new(content.as_str())?;
	}

	let config = web::Data::new(config);
	let _config = config.clone();

	let pool = SqlitePoolOptions::new()
		.max_connections(5)
		.min_connections(1)
		.max_lifetime(Some(Duration::from_secs(60 * 60)))
		.connect(config.database_url.as_str())
		.await?;

	sqlx::migrate!()
		.run(&pool)
		.await?;

	let links = web::Data::new(LinkStore::new(pool.clone()));
	let _links = links.clone();

	tokio::task::spawn(async move {
		loop {
			_links.clean().await;
			tokio::time::sleep(CLEAN_SLEEP_DURATION).await;
		}
	});

	let pool = web::Data::new(pool);
	info!("Starting server at {}:{}", config.base_url, config.port);
	HttpServer::new(move ||
		App::new()
			.app_data(_config.clone())
			.app_data(links.clone())
			.app_data(pool.clone())
			.service(get_shortened)
			.service(create_shortened)
	)
		.bind((config.base_url.as_str(), config.port))?
		.run()
		.await?;


	Ok(())
}
