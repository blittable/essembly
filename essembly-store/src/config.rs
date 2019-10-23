use crate::prelude::*;
use failure::Fallible;
use regex::Regex;
pub use serde_derive::{Deserialize, Serialize};
use serde_regex;
use std::collections::{HashMap, HashSet};
use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use toml::from_str;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub server_config: ServerConfig,
    pub db_config: DBConfig,
}

fn default_config_file() -> PathBuf {
    env::var_os("SUSUDB_CONFIG")
        .unwrap_or_else(|| OsStr::new("config.toml").to_os_string())
        .into()
}

#[derive(Debug, Fail)]
#[fail(display = "the configuration file has errors")]
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
    pub state: String,
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
