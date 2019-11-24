#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use self::importer::Parser;
use self::importer::XLBRParser;
use clap::arg_enum;
use core::str::FromStr;
use essembly::config::Config;
use essembly::logging::*;
use essembly::store::*;
use failure::Fallible;
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;
use std::string::String;
use structopt::StructOpt;
use tracing::level_filters::*;
use tracing::*;

mod importer;

#[derive(Debug, Clone)]
pub struct Transaction(String);

#[derive(Debug, Clone)]
pub struct Item(String);

#[derive(Debug, Clone)]
pub struct Dest(PathBuf);

#[derive(Debug, Clone)]
pub struct ImportFile(String);

impl FromStr for ImportFile {
    type Err = std::string::ParseError;
    fn from_str(ex: &str) -> Result<Self, Self::Err> {
        let result = ex.to_string();
        Ok(ImportFile(result))
    }
}

impl FromStr for Item {
    type Err = std::string::ParseError;
    fn from_str(ex: &str) -> Result<Self, Self::Err> {
        let result = ex.to_string();
        Ok(Item(result))
    }
}

impl FromStr for Transaction {
    type Err = std::string::ParseError;
    fn from_str(ex: &str) -> Result<Self, Self::Err> {
        let result = ex.to_string();
        Ok(Transaction(result))
    }
}

#[derive(Debug, structopt_derive::StructOpt)]
#[allow(clippy::large_enum_variant)]
#[structopt(name = "essembly", about = "Essembly CLI")]
pub enum Essembly {
    #[structopt(
        name = "initialize-local",
        about = "Initialize a local instance of essembly"
    )]
    InitializeLocal,
    #[structopt(name = "acct", about = "run an accouting operation")]
    Acct {
        #[structopt(possible_values = &TransactionactionTypes::variants(), case_insensitive = true)]
        operation: TransactionactionTypes,
        #[structopt(name = "data", about = "a formed acct operation")]
        data: Transaction,
    },
    #[structopt(name = "inventory", about = "run an inventory operation")]
    Inventory {
        #[structopt(possible_values = &InventoryOperations::variants(), case_insensitive = true)]
        operation: InventoryOperations,
        #[structopt(name = "data", about = "data for the operation")]
        data: Transaction,
    },
    #[structopt(name = "import", about = "import data")]
    Import {
        #[structopt(possible_values = &ImportTypes::variants(), case_insensitive = true)]
        operation: ImportOperations,
        #[structopt(name = "file", about = "file to import")]
        file: String,
    },
}

impl Essembly {
    pub async fn run(&self) -> Fallible<()> {
        match *self {
            Essembly::Inventory {
                ref operation,
                ref data,
            } => {
                println!("{:?}", "inv");
                Ok(())
            }
            Essembly::Acct {
                ref operation,
                ref data,
            } => {
                println!("{:?}", "acct");
                Ok(())
            }
            Essembly::InitializeLocal {} => {
                println!("{:?}", "init local");
                Ok(())
            }
            Essembly::Import {
                ref operation,
                ref file,
            } => {
                let contents = tokio::fs::read(file).await?;
                let i: XLBRParser = Parser::new();
                i.parse(String::from_utf8(contents).unwrap());
                Ok(())
            }
        }
    }
}

#[derive(Debug, StructOpt)]
#[allow(clippy::large_enum_variant)]
struct Opts {
    #[structopt(possible_values = &TraceLevels::variants(), case_insensitive = true)]
    #[structopt(short = "t", help = "set the trace level")]
    trace: Option<TraceLevels>,
    #[structopt(flatten)]
    essmbly: Essembly,
}

//Debug
arg_enum! {
    #[derive(Debug)]
    pub enum TraceLevels {  TRACE,
    DEBUG,
    INFO,
    }
}

//Accounting
arg_enum! {
    #[derive(Debug)]
        pub enum TransactionactionTypes {  Post,
        Debit,
        Credit
    }
}

arg_enum! {
    #[derive(Debug)]
    pub enum InventoryOperations {  AddNewItem,
    DeleteItem,
    DecrementItemCount,
    IncrementItemCount,
}
}

//Import
arg_enum! {
    #[derive(Debug)]
    pub enum ImportOperations {
        Xblr,
        Json,
        Csv,
    }
}

arg_enum! {
    #[derive(Debug)]
    pub enum ImportTypes {
        Xblr,
        Json,
        Csv
    }
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
enum INIT_CASES {
    FILE_DOES_NOT_EXIST,
    CONFIG_ERROR,
}

fn initialize_configuration(case: INIT_CASES, errant_parameters: Vec<String>) -> bool {
    match case {
        INIT_CASES::FILE_DOES_NOT_EXIST => {
            println!("\n");
            warn!(
                "\nThe database file in the config.toml does not exist: {:?}",
                errant_parameters
            );
            info!("\nDo you want to create it now? (y/n)");

            let mut s = String::new();
            print!(": ");
            let _ = stdout().flush();

            stdin()
                .read_line(&mut s)
                .expect("incorrect choice");
            if let Some('\n') = s.chars().next_back() {
                s.pop();
            }
            if let Some('\r') = s.chars().next_back() {
                s.pop();
            }

            if s.eq_ignore_ascii_case("y")
            {
                println!("Creating database file.");
                println!("\n");
                return true;
            }
            else
            {
                return false;
            }
        },
        INIT_CASES::CONFIG_ERROR => {
            warn!(
                "The config.toml was unreadable or contained an error: {:?}",
                case
            );
                return false;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    //Initialize the client with its configuration
    let config = &Config::new().load();
    let primary = &config.cli.details;

    //logging
    let s = essembly::logging::trace::EssemblySubscriber::new(4);
    tracing::subscriber::set_global_default(s).unwrap();

    info!("cli runtime started");
    debug!("cli config: {:?}", config.cli);
    debug!("api config: {:?}", config.cli);

    //Validation - Should this be done here or 100% in essembly-config?
    if config.cli.details.direct_to_db {
        debug!(
            "direct to database is: {:?}",
            config.cli.details.direct_to_db
        );

        trace!("direct to database file path: {:?}", config.db.local);
        debug!("database exists: {:?}", config.db_file_exists());

        if !config.db_file_exists() {
            let create = initialize_configuration(
                INIT_CASES::FILE_DOES_NOT_EXIST,
                vec![config.db.local.path.clone(), config.db.local.file.clone()],
            );

            if create {
                let mut db = essembly::store::StoreBuilder::new();

                db.path = Box::new(config.db.local.path.clone());
                db.file = Box::new(config.db.local.file.clone());
                db.db_type = essembly::store::Supported_Databases::Sqlite;

                db.initialize();
            }
        }
    }

    Essembly::from_args().run().await?;

    Ok(())
}
