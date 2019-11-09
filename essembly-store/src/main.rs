///! database service abstracted over different database types
#[allow(dead_code)]
#[allow(warnings)]
pub use serde_derive::{Deserialize, Serialize};

use essembly_interfaces::api::*;
use essembly_interfaces::registration::*;

use tokio;

#[allow(dead_code)]
static DATABASE_NAME: &str = "susu.db";

use sled::Db;

use essembly_config::config;
use std::collections::VecDeque;
use std::str;
use tonic::transport::{Identity, Server, ServerTlsConfig};
use tonic::{body::BoxBody, Request, Response, Status, Streaming};
use tower::Service;

type SusuResult<T> = Result<Response<T>, Status>;
type Stream = VecDeque<Result<SusuResponse, Status>>;

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
pub struct SusuServer;

#[tonic::async_trait]
impl server::Susu for SusuServer {
    async fn register_client(
        &self,
        request: Request<SusuClientRegistration>,
    ) -> SusuResult<SusuResponse> {
        let message = request.into_inner().address.unwrap();

        println!("received message: {:?}", message);

        match save_to_db(message.clone()) {
            Ok(result) => println!("Message save to DB {:?}", result),
            Err(err) => eprintln!("Error saving to the database. {:?}", err),
        }

        Ok(Response::new(SusuResponse {
            message: "Received Registration".to_string(),
        }))
    }

    type ServerStreamingSusuStream = Stream;

    async fn client_streaming_susu(
        &self,
        _: Request<Streaming<SusuRequest>>,
    ) -> SusuResult<SusuResponse> {
        Err(Status::unimplemented("not implemented"))
    }

    type BidirectionalStreamingSusuStream = Stream;

    async fn bidirectional_streaming_susu(
        &self,
        _: Request<Streaming<SusuRequest>>,
    ) -> SusuResult<Self::BidirectionalStreamingSusuStream> {
        Err(Status::unimplemented("not implemented"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Read config file
    let config: config::Config = config::Config::new().load();

    println!(
        "loaded Config with local db: {:?}",
        config.db_config.primary_db
    );
    println!(
        "loaded Config with remote db: {:?}",
        config.db_config.remote_db
    );

    //let config_string = tokio::fs::read(config).await?;
    //let list: Config = toml::from_str(&tokio::fs::read(config)).unwrap();

    //Initialize DB
    let cert = tokio::fs::read("tls/server.pem").await?;
    let key = tokio::fs::read("tls/server.key").await?;

    let identity = Identity::from_pem(cert, key);

    let addr = "127.0.0.1:50051".parse().unwrap();

    let server = SusuServer::default();

    Server::builder()
        .tls_config(ServerTlsConfig::with_rustls().identity(identity))
        .interceptor_fn(move |svc, req| {
            let auth_header = req.headers().get("authorization").clone();

            let authed = if let Some(auth_header) = auth_header {
                auth_header == "Bearer some-secret-token"
            } else {
                false
            };

            let fut = svc.call(req);

            async move {
                if authed {
                    fut.await
                } else {
                    // Cancel the inner future since we never await it
                    // the IO never gets registered.
                    drop(fut);

                    let res = http::Response::builder()
                        .header("grpc-status", "16")
                        .body(BoxBody::empty())
                        .unwrap();
                    Ok(res)
                }
            }
        })
        .clone()
        .add_service(server::SusuServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}
