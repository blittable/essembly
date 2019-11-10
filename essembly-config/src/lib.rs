#![allow(non_camel_case_types)]

pub use serde_derive::{Deserialize, Serialize};
use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub traffic_cop: TrafficCop,
    pub cli: CLI,
    pub db: DB,
    pub api: API
}

impl Config {
    pub fn new() -> Config  {

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
            ip: String::new(),
            port: String::new(),
            logging: String::new(),
        };

        let cli_secondary: CliDetails = CliDetails {
            ip: String::new(),
            port: String::new(),
            logging: String::new(),
        };

        let _cli: CLI= CLI {
            primary: cli_primary,
            secondary: cli_secondary,
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
        let db_primary: DbDetails = DbDetails {
            db_type: String::new(),
            ip: String::new(),
            port: String::new(),
            logging: String::new(),
        };

        let db_secondary: DbDetails = DbDetails {
            db_type: String::new(),
            ip: String::new(),
            port: String::new(),
            logging: String::new(),
        };

        let _db: DB= DB {
            primary: db_primary,
            secondary: db_secondary,
        };

        Config {
            traffic_cop: _traffic_cop,
            cli: _cli,
            db: _db,
            api: _api,
        }
    }

    pub fn load(self) -> Self {
        let config_source = get_default_config_file();

        dbg!("CONFIG SRC");
        dbg!(&config_source);

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
    
    let env = env::var_os("ESSEMBLY_CONFIG");
    dbg!("GETTING");
    dbg!(env);

    env::var_os("ESSEMBLY_CONFIG")
        .unwrap_or_else(|| OsStr::new("config.toml").to_os_string())
        .into()
}

#[serde(rename_all = "kebab-case")]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TrafficCop{
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
pub struct API{
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
pub struct DB{
    pub primary: DbDetails,
    pub secondary: DbDetails,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct DbDetails {
    pub db_type: String,
    pub ip: String,
    pub port: String,
    pub logging: String,
}

#[serde(rename_all = "kebab-case")]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CLI{
    pub primary: CliDetails,
    pub secondary: CliDetails,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct CliDetails {
    pub ip: String,
    pub port: String,
    pub logging: String,
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
            "primary = { ip = \"localhost\", port = \"2234\", logging = \"trace\" }\n",
            "secondary = { ip = \"localhost\", port = \"2234\", logging = \"trace\" }\n",
            "[api]\n",
            "primary = { ip = \"localhost\", port = \"2234\", logging = \"trace\" }\n",
            "secondary = { ip = \"localhost\", port = \"2234\", logging = \"trace\" }\n",
            "[db]\n",
            "primary = { db-type = \"sled\", ip = \"localhost\", port = \"2234\", logging = \"trace\" }\n",
            "secondary = { db-type = \"sled\", ip = \"222.222.222.2\", port = \"2234\", logging = \"trace\" }\n",
            "[logger]\n",
            "primary = { ip = \"localhost\", port = \"2234\"}\n",
        );

        let test_config: Config = toml::from_str(&config).unwrap();
        assert_ne!(test_config.traffic_cop.primary, test_config.traffic_cop.secondary);
        assert_ne!(test_config.db.primary, test_config.db.secondary);
    }


    #[test]
    fn test_config_load() {

        let path = env::current_dir().unwrap();

        dbg!("The current directory is:"); 
        dbg!(path.display());

        let test_config: Config = Config::new().load_from_file("src/config/test_config.toml".to_string());
        dbg!("Loaded:");
        dbg!(&test_config);

        assert_ne!(test_config.traffic_cop.primary, test_config.traffic_cop.secondary);
        assert_ne!(test_config.db.primary, test_config.db.secondary);
    }


    #[test]
    fn test_config_env_variable() {

        let path = env::current_dir().unwrap();

        let env_config_key = "ESSEMBLY_CONFIG";
        env::set_var(env_config_key, "../config.toml");

        let test_config: Config = Config::new().load(); 

        dbg!(&test_config);

    let test_config: Config = Config::new().load(); 

        // let config = concat!(
        //     "[traffic-cop]\n",
        //     "primary = { ip = \"localhost\", port = \"2888\" } \n",
        //     "secondary = { ip = \"222.222.222.2\", port = \"2888\" } \n",
        //     "[cli]\n",
        //     "primary = { ip = \"localhost\", port = \"2234\", logging = \"trace\" }\n",
        //     "secondary = { ip = \"localhost\", port = \"2234\", logging = \"trace\" }\n",
        //     "[api]\n",
        //     "primary = { ip = \"localhost\", port = \"2234\", logging = \"trace\" }\n",
        //     "secondary = { ip = \"localhost\", port = \"2234\", logging = \"trace\" }\n",
        //     "[db]\n",
        //     "primary = { db-type = \"sled\", ip = \"localhost\", port = \"2234\", logging = \"trace\" }\n",
        //     "secondary = { db-type = \"sled\", ip = \"222.222.222.2\", port = \"2234\", logging = \"trace\" }\n",
        //     "[logger]\n",
        //     "primary = { ip = \"localhost\", port = \"2234\"}\n",
        // );

        assert_ne!(test_config.traffic_cop.primary, test_config.traffic_cop.secondary);
        assert_ne!(test_config.db.primary, test_config.db.secondary);
}
}
