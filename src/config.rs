use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
	pub base_url: String,
	pub public_url: String,
	#[serde(default = "port_default")]
	pub port: u16,
}

impl Config {
	pub fn new(config: &str) -> Result<Self, Box<dyn std::error::Error>> {
		Ok(toml::from_str(config)?)
	}
}

const fn port_default() -> u16 {
	7999
}