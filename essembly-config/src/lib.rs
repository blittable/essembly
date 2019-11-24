#![allow(non_camel_case_types)]

pub use serde_derive::{Deserialize, Serialize};
use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub traffic_cop: TrafficCop,
    pub cli: CLI,
    pub db: DB,
    pub api: API,
    pub logger: Logger,
}

impl Config {
    pub fn new() -> Config {
        //Traffic Cop
        let traffic_cop_primary: TrafficCopDetails = TrafficCopDetails {
            ip: String::new(),
            port: String::new(),
        };

        let traffic_cop_secondary: TrafficCopDetails = TrafficCopDetails {
            ip: String::new(),
            port: String::new(),
        };

        let _traffic_cop: TrafficCop = TrafficCop {
            primary: traffic_cop_primary,
            secondary: traffic_cop_secondary,
        };

        //CLI
        let cli_primary: CliDetails = CliDetails {
            direct_to_db: false,
            logging: String::new(),
        };

        let _cli: CLI = CLI {
            details: cli_primary,
        };

        //API
        let api_primary: ApiDetails = ApiDetails {
            ip: String::new(),
            port: String::new(),
            logging: String::new(),
        };

        let api_secondary: ApiDetails = ApiDetails {
            ip: String::new(),
            port: String::new(),
            logging: String::new(),
        };

        let _api: API = API {
            primary: api_primary,
            secondary: api_secondary,
        };

        //DB
        let db_remote: DbRemote = DbRemote {
            db_type: String::new(),
            ip: String::new(),
            port: String::new(),
            logging: String::new(),
        };

        let db_local: DbLocal = DbLocal {
            db_type: String::new(),
            path: String::new(),
            file: String::new(),
            logging: String::new(),
        };

        let _db: DB = DB {
            remote: db_remote,
            local: db_local,
        };

        //Logger
        let _local: LoggerLocal = LoggerLocal {
            directory: String::new(),
        };

        let _remote: LoggerRemote = LoggerRemote {
            ip: String::new(),
            port: String::new(),
        };

        let _logger: Logger = Logger {
            local: _local,
            remote: _remote,
        };

        Config {
            traffic_cop: _traffic_cop,
            cli: _cli,
            db: _db,
            api: _api,
            logger: _logger,
        }
    }

    /// This method validates that the database in the configuration
    /// is reachable.  
    pub fn db_file_exists(&self) -> bool {

        let full_db_path_file = &format!("{}/{}", self.db.local.path, self.db.local.file);
        //let full_db_path_file = &self.path, &self.file;
        Path::new(full_db_path_file).exists()
    }

    pub fn load(self) -> Self {
        let config_source = get_default_config_file();

        println!("\n");
        println!("Loading configuration file from {:?}", &config_source);

        let mut f = File::open(&config_source).unwrap();

        let mut buffer = String::new();

        f.read_to_string(&mut buffer).unwrap();
        let toml_config: Config = toml::from_str(&buffer).unwrap();

        toml_config
    }

    pub fn load_from_file(self, file: String) -> Self {
        let p = PathBuf::from(file);

        let mut f = File::open(p).unwrap();
        let mut buffer = String::new();

        f.read_to_string(&mut buffer).unwrap();
        let toml_config: Config = toml::from_str(&buffer).unwrap();

        toml_config
    }
}

pub fn get_default_config_file() -> PathBuf {
    env::var_os("ESSEMBLY_CONFIG")
        .unwrap_or_else(|| OsStr::new("config.toml").to_os_string())
        .into()
}

#[serde(rename_all = "kebab-case")]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TrafficCop {
    pub primary: TrafficCopDetails,
    pub secondary: TrafficCopDetails,
}

//= TrafficCop_Details { ip = String::new(), port = String::new() },

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct TrafficCopDetails {
    pub ip: String,
    pub port: String,
}

#[serde(rename_all = "kebab-case")]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct API {
    pub primary: ApiDetails,
    pub secondary: ApiDetails,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct ApiDetails {
    pub ip: String,
    pub port: String,
    pub logging: String,
}

#[serde(rename_all = "kebab-case")]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DB {
    pub remote: DbRemote,
    pub local: DbLocal,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct DbRemote {
    pub db_type: String,
    pub ip: String,
    pub port: String,
    pub logging: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct DbLocal {
    pub db_type: String,
    pub path: String,
    pub file: String,
    pub logging: String,
}

#[serde(rename_all = "kebab-case")]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CLI {
    pub details: CliDetails,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct CliDetails {
    pub direct_to_db: bool,
    pub logging: String,
}

#[serde(rename_all = "kebab-case")]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Logger {
    pub local: LoggerLocal,
    pub remote: LoggerRemote,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct LoggerLocal {
    pub directory: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct LoggerRemote {
    pub ip: String,
    pub port: String,
}


#[cfg(test)]
mod tests {
    use super::Config;
    use std::env;

    #[test]
    fn test_config_parse() {
        let config = concat!(
            "[traffic-cop]\n",
            "primary = { ip = \"localhost\", port = \"2888\" } \n",
            "secondary = { ip = \"222.222.222.2\", port = \"2888\" } \n",
            "[cli]\n",
            "details = { direct_to_db = true, logging = \"trace\" }\n",
            "[api]\n",
            "primary = { ip = \"localhost\", port = \"2234\", logging = \"trace\" }\n",
            "secondary = { ip = \"localhost\", port = \"2234\", logging = \"trace\" }\n",
            "[db]\n",
            "remote = { db-type = \"sled\", ip = \"localhost\", port = \"2234\", logging = \"trace\" }\n",
            "local = { db-type = \"sled\", path = \"/db\", file = \"essembly.db\", logging = \"trace\" }\n",
            "[logger]\n",
            "local = { directory = \"/var/lib/vector\" }\n",
            "remote = { ip = \"localhost\", port = \"2234\" }\n",
        );

        let test_config: Config = toml::from_str(&config).unwrap();
        assert_ne!(
            test_config.traffic_cop.primary,
            test_config.traffic_cop.secondary
        );
    }

    #[test]
    fn test_config_load() {
        let path = env::current_dir().unwrap();

        dbg!("The current directory is:");
        dbg!(path.display());

        let test_config: Config =
            Config::new().load_from_file("src/config/test_config.toml".to_string());
        dbg!("Loaded:");
        dbg!(&test_config);

        assert_ne!(
            test_config.traffic_cop.primary,
            test_config.traffic_cop.secondary
        );
    }

    #[test]
    fn test_config_logger() {
        let test_config: Config =
            Config::new().load_from_file("src/config/test_config.toml".to_string());

        let test_local_logger_empty = super::LoggerLocal {
            directory: String::new(),
        };

        assert_ne!(test_config.logger.local, test_local_logger_empty);
    }

    #[test]
    fn test_config_env_variable() {
        let path = env::current_dir().unwrap();

        let env_config_key = "ESSEMBLY_CONFIG";
        env::set_var(env_config_key, "../config.toml");

        let test_config: Config = Config::new().load();

        dbg!(&test_config);

        let test_config: Config = Config::new().load();

        assert_ne!(
            test_config.traffic_cop.primary,
            test_config.traffic_cop.secondary
        );
    }
}
