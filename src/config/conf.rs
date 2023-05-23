use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct Conf {
    pub log: Log,
    pub server: Server,
    pub db: DB,
}

#[derive(Deserialize, Debug)]
pub struct Log {
    pub level: String,
    pub log_file: String,
    pub max_size: u32,
    pub max_backups: u32,
    pub max_age: u32,
    pub local_time: bool,
}

#[derive(Deserialize, Debug)]
pub struct Server {
    pub ip: String,
    pub port: u32,
    pub enable_cors: bool,
    pub enable_tls: bool,
    pub cert_file: String,
    pub key_file: String,
}

#[derive(Deserialize, Debug)]
pub struct DB {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u32,
    pub db_name: String,
    pub ssl_mode: String,
    pub max_idle_conns: u32,
    pub max_open_conns: u32,
    pub debug: bool,
    pub log_level: u32,
}

impl Conf {
    pub fn parse(path: &Path) -> Result<Self, ConfigError> {
        let c = Config::builder()
            .add_source(File::from(path))
            .add_source(Environment::with_prefix("AC"))
            .build()?;
        c.try_deserialize()
    }
}
