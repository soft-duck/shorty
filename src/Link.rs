use std::fmt::{Display, Formatter};
use std::sync::atomic::AtomicI32;

pub struct Link {
	id: String,
	redirect_to: String,
	uses_limited: bool,
	uses_remaining: AtomicI32,
}

impl Link {
	pub fn new(link: String) -> Self {
		Self {
			id: "".to_string(),
			redirect_to: "".to_string(),
			uses_limited: false,
			uses_remaining: Default::default()
		}
	}
}

