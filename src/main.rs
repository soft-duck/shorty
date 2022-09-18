#![allow(clippy::module_name_repetitions)]
#![allow(clippy::module_inception)]

use std::io::{Read, Write};
use std::path::Path;
use std::time::Duration;
use actix_cors::Cors;

use actix_web::{App, get, HttpRequest, HttpResponse, HttpServer, post, Responder, web};
use lazy_static::lazy_static;
use sqlx::migrate::MigrateDatabase;
use sqlx::Sqlite;
use sqlx::sqlite::SqlitePoolOptions;
use tracing::{debug, error, info, Level};
use tracing_subscriber::EnvFilter;

use crate::config::Config;
use crate::config::SAMPLE_CONFIG;
use crate::error::ShortyError;
use crate::file_serving::endpoints::{api_docs, index, serve_file};
use crate::link::{LinkConfig, LinkStore};
use crate::util::{ensure_http_prefix, uri_to_url};

pub mod util;
pub mod link;
pub mod config;
pub mod error;
mod file_serving;

const CLEAN_SLEEP_DURATION: Duration = Duration::from_secs(60 * 60);

lazy_static! {
	static ref CONFIG: Config = {
		let config_location = std::env::var("SHORTY_CONFIG")
			.unwrap_or_else(|_| "./config.toml".to_owned());
		let path = Path::new(&config_location);

		if !path.exists() {
			let mut file = std::fs::File::create(path).expect("Failed to create sample config file");
			file.write_all(SAMPLE_CONFIG.as_bytes()).expect("Couldn't write the sample config file");

			error!(
				"You have to configure the config file. A sample config was created at {}",
				config_location
			);
			std::process::exit(1);
		}

		let mut file = std::fs::File::open(path).expect("Failed to open config file.");
		let mut content = String::new();
		file.read_to_string(&mut content).expect("Failed to read config file.");


		Config::new(content.as_str()).expect("Failed to parse config")
	};
}

#[get("/{shortened_url:.*}")]
async fn get_shortened(
	params: web::Path<String>,
	link_store: web::Data<LinkStore>,
) -> Result<impl Responder, ShortyError> {
	let shortened_url = params.into_inner();
	debug!("Got request for {shortened_url}");


	if let Some(link) = link_store.get(shortened_url.as_str()).await {
		info!("Return url for {shortened_url} is {link}");
		Ok(
			HttpResponse::TemporaryRedirect()
				.append_header(("Location", link.redirect_to.as_str()))
				.finish()
		)
	} else {
		Ok(HttpResponse::NotFound().finish())
	}
}

// The function is async because the actix-web macro requires it.
#[get("/config")]
#[allow(clippy::unused_async)]
async fn get_config() -> impl Responder {
	HttpResponse::Ok()
		.content_type("application/json; charset=utf-8")
		.body(CONFIG.json_string())
}

/// Creates a shortened link by taking the requested uri and turning it into a shortened link.
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

/// Custom shortened URL, configured via Json.
/// Also see [`LinkConfig`].
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


#[tokio::main]
async fn main() -> Result<(), ShortyError> {
	dotenv::dotenv()?;

	let env_filter = EnvFilter::from_default_env()
		.add_directive(Level::INFO.into())
		.add_directive("shorty=debug".parse().unwrap());

	tracing_subscriber::fmt()
		.with_env_filter(env_filter)
		.with_line_number(true)
		.with_file(true)
		.init();


	if !Sqlite::database_exists(CONFIG.database_url.as_str()).await? {
		Sqlite::create_database(CONFIG.database_url.as_str()).await.expect("Couldn't create database file");
	}

	let pool = SqlitePoolOptions::new()
		.max_connections(5)
		.min_connections(1)
		.max_lifetime(Some(Duration::from_secs(60 * 60)))
		.connect(CONFIG.database_url.as_str())
		.await?;

	sqlx::migrate!()
		.run(&pool)
		.await
		.expect("Failed db schema migration.");

	// Gracefully close the database connection(s) on CTRL+C
	let pool_clone = pool.clone();
	tokio::task::spawn(async move {
		tokio::signal::ctrl_c().await.expect("Error awaiting SIGINT.");
		info!("Received SIGINT, shutting down...");
		debug!("Closing Database pool.");
		pool_clone.close().await;
		debug!("Closed Database pool.");
	});

	let links = web::Data::new(LinkStore::new(pool.clone()));
	let links_clone = links.clone();

	tokio::task::spawn(async move {
		loop {
			if let Err(why) = links_clone.clean().await {
				error!("{why}");
			}
			tokio::time::sleep(CLEAN_SLEEP_DURATION).await;
		}
	});

	let pool = web::Data::new(pool);
	info!("Starting server at {}:{}", CONFIG.listen_url, CONFIG.port);

	HttpServer::new(move || {
		let json_config = web::JsonConfig::default()
			.limit(CONFIG.max_json_size);

		let cors = Cors::default()
			.allow_any_origin()
			.allowed_methods(vec!["GET", "POST"]);

		App::new()
			.wrap(cors)
			.app_data(json_config)
			.app_data(links.clone())
			.app_data(pool.clone())
			.service(get_config)
			.service(index)
			.service(serve_file)
			.service(get_favicon)
			.service(get_shortened)
			.service(create_shortened_custom)
			.service(create_shortened)
			.service(api_docs)
	})
		.bind((CONFIG.listen_url.as_str(), CONFIG.port))
		.expect("Failed to bind port or listen address.")
		.run()
		.await
		.expect("Error running the HTTP server.");


	Ok(())
}
