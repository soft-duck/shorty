use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use chrono::Local;
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
	pub fn new(link: String) -> Self {
		Self {
			id: generate_random_chars(),
			redirect_to: link,
			max_uses: 0, //unlimited uses
			invocations: 0,
			created_at: Local::now().timestamp_millis(),
			valid_for: 1000 * 60 * 60 * 24, //24 hours
		}
	}

	pub fn is_invalid(&self) -> bool {
		self.invocations > self.max_uses
			|| (Local::now().timestamp_millis() - self.created_at) > self.valid_for
	}
}

pub struct LinkStore {
	links: RwLock<HashMap<String, Link>>,
}

impl LinkStore {
	pub fn new() -> Self {
		Self {
			links: RwLock::new(HashMap::new())
		}
	}

	pub async fn get(&self, id: &str) -> Option<Link> {
		let link;
		{
			let links = self.links.read().await;
			let link_opt = links.get(id);

			if link_opt.is_none() {
				return None;
			}

			link = link_opt.unwrap().clone();
		}

		if link.is_invalid() {
			debug!("{} got requested but is expired.", link.id);
			return None;
		}


		Some(link)
	}

	pub async fn insert(&self, link: Link) {
		let mut links = self.links.write().await;
		links.insert(link.id.clone(), link);
	}

	pub async fn clean(&self) {
		debug!("Clearing stale links");
		let mut links = self.links.write().await;
		links.retain(|_, link| !link.is_invalid())
	}
}
