use serde::Deserialize;

pub const SAMPLE_CONFIG: &str = include_str!("../config.toml.sample");

#[derive(Deserialize)]
pub struct Config {
	pub listen_url: String,
	pub public_url: String,
	#[serde(default = "port_default")]
	pub port: u16,
	pub database_url: String
}

impl Config {
	/// # Errors
	/// Errors when the config couldn't be deserialized.
	pub fn new(config: &str) -> Result<Self, toml::de::Error> {
		toml::from_str(config)
	}
}

const fn port_default() -> u16 {
	7999
}