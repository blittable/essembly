use crate::prelude::*;
use serde_regex;
use failure::{Fail};
use toml::from_str;
use std::collections::{HashMap, HashSet};
use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn default_config_file() -> PathBuf {
    env::var_os("SUSUDB_CONFIG")
        .unwrap_or_else(|| OsStr::new("config.toml").to_os_string())
        .into()
}

#[derive(Debug, Fail)]
#[fail(display = "the configuration file has errors")]
pub struct BadConfig;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CrateConfig {
    #[serde(default = "default_false")]
    pub skip: bool,
    #[serde(default = "default_false")]
    pub skip_tests: bool,
    #[serde(default = "default_false")]
    pub quiet: bool,
    #[serde(default = "default_false")]
    pub broken: bool,
}

fn default_false() -> bool {
    false
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ServerConfig {
    pub bot_acl: BotACL,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BotACL {
    pub rust_teams: bool,
    pub github: Vec<String>,
}


#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub crates: HashMap<String, CrateConfig>,
    pub github_repos: HashMap<String, CrateConfig>,
    pub local_crates: HashMap<String, CrateConfig>,
    pub server: ServerConfig,
}

impl Config {
    pub fn load() -> Fallible<Self> {
        let buffer = Self::load_as_string(default_config_file())?;

        Ok(::toml::from_str(&buffer)?)
    }

    fn load_as_string(filename: PathBuf) -> Fallible<String> {
        let mut buffer = String::new();
        File::open(filename)?.read_to_string(&mut buffer)?;

        Ok(buffer)
    }

    fn check_all(filename: PathBuf) -> Fallible<()> {

        let buffer = Self::load_as_string(filename)?;
        // let mut has_errors = Self::check_for_dup_keys(&buffer).is_err();
        // let cfg: Self = ::toml::from_str(&buffer)?;
        // let db = crate::db::Database::open()?;
        // let crates = crate::crates::lists::get_crates(CrateSelect::Full, &db, &cfg)?;
        // has_errors |= cfg.check_for_missing_crates(&crates).is_err();
        // has_errors |= cfg.check_for_missing_repos(&crates).is_err();
        // if has_errors {
        //     Err(BadConfig.into())
        // } else {
        //     Ok(())
        // }
        Ok(())
    }
}


#[cfg(test)]
impl Default for Config {
    fn default() -> Self {
        Config {
            crates: HashMap::new(),
            github_repos: HashMap::new(),
            local_crates: HashMap::new(),
            server: ServerConfig {
                bot_acl: BotACL {
                    rust_teams: false,
                    github: vec![],
                },
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn test_config() {
        // A sample config file loaded from memory
        let config = concat!(
            "[server.bot-acl]\n",
            "rust-teams = false\n",
            "github = []\n",
            "remove = \"\"\n",
            "experiment-queued = \"\"\n",
            "experiment-completed = \"\"\n",
            "crates = []\n",
            "github-repos = []\n",
            "local-crates = []\n",
            "memory-limit = \"2G\"\n",
            "build-log-max-size = \"2M\"\n",
            "build-log-max-lines = 1000\n",
            "[crates]\n",
            "lazy_static = { skip = true }\n",
            "[github-repos]\n",
            "\"rust-lang/rust\" = { quiet = true }\n", // :(
            "[local-crates]\n"
        );

        let list: Config = toml::from_str(&config).unwrap();
    }
}
