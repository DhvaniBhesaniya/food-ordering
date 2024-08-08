use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub mongodb_url: String,
    pub port: u16,
    pub jwt_secret: String,
    pub stripe_secret_key: String
}

impl Config {
    pub fn from_env() -> Self {
        let config_file = fs::read_to_string("src/config/config_env.json").expect("config_env.json not found");
        serde_json::from_str(&config_file).expect("Invalid config format")
    }
}
