use failure::Fallible;
use regex::Regex;
pub use serde_derive::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;
use toml::from_str;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub server_config: ServerConfig,
    pub db_config: DBConfig,
}

impl Config {
    pub fn new() -> Config {
        let server = ServerConfig {
            state: "".to_string(),
        };

        let db = DBConfig { db: String::new() };

        Config {
            server_config: server,
            db_config: db,
        }
    }

    pub fn load(self) -> Self {
        let config_source = default_config_file();
        let mut f = File::open(config_source).unwrap();

        let mut buffer = String::new();

        f.read_to_string(&mut buffer).unwrap();
        let tom_config: Config = toml::from_str(&buffer).unwrap();

        tom_config
    }
}

pub fn default_config_file() -> PathBuf {
    env::var_os("SUSUDB_CONFIG")
        .unwrap_or_else(|| OsStr::new("config.toml").to_os_string())
        .into()
}

#[derive(Debug)]
pub struct BadConfig;

fn default_false() -> bool {
    false
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ServerConfig {
    pub state: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DBConfig {
    pub db: String,
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn test_config() {
        let config = concat!(
            "[server-config]\n",
            "state = \"grpc\"\n",
            "[db-config]\n",
            "state = \"running\"",
        );

        let list: Config = toml::from_str(&config).unwrap();
    }
}
