use rusqlite::{Connection};
use tracing::{info,trace };

pub struct Store {}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq)]
pub enum Supported_Databases {
    Sqlite,
    Sled,
}

#[derive(Debug, Clone)]
pub struct StoreBuilder {
    pub path: Box<String>,
    pub file: Box<String>,
    pub db_type: Supported_Databases, //Add Enum
}

impl StoreBuilder {
    /// Create a new sandbox builder.
    pub fn new() -> Self {
        trace!("New storeBuilder invoked.");
        Self {
            path: Box::new(String::new()),
            file: Box::new(String::new()),
            db_type: Supported_Databases::Sqlite,
        }
    }

    pub fn initialize(self) -> Self {

        if self.db_type == Supported_Databases::Sqlite {
            info!("Creating sqlite database");

            let res = &format!("{}/{}", *self.path, *self.file);
            println!("{}", res);

            let _conn = Connection::open(res).unwrap();
        
           trace!("Sqlite database successfully created.");

        //     conn.execute(
        //         "create table if not exists cat_colors (
        //      id integer primary key,
        //      name text not null unique
        //  )",
        //         NO_PARAMS,
        //     );
        //     conn.execute(
        //         "create table if not exists cats (
        //      id integer primary key,
        //      name text not null,
        //      color_id integer not null references cat_colors(id)
        //  )",
        //         NO_PARAMS,
        //     )
        //     .unwrap()
        } else {
            println!("Init Sled");
        }
        self
    }
}
