use serde_derive::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: usize,
}

pub fn parse(path: &str) -> Config {
    let content = String::from_utf8(fs::read(path).expect("Unable to load the config file"))
        .expect("Unable to read the config file");
    toml::from_str::<Config>(&content).expect("Unable to parse the config file")
}
