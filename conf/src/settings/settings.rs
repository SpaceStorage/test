use std::env;
use config::{ConfigError, Config, File, Environment};

#[derive(Debug, Deserialize)]
struct HTTPserverSettings {
    addr: String,
    port: u32,
    tls: u8,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    log_level: u8,
    http_server: Vec<HTTPserverSettings>,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config_path = env::var("SPACESTORAGE_PATH").unwrap_or_else(|_| "/etc/spacestorage/spacestorage".to_string());
        let mut s = Config::default();

        s.merge(File::with_name("defaults"))?;
        s.merge(File::with_name(&*config_path))?;
        s.merge(Environment::with_prefix("spacestorage"))?;

        s.try_into()
    }
}
