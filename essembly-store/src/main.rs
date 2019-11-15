///! database service abstracted over different database types
#[allow(dead_code)]
#[allow(warnings)]
use rand::Rng;
use std::collections::HashMap;
pub use serde_derive::{Deserialize, Serialize};
use tempfile::tempdir;


use essembly_core::permissions::*;
use essembly_config::*;
use essembly_interfaces::api::*;
use essembly_interfaces::registration::*;
use std::time::{Duration, Instant};

use std::fs::File;

use tokio;

use rayon::prelude::*;

#[allow(dead_code)]
static DATABASE_NAME: &str = "essembly.db";

use sled::Db;
use essembly_core::{ permissions, user };
use std::collections::VecDeque;
use std::str;

//Initialize via the store trait
fn initialize_store() {

    //SQLite
    //
    //Sled
    //
    //Postgres
}

fn save_to_db(message: Address) -> Result<(), Box<dyn std::error::Error>> {

    let p= essembly_core::permissions::Permissions::new();


    let path = "/sled/essembly.db";
    let tree = Db::open(path)?;

    let sys= tree.open_tree("sys")?;
    //let options = tree.open_tree("options")?;
    //let locations = tree.open_tree("locations")?;

    sys.insert(&[p.sys, p.org, p.group, p.user], b"john");

    //println!("new user: {:?}", u);


    let location = message.latlng.unwrap();
    let longitude = location.longitude.to_string();

    for node in &tree.tree_names() {
        println!("tree: {:?}", str::from_utf8(&node));
    }

    // for x in locations.into_iter() {
    //     match x {
    //         Ok(e) => {
    //             println!("key: {:?}", str::from_utf8(&e.0));
    //             println!("value: {:?}", str::from_utf8(&e.1));
    //         }
    //         Err(_) => (),
    //     }
    // }

    tree.flush()?;

    Ok(())
}

pub fn write_users() {

}

pub fn load_test() {
    let start = Instant::now();

    let config = sled::Config::default()
        .path("/sled/sled.db")
        .to_owned()
        .cache_capacity(51200000)
        .use_compression(false)
        .flush_every_ms(Some(1000000))
        .compression_factor(5)
        .snapshot_after_ops(1_000_000_000); //never

    let mut cacheVals: HashMap<[u8; 32], [u8; 32]> = HashMap::with_capacity(250_000); 

    println!("{:#?}", config);
    let db = config.open().unwrap();

    cacheVals.into_par_iter().for_each(|x| {
        //cacheVals.insert(x.key, x.value);
        //println!("KV on insert: {:?}, {:?}", x.0.to_vec(), x.1.to_vec());
        db.insert(x.0.to_vec(), x.1.to_vec());
    });

    let duration = start.elapsed();
    println!("250K insert time is: {:?}", duration);
}

#[derive(Default)]
pub struct EssemblyServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Read config file
    let config: Config = Config::new().load();
    let db_type = config.db.primary.db_type;

    load_test();

    //let config_string = tokio::fs::read(config).await?;
    //let list: Config = toml::from_str(&tokio::fs::read(config)).unwrap();

    //Initialize DB
    //let mut cert_a = File::open("essembly-store/tls/server.pem")?;

    //let cert = tokio::fs::read("tls/server.pem").await?;
    //let key = tokio::fs::read("tls/server.key").await?;

    Ok(())
}
