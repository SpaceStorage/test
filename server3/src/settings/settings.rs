use std::env;
use config::{ConfigError, Config, File, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TLSSettings {
    pub certificate: String,
    pub key: String,
}

#[derive(Debug, Deserialize)]
pub struct HTTPserverSettings {
    pub addr: String,
    pub tls: TLSSettings,
    pub proto: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub log_level: u8,
    pub server: Vec<HTTPserverSettings>,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config_path = env::var("SPACESTORAGE_PATH").unwrap_or_else(|_| "/etc/spacestorage/spacestorage".to_string());
        let mut s = Config::default();

        s.merge(File::with_name("etc/defaults"))?;
        s.merge(File::with_name(&*config_path).required(false))?;
        s.merge(Environment::with_prefix("spacestorage"))?;

        println!("debug: {:?}", s.get::<Vec<HTTPserverSettings>>("server"));

        s.try_into()
    }
}
