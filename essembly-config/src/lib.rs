#![allow(non_camel_case_types)]

pub use serde_derive::{Deserialize, Serialize};
use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub server_config: ServerConfig,
    pub db_config: DBConfig,
}

impl Config {
    pub fn new() -> Config {
        let server = ServerConfig {
            state: String::new(),
        };

        let db_details: DB_Details = DB_Details {
            db: String::new(),
            ip: String::new(),
            protocol: String::new(),
        };

        let db = DBConfig {
            primary_db: db_details.clone(),
            remote_db: db_details.clone(),
        };

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
        let toml_config: Config = toml::from_str(&buffer).unwrap();

        toml_config
    }
}

pub fn default_config_file() -> PathBuf {
    let path = env::current_dir().unwrap();

    println!("Current Path: {:?}", path);

    env::var_os("SUSUDB_CONFIG")
        .unwrap_or_else(|| OsStr::new("./config.toml").to_os_string())
        .into()
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ServerConfig {
    pub state: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct DB_Details {
    pub db: String,
    pub ip: String,
    pub protocol: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct primary_db {
    primary_db: DB_Details,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct remote_db {
    remote_db: DB_Details,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct DBConfig {
    pub primary_db: DB_Details,
    pub remote_db: DB_Details,
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
            "primary-db = { db = \"sled\", ip = \"localhost\", protocol = \"grpc\" }\n",
            "remote-db = { db = \"sled\", ip = \"localhost\", protocol = \"grpc\" }",
        );

        let list: Config = toml::from_str(&config).unwrap();
        assert_ne!(list.db_config.primary_db.db, String::new());
    }
}
