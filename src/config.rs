use serde::Deserialize;

pub const SAMPLE_CONFIG: &str = include_str!("../config.toml.sample");

#[derive(Deserialize)]
pub struct Config {
	#[serde(default = "listen_url_default")]
	pub listen_url: String,
	/// The public URL that gets used for shortened links.
	/// It is different from the listen_url if shorty is run behind a reverse proxy.
	pub public_url: String,
	/// The listen port.
	#[serde(default = "port_default")]
	pub port: u16,
	/// The database connection String.
	pub database_url: String,
	/// The maximum length a link should be allowed to have.
	#[serde(default = "max_link_length_default")]
	pub max_link_length: usize,
	/// The max size of accepted json.
	#[serde(default = "max_json_size_default")]
	pub max_json_size: usize,
}

impl Config {
	/// # Errors
	/// Errors when the config couldn't be deserialized.
	pub fn new(config: &str) -> Result<Self, toml::de::Error> {
		toml::from_str(config)
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
	2_097_152 // 2 Mebibytes
}
