#![warn(rust_2018_idioms)]
#[allow(warnings)]

mod susu_error;
mod db;

use crate::susu_error::{Error, ErrorCode, Result};

use std::path::PathBuf;

use tracing::{debug, log, instrument, Subscriber};
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use tracing_attributes;
use tracing_futures;

use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};

use tokio;
use tokio::codec::{Framed, LinesCodec};
use tokio::net::TcpListener;

use futures::{SinkExt, StreamExt};

static DATABASE_NAME: &str = "susu.db";

struct Database {
    map: Mutex<HashMap<String, String>>,
}

/// Possible requests our clients can send us
enum Request {
    Get { key: String },
    Set { key: String, value: String },
}

/// Responses to the `Request` commands above
enum Response {
    Value {
        key: String,
        value: String,
    },
    Set {
        key: String,
        value: String,
        previous: Option<String>,
    },
    Error {
        msg: String,
    },
}


#[tokio::main]
async fn main() -> Result<()> {

    let mut db = db::SusuDB::open("susu.db".to_string()).await?;
    // Parse the address we're going to run this server on
    // and set up our TCP listener to accept connections.
    let addr = env::args().nth(1).unwrap_or("127.0.0.1:8080".to_string());

    let mut listener = TcpListener::bind(&addr).await.unwrap();
    println!("Listening on: {}", addr);

    // Create the shared state of this server that will be shared amongst all
    // clients. We populate the initial database and then create the `Database`
    // structure. Note the usage of `Arc` here which will be used to ensure that
    // each independently spawned client will have a reference to the in-memory
    // database.
    let mut initial_db = HashMap::new();
    initial_db.insert("foo".to_string(), "bar".to_string());
    let db = Arc::new(Database {
        map: Mutex::new(initial_db),
    });

    loop {
        match listener.accept().await {
            Ok((socket, _)) => {
                // After getting a new connection first we see a clone of the database
                // being created, which is creating a new reference for this connected
                // client to use.
                let db = db.clone();

                tokio::spawn(async move {
                    // Since our protocol is line-based we use `tokio_codecs`'s `LineCodec`
                    // to convert our stream of bytes, `socket`, into a `Stream` of lines
                    // as well as convert our line based responses into a stream of bytes.
                    let mut lines = Framed::new(socket, LinesCodec::new());

                    // Here for every line we get back from the `Framed` decoder,
                    // we parse the request, and if it's valid we generate a response
                    // based on the values in the database.

                    while let Some(result) = lines.next().await {
                        match result {
                            Ok(line) => {
                                let response = handle_request(&line, &db);

                                let response = response.serialize();

                                if let Err(e) = lines.send(response).await {
                                    println!("error on sending response; error = {:?}", e);
                                }
                            }
                            Err(e) => {
                                println!("error on decoding from socket; error = {:?}", e);
                            }
                        }
                    }

                    // The connection will be closed at this point as `lines.next()` has returned `None`.
                });
            }
            Err(e) => println!("error accepting socket; error = {:?}", e),
        }
    }
}

fn handle_request(line: &str, db: &Arc<Database>) -> Response {
    let request = match Request::parse(&line) {
        Ok(req) => req,
        Err(e) => return Response::Error { msg: e.to_string() },
    };

    let mut db = db.map.lock().unwrap();
    match request {
        Request::Get { key } => match db.get(&key) {
            Some(value) => Response::Value {
                key,
                value: value.clone(),
            },
            None => Response::Error {
                msg: format!("no key {}", key),
            },
        },
        Request::Set { key, value } => {
            let previous = db.insert(key.clone(), value.clone());
            Response::Set {
                key,
                value,
                previous,
            }
        }
    }
}

impl Request {
    fn parse(input: &str) -> Result<Request> {
        let mut parts = input.splitn(3, " ");
        match parts.next() {
            Some("GET") => {
                let key = match parts.next() {
                    Some(key) => key,
                    None => return Err(Error::new(ErrorCode::MessageFormat("Get must be followed by a key".to_string()))),
                };
                if parts.next().is_some() {
                    return Err(Error::new(ErrorCode::MessageFormat("Key must not have any following text.".to_string())));
                }
                Ok(Request::Get {
                    key: key.to_string(),
                })
            }
            Some("SET") => {
                let key = match parts.next() {
                    Some(key) => key,
                    None => return Err(Error::new(ErrorCode::MessageFormat("Set must be followed by a key".to_string()))),
                };
                let value = match parts.next() {
                    Some(value) => value,
                    None => return Err(Error::new(ErrorCode::MessageFormat("Set needs a value.".to_string()))),
                };
                Ok(Request::Set {
                    key: key.to_string(),
                    value: value.to_string(),
                })
            }
            Some(cmd) => return Err(Error::new(ErrorCode::MessageFormat("Command parse error".to_string()))),
            None => return Err(Error::new(ErrorCode::MessageFormat("No value in command.".to_string()))),
        }
    }
}

impl Response {
    fn serialize(&self) -> String {
        match *self {
            Response::Value { ref key, ref value } => format!("{} = {}", key, value),
            Response::Set {
                ref key,
                ref value,
                ref previous,
            } => format!("set {} = `{}`, previous: {:?}", key, value, previous),
            Response::Error { ref msg } => format!("error: {}", msg),
        }
    }
}
