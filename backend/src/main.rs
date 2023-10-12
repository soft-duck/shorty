#![allow(clippy::module_name_repetitions)]
#![allow(clippy::module_inception)]

use std::io::{Read, Write};
use std::path::Path;
use std::time::Duration;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use lazy_static::lazy_static;
use sqlx::migrate::MigrateDatabase;
use sqlx::Sqlite;
use sqlx::sqlite::{SqliteAutoVacuum, SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use tracing::{debug, error, info, Level};
use tracing_subscriber::EnvFilter;

use crate::config::Config;
use crate::config::SAMPLE_CONFIG;
use crate::endpoints::{api_docs, create_shortened, create_shortened_custom, get_config, get_favicon, get_shortened, index, serve_file};
use crate::error::ShortyError;
use crate::link::{LinkConfig, LinkStore};
use crate::util::ensure_http_prefix;

pub mod util;
pub mod link;
pub mod config;
pub mod error;
pub mod endpoints;

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

#[tokio::main]
async fn main() -> Result<(), ShortyError> {
	if Path::new(".env").exists() {
		dotenvy::dotenv()?;
	}

	let env_filter = EnvFilter::from_default_env()
		.add_directive(Level::INFO.into())
		.add_directive("shorty=debug".parse().unwrap());

	tracing_subscriber::fmt()
		.with_env_filter(env_filter)
		.with_line_number(true)
		.with_file(true)
		.init();

	if !Sqlite::database_exists(CONFIG.database_location.as_str()).await? {
		Sqlite::create_database(CONFIG.database_location.as_str()).await.expect("Couldn't create database file");
	}

	let db_options = SqliteConnectOptions::new()
		.auto_vacuum(SqliteAutoVacuum::Full)
		.journal_mode(SqliteJournalMode::Wal)
		.filename(CONFIG.database_location.as_str());

	let pool = SqlitePoolOptions::new()
		.max_connections(5)
		.min_connections(1)
		.max_lifetime(Some(Duration::from_secs(60 * 60)))
		.connect_with(db_options)
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
			.service(api_docs)
			.service(serve_file)
			.service(get_favicon)
			.service(get_shortened)
			.service(create_shortened_custom)
			.service(create_shortened)
	})
		.bind((CONFIG.listen_url.as_str(), CONFIG.port))
		.expect("Failed to bind port or listen address.")
		.run()
		.await
		.expect("Error running the HTTP server.");


	Ok(())
}
