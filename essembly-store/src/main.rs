///! database service abstracted over different database types
#[allow(dead_code)]
#[allow(warnings)]
pub use serde_derive::{Deserialize, Serialize};

use essembly_config::*;
use essembly_interfaces::api::*;
use essembly_interfaces::registration::*;
use std::fs::File;

use tokio;

#[allow(dead_code)]
static DATABASE_NAME: &str = "essembly.db";

use sled::Db;

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
    let path = "./foo.db";
    let tree = Db::open(path)?;

    let options = tree.open_tree("options")?;
    let locations = tree.open_tree("locations")?;

    options.insert(b"save", "doo")?;

    let location = message.latlng.unwrap();
    let longitude = location.longitude.to_string();

    locations.insert(b"goo_lng", longitude.as_bytes())?;
    locations.insert(b"goo_lat", longitude.as_bytes())?;

    for node in &tree.tree_names() {
        println!("tree: {:?}", str::from_utf8(&node));
    }

    for x in locations.into_iter() {
        match x {
            Ok(e) => {
                println!("key: {:?}", str::from_utf8(&e.0));
                println!("value: {:?}", str::from_utf8(&e.1));
            }
            Err(_) => (),
        }
    }

    tree.flush()?;

    Ok(())
}

#[derive(Default)]
pub struct EssemblyServer;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Read config file
    let config: Config = Config::new().load();
    let db_type = config.db.primary.db_type;



    //let config_string = tokio::fs::read(config).await?;
    //let list: Config = toml::from_str(&tokio::fs::read(config)).unwrap();

    //Initialize DB
    //let mut cert_a = File::open("essembly-store/tls/server.pem")?;

    //let cert = tokio::fs::read("tls/server.pem").await?;
    //let key = tokio::fs::read("tls/server.key").await?;





    Ok(())
}
