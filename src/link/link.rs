use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, HttpResponseBuilder, ResponseError};
use actix_web::body::BoxBody;

use chrono::Local;
use serde::Deserialize;
use sqlx::{Pool, Sqlite};
use tokio::sync::RwLock;
use tracing::debug;
use thiserror::Error;

use crate::{Config, generate_random_chars};
use crate::util::time_now;

#[derive(Debug, Error)]
pub enum LinkError {
	#[error("Link with provided ID already exists")]
	Conflict,
	#[error(transparent)]
	Database(#[from] sqlx::Error),
	#[error(transparent)]
	Other(#[from] anyhow::Error),
}

/// This struct holds configuration options for a custom link.
/// Optional fields are: `custom_id`, `max_uses`, and `valid_for`.
/// `valid_for` and `max_uses` default to 0, which means essentially infinite
#[derive(Debug, Clone, Deserialize)]
pub struct LinkConfig {
	link: String,
	#[serde(alias = "id")]
	custom_id: Option<String>,
	#[serde(default)]
	max_uses: i64,
	#[serde(default)]
	valid_for: i64,
}

/// Struct representing a (shortened) Link.
/// All timestamps are in milliseconds.
#[derive(Debug, Clone)]
pub struct Link {
	pub id: String,
	pub redirect_to: String,
	max_uses: i64,
	invocations: i64,
	created_at: i64,
	valid_for: i64,
}

impl Display for Link {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.redirect_to)
	}
}

impl Link {
	/// Creates a new link. Also inserts it into the database.
	async fn new(
		link: String,
		pool: &Pool<Sqlite>,
	) -> Result<Self, LinkError> {
		let link_config = LinkConfig {
			link,
			custom_id: None,
			max_uses: 0, // unlimited uses
			valid_for: 1000 * 60 * 60 * 24, // 24 hours
		};

		Link::new_with_config(link_config, pool).await
	}

	pub async fn new_with_config(
		link_config: LinkConfig,
		pool: &Pool<Sqlite>,
	) -> Result<Self, LinkError> {
		let id = if let Some(id) = link_config.custom_id {
			id
		} else {
			generate_random_chars()
		};
		let redirect_to = link_config.link;
		let max_uses = link_config.max_uses;
		let invocations = 0;
		let created_at = time_now();
		let valid_for = link_config.valid_for;

		// If a link with the same ID exists already, return a conflict error.
		let existing_opt = Link::from_id(id.as_str(), pool).await?;
		if let Some(link) = existing_opt {
			if !link.is_invalid() {
				return Err(LinkError::Conflict);
			}
		}

		// We checked if the link exists already and is valid.
		// If it exists it has to be stale and can be replaced.
		sqlx::query!(
			r#"
				INSERT OR REPLACE INTO links
				VALUES ($1, $2, $3, $4, $5, $6)
			"#,
			id,
			redirect_to,
			max_uses,
			invocations,
			created_at,
			valid_for
		)
			.execute(pool)
			.await?;

		Ok(Self {
			id,
			redirect_to,
			max_uses,
			invocations,
			created_at,
			valid_for,
		})
	}

	pub fn is_invalid(&self) -> bool {
		let expired = self.valid_for != 0
			&& (Local::now().timestamp_millis() - self.created_at) > self.valid_for;

		let uses_valid = self.max_uses != 0 && self.invocations >= self.max_uses;


		expired || uses_valid
	}

	/// Retrieves a link from the database, if it exists.
	/// Calling this function also increments the invocations if the link exists in the database.
	async fn from_id(id: &str, pool: &Pool<Sqlite>) -> Result<Option<Self>, LinkError> {
		let link = sqlx::query_as!(
			Self,
			r#"
			SELECT * FROM links
			WHERE id = $1;
			UPDATE links
			SET invocations = invocations + 1
			WHERE id = $2;
			"#,
			id,
			id
		)
			.fetch_optional(pool)
			.await?;


		Ok(link)
	}

	pub fn formatted(&self, config: &Config) -> String {
		format!("{}/{}", config.public_url, self.id)
	}
}

pub struct LinkStore {
	links: RwLock<HashMap<String, Link>>,
	db: Pool<Sqlite>,
}

impl LinkStore {
	pub fn new(db: Pool<Sqlite>) -> Self {
		Self {
			links: RwLock::new(HashMap::new()),
			db,
		}
	}

	/// Retrieves a link with the provided ID, if it exists.
	pub async fn get(&self, id: &str) -> Option<Link> {
		let link = Link::from_id(id, &self.db).await;

		if let Ok(Some(link)) = link {
			if !link.is_invalid() {
				return Some(link);
			}

			debug!("{} got requested but is expired.", link.id);
		}

		None
	}

	pub async fn create_link(&self, link: String) -> Result<Link, LinkError> {
		Link::new(link, &self.db).await
	}

	pub async fn create_link_with_config(
		&self,
		link_config: LinkConfig,
	) -> Result<Link, LinkError> {
		Link::new_with_config(link_config, &self.db).await
	}

	pub async fn clean(&self) {
		debug!("Clearing stale links");
		let mut links = self.links.write().await;
		let num_before = links.len();

		links.retain(|_, link| !link.is_invalid());
		let num_after = links.len();
		let delta = num_before - num_after;
		debug!("Size before cleaning: {num_before}. After cleaning: {num_after}. Removed elements: {delta}");
	}
}

impl ResponseError for LinkError {
	fn status_code(&self) -> StatusCode {
		match self {
			LinkError::Conflict => StatusCode::CONFLICT,
			_ => StatusCode::INTERNAL_SERVER_ERROR,
		}
	}

	fn error_response(&self) -> HttpResponse<BoxBody> {
		HttpResponseBuilder::new(self.status_code())
			.body(self.to_string())
	}
}