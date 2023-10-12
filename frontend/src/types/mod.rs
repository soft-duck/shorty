use serde::Deserialize;

pub mod duration;
pub mod error;
pub mod link_config;

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    pub public_url: String,
    pub max_link_length: usize,
    pub max_json_size: usize,
    pub max_custom_id_length: usize,
    pub default_max_uses: i64,
    pub default_valid_for: i64,
}
