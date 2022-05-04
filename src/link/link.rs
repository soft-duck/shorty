use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

use chrono::Local;
use sqlx::{Pool, Sqlite};
use tokio::sync::RwLock;
use tracing::debug;

use crate::generate_random_chars;

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
	async fn new(link: String, pool: &Pool<Sqlite>) -> Result<Self, Box<dyn Error>> {
		let new_link = Self {
			id: generate_random_chars(),
			redirect_to: link,
			max_uses: 0, //unlimited uses
			invocations: 0,
			created_at: Local::now().timestamp_millis(),
			valid_for: 1000 * 60 * 60 * 24, //24 hours
		};

		let id = &new_link.id;
		let redirect_to = &new_link.redirect_to;

		sqlx::query!(
			r#"
			INSERT INTO links
			VALUES ($1, $2, $3, $4, $5, $6)
			"#,
			id,
			redirect_to,
			0,
			new_link.invocations,
			new_link.created_at,
			new_link.valid_for
		)
			.execute(pool)
			.await?;


		Ok(new_link)
	}

	pub fn is_invalid(&self) -> bool {
		self.invocations > self.max_uses
			|| (Local::now().timestamp_millis() - self.created_at) > self.valid_for
	}

	/// Retrieves a link from the database, if it exists.
	/// Calling this function also increments the invocations if the link exists in the database.
	async fn from_id(id: &str, pool: &Pool<Sqlite>) -> Result<Option<Self>, Box<dyn Error>> {
		let link = sqlx::query_as!(
			Self,
			r#"
			SELECT * FROM links
			WHERE id = $1
			"#,
			id
		)
			.fetch_optional(pool)
			.await?;


		Ok(link)
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
			db
		}
	}

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

	pub async fn create_link(&self, link: String) -> Result<Link, Box<dyn Error>> {
		Link::new(link, &self.db).await
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
