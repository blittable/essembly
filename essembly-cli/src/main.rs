use clap::arg_enum;
use core::str::FromStr;
use std::path::PathBuf;
use structopt::clap::ArgGroup;
use structopt::StructOpt;

#[derive(Debug, Clone)]
pub struct Transaction(String);

#[derive(Debug, Clone)]
pub struct Item(String);

#[derive(Debug, Clone)]
pub struct Dest(PathBuf);

impl FromStr for Transaction {
    type Err = std::string::ParseError;
    fn from_str(ex: &str) -> Result<Self, Self::Err> {
        let result = ex.to_string();
        Ok(Transaction(result))
    }
}

impl FromStr for Item {
    type Err = std::string::ParseError;
    fn from_str(ex: &str) -> Result<Self, Self::Err> {
        let result = ex.to_string();
        Ok(Item(result))
    }
}

#[derive(Debug, structopt_derive::StructOpt)]
#[allow(clippy::large_enum_variant)]
#[structopt(name = "essembly", about = "CLI!")]
pub enum Essembly {
    #[structopt(
        name = "intialize-local",
        about = "Initialize a local instance of essembly"
    )]
    InitializeLocal,
    #[structopt(name = "acct", about = "run an accouting operation", group = acct_arg_group())]
    Acct {
        #[structopt(name = "ledger::value", short = "v", long = "value")]
        value: Transaction,
        #[structopt(possible_values = &TransactionactionTypes::variants(), case_insensitive = true)]
        operation: TransactionactionTypes,
    },
    #[structopt(name = "inventory", about = "run an inventory operation")]
    Inventory {
        #[structopt(possible_values = &InventoryOperations::variants(), case_insensitive = true)]
        operation: InventoryOperations,
        #[structopt(name = "data", about = "run an inventory operation")]
        data: Transaction,
    },
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

fn acct_arg_group() -> ArgGroup<'static> {
    ArgGroup::with_name("acct").required(true)
}

arg_enum! {
    #[derive(Debug)]
    pub enum TraceLevels {  TRACE,
    DEBUG,
    INFO,
    }
}

arg_enum! {
    #[derive(Debug)]
    pub enum TransactionactionTypes {  post,
    debit,
   credit
}
}

fn inv_arg_group() -> ArgGroup<'static> {
    ArgGroup::with_name("inv").required(true)
}

arg_enum! {
    #[derive(Debug)]
    pub enum InventoryOperations {  add_new_item,
    delete_item,
    decrement_item_count,
    increment_item_count,
}
}


fn main() {
    let opt = Opts::from_args();
    println!("{:?}", opt);
}
