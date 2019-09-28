// Copyright 2019 John Douglas (@instransitvita) <blittable.bits@gmail.com>

use clap::arg_enum;
use core::str::FromStr;
use std::error::Error;
use std::path::PathBuf;
use structopt::clap::AppSettings;
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

// #[derive(StructOpt, Debug)]
// #[structopt(
//     name = "essembly",
//     about = "A cli (command line interface) for essembly.",
//     group = acct_arg_group(),
// )]
// struct Opt {
//     #[structopt(possible_values = &Modules::variants(), case_insensitive = true, group = "acct")]
//     module: Modules,

//     #[structopt(long = "post", group = "acct")]
//     post: Option<String>,

//     #[structopt(long = "balance", group = "acct")]
//     balance: Option<String>,

//     /// Needed parameter, the first on the command line.
//     #[structopt(help = "Input file")]
//     input: Option<String>,

//     /// specify the logging target: `--log
//     /// log.txt`). If a target is not specified, stdout is used.
//     #[structopt(
//         long = "log",
//         help = "Log file, stdout if no file, no logging if not present"
//     )]
//     #[allow(clippy::option_option)]
//     log: Option<Option<String>>,
// }

fn main() {
    let opt = Opts::from_args();
    println!("{:?}", opt);
}
