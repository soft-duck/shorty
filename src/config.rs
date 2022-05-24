use serde::{Serialize, Deserialize};

pub const SAMPLE_CONFIG: &str = include_str!("../config.toml.sample");

#[derive(Serialize, Deserialize)]
pub struct Config {
	#[serde(default = "listen_url_default")]
	#[serde(skip_serializing)]
	pub listen_url: String,
	/// The public URL that gets used for shortened links.
	/// It is different from the listen_url if shorty is run behind a reverse proxy.
	pub public_url: String,
	/// The listen port.
	#[serde(default = "port_default")]
	#[serde(skip_serializing)]
	pub port: u16,
	/// The database connection String.
	#[serde(skip_serializing)]
	pub database_url: String,
	/// The maximum length a link should be allowed to have.
	#[serde(default = "max_link_length_default")]
	pub max_link_length: usize,
	/// The max size of accepted json.
	#[serde(default = "max_json_size_default")]
	pub max_json_size: usize,
	/// Default max uses for a link.
	#[serde(default = "max_uses_default")]
	pub default_max_uses: i64,
	/// Default duration a link is valid for.
	#[serde(default = "valid_for_duration_default")]
	pub default_valid_for: i64,
}

impl Config {
	/// # Errors
	/// Errors when the config couldn't be deserialized.
	pub fn new(config: &str) -> Result<Self, toml::de::Error> {
		toml::from_str(config)
	}

	pub fn json_string(&self) -> String {
		serde_json::to_string(self).unwrap()
	}
}

fn listen_url_default() -> String {
	"127.0.0.1".to_owned()
}

const fn port_default() -> u16 {
	7999
}

const fn max_link_length_default() -> usize {
	2_500
}

const fn max_json_size_default() -> usize {
	2_097_152 // 2 Mebibyte
}

// Link configuration default values

const fn max_uses_default() -> i64 {
	0 // unlimited uses
}

const fn valid_for_duration_default() -> i64 {
	1000 * 60 * 60 * 24 // 24 hours
}
