use clap::arg_enum;
use core::str::FromStr;
use failure::{bail, Error, Fallible};
use std::path::PathBuf;
use structopt::StructOpt;

mod importop;
use importop::ImportOp;

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
        name = "intialize-local",
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
                if let operation = operation {
                    println!("{:?}", "acct");
                    Ok(())
                } else {
                    bail!("error in acct {}", "foo");
                }
            }
            Essembly::InitializeLocal {} => {
                println!("{:?}", "init local");
                Ok(())
            }
            Essembly::Import {
                ref operation,
                ref file,
            } => {
                if let file_handle = importop::ImportOp::open(file.to_string()) {
                    println!("Starting import... {:?}", file.to_string());
                    file_handle.await?;
                    Ok(())
                } else {
                    bail!("Incorrect file path or name: {:?}", file)
                }
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Essembly::from_args().run().await;
    Ok(())
}
