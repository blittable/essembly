///! database service abstracted over different database types
#[allow(dead_code)]
#[allow(warnings)]
use rand::Rng;
use std::collections::HashMap;
pub use serde_derive::{Deserialize, Serialize};

//use essembly_logging::*;

#[allow(unused_imports)]
use tracing::{debug, error, event, info, span, trace, warn, Level};

use essembly_config::*;
use essembly_interfaces::registration::*;

use tokio;

use rayon::prelude::*;

#[allow(dead_code)]
static DATABASE_NAME: &str = "essembly.db";

use sled::Db;

//Initialize via the store trait
#[allow(dead_code)]
fn initialize_store() {

    //SQLite
    //
    //Sled
    //
    //Postgres
}


#[allow(dead_code)]
fn save_to_db(message: Address) -> Result<(), Box<dyn std::error::Error>> {

    let p= essembly_core::permissions::Permissions::new();


    let path = "/sled/essembly.db";
    let tree = Db::open(path)?;

    let sys= tree.open_tree("sys")?;
    //let options = tree.open_tree("options")?;
    //let locations = tree.open_tree("locations")?;

    let _result = sys.insert(&[p.sys, p.org, p.group, p.user], b"john");

    //println!("new user: {:?}", u);


    let _location = message.latlng.unwrap();
    let _longitude = _location.longitude.to_string();

    for node in &tree.tree_names() {
        println!("tree: {:?}", std::str::from_utf8(&node));
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

    use std::time::{Instant };
    let start = Instant::now();

    let config = sled::Config::default()
        .path("/sled/sled.db")
        .to_owned()
        .cache_capacity(51200000)
        .use_compression(false)
        .flush_every_ms(Some(1000000))
        .compression_factor(5)
        .snapshot_after_ops(1_000_000_000); //never

    let cache_vals: HashMap<[u8; 32], [u8; 32]> = HashMap::with_capacity(250_000); 

    println!("{:#?}", config);
    let db = config.open().unwrap();

    cache_vals.into_par_iter().for_each(|x| {
        //cacheVals.insert(x.key, x.value);
        //println!("KV on insert: {:?}, {:?}", x.0.to_vec(), x.1.to_vec());
        let _res = db.insert(x.0.to_vec(), x.1.to_vec());
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
    let db_type = &config.db.primary.db_type;

    let subscriber = tracing_subscriber::fmt::Subscriber::builder().finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("server started");
    trace!("tracing...");
    warn!("tracing...");

    let span = span!(
        Level::DEBUG,
        "starting",
        ip = ?config.cli.details.direct_to_db,
        log_level = ?config.cli.details.logging,
    );

    debug!("API configuration: {:?}", config.api);
    debug!("Store logging configuration: {:?}", config.logger);

    tracing::debug!("Store db directory: {:?}", config.db.primary);
    tracing::debug!("Store db type: {:?}", db_type);


    let _enter = span.enter();

    // Test 250K insert into essembly
    // load_test();

    //let config_string = tokio::fs::read(config).await?;
    //let list: Config = toml::from_str(&tokio::fs::read(config)).unwrap();

    //Initialize DBhttps://www.davekuhlman.org/generateDS.html
    //let mut cert_a = File::open("essembly-store/tls/server.pem")?;

    //let cert = tokio::fs::read("tls/server.pem").await?;
    //let key = tokio::fs::read("tls/server.key").await?;

    Ok(())
}
