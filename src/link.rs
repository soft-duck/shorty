use std::fmt::{Display, Formatter};

use chrono::Local;
use serde::Deserialize;
use sqlx::{Pool, Sqlite};
use tracing::debug;

use crate::{Config, generate_random_chars};
use crate::error::ShortyError;
use crate::util::time_now;

/// This struct holds configuration options for a custom link.
/// Optional fields are: `custom_id`, `max_uses`, and `valid_for`.
/// `valid_for` and `max_uses` default to 0, which means essentially infinite
#[derive(Debug, Clone, Deserialize)]
pub struct LinkConfig {
	/// The link that should be shortened.
	link: String,
	/// Custom ID for the link (like when you want a word instead of random jumble of chars).
	#[serde(alias = "id")]
	custom_id: Option<String>,
	/// How often the link may be used.
	#[serde(default)]
	max_uses: i64,
	/// How long the link is valid for in milliseconds.
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
	/// Creates a new link with a default configuration.
	/// Just creates a default config and calls [`Link::new_with_config`] with it.
	///
	/// # Errors
	///
	/// Errors if the underlying [`Link::new_with_config`] errors.
	pub async fn new(
		link: String,
		pool: &Pool<Sqlite>,
	) -> Result<Self, ShortyError> {
		let link_config = LinkConfig {
			link,
			custom_id: None,
			max_uses: 0, // unlimited uses
			valid_for: 1000 * 60 * 60 * 24, // 24 hours
		};


		Link::new_with_config(link_config, pool).await
	}

	/// Creates a new link according to the config provided.
	///
	/// # Errors
	///
	/// Returns an error if the link with the requested ID already exists.
	/// Also returns an error if there was a problem executing the SQL queries.
	pub async fn new_with_config(
		link_config: LinkConfig,
		pool: &Pool<Sqlite>,
	) -> Result<Self, ShortyError> {
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
				return Err(ShortyError::LinkConflict);
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

	#[must_use]
	pub fn is_invalid(&self) -> bool {
		let expired = self.valid_for != 0
			&& (Local::now().timestamp_millis() - self.created_at) > self.valid_for;

		let uses_valid = self.max_uses != 0 && self.invocations >= self.max_uses;


		expired || uses_valid
	}

	/// Retrieves a link from the database, if it exists.
	/// Calling this function also increments the invocations if the link exists in the database.
	async fn from_id(id: &str, pool: &Pool<Sqlite>) -> Result<Option<Self>, ShortyError> {
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

	/// Formats self, according to the options set in the config file.
	#[must_use]
	pub fn formatted(&self, config: &Config) -> String {
		format!("{}/{}", config.public_url, self.id)
	}
}

pub struct LinkStore {
	db: Pool<Sqlite>,
}

impl LinkStore {
	#[must_use]
	pub fn new(db: Pool<Sqlite>) -> Self {
		Self { db }
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

	/// Creates a shortened link with default settings.
	///
	/// # Errors
	///
	/// Returns an error if the underlying [`Link::new`] call fails.
	pub async fn create_link(&self, link: String) -> Result<Link, ShortyError> {
		Link::new(link, &self.db).await
	}

	/// Creates a shortened link with custom settings.
	///
	/// # Errors
	///
	/// Returns an error if the underlying [`Link::new_with_config`] call fails.
	pub async fn create_link_with_config(
		&self,
		link_config: LinkConfig,
	) -> Result<Link, ShortyError> {
		Link::new_with_config(link_config, &self.db).await
	}

	/// This function deletes stale links from the database.
	///
	/// # Errors
	///
	/// Errors if theres a problem executing the SQL queries.
	pub async fn clean(&self) -> Result<(), ShortyError> {
		debug!("Clearing stale links");

		let res = sqlx::query!("SELECT COUNT(*) AS num_before FROM links").fetch_one(&self.db).await?;
		let num_before = res.num_before;

		let now = time_now();
		sqlx::query!(
			r#"
			DELETE FROM links
			WHERE max_uses != 0 AND invocations > max_uses
			OR created_at + valid_for < $1
			"#,
			now
		)
			.execute(&self.db)
			.await?;

		let res = sqlx::query!("SELECT COUNT(*) AS num_after FROM links").fetch_one(&self.db).await?;
		let num_after = res.num_after;

		let delta = num_before - num_after;
		debug!("Size before cleaning: {num_before}. After cleaning: {num_after}. Removed elements: {delta}");


		Ok(())
	}
}
