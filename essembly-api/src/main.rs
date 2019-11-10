#![deny(rust_2018_idioms)]
#[allow(dead_code)]
#[allow(warnings)]
pub use serde_derive::{Deserialize, Serialize};

use essembly::interfaces::*;
use tracing;

#[allow(unused_imports)]
use tracing::{debug, error, event, info, span, warn, Level};
use tokio::prelude::Future;

use essembly::core::*;
use essembly::logging;

#[allow(dead_code)]
static DATABASE_NAME: &str = "essembly.db";

use essembly::interfaces::api::{EssemblyRequest, EssemblyResponse };
use essembly::config::{Config};

use std::collections::VecDeque;
use std::str;
use std::fs::File;
use tonic::{transport::{Server, Identity, Certificate, ServerTlsConfig},  Request, Response, Status, Streaming };

type EssemblyResult<T> = Result<Response<T>, Status>;
type Stream = VecDeque<Result<EssemblyResponse, Status>>;

fn save_to_db(message: registration::Address) -> Result<(), Box<dyn std::error::Error>> {
    debug!("started");
    warn!("started");
    info!("started");

    event!(Level::INFO, "Dogs and Cats");

    // #[cfg(debug)] {
    //     println!("Data file loaded at: {:?}", path);
    // }

    // #[cfg(debug)] {
    //     println!("Data file loaded at: {:?}", path);
    // }

    let location = message.latlng.unwrap();
    #[allow(dead_code)]
    #[allow(unused_variables)]
    let latitude = location.latitude.to_string();
    #[allow(dead_code)]
    #[allow(unused_variables)]
    let longitude = location.longitude.to_string();

    Ok(())
}

#[derive(Default)]
pub struct EssemblyServer;

#[tonic::async_trait]
impl api::server::Essembly for EssemblyServer {
    // async fn register_client(
    //     &self,
    //     request: Request<api::EssemblyClientRegistration>,
    // ) -> EssemblyResult<EssemblyResponse> {
    //     let message = request.into_inner().address.unwrap();

    //     println!("received message: {:?}", message);

    //     match save_to_db(message.clone()) {
    //         Ok(result) => println!("Message save to DB {:?}", result),
    //         Err(err) => eprintln!("Error saving to the database. {:?}", err),
    //     }

    //     Ok(Response::new(EssemblyResponse {
    //         message: "Received Registration".to_string(),
    //     }))
    // }


    async fn client_streaming_essembly(
        &self,
        _: Request<Streaming<EssemblyRequest>>,
    ) -> EssemblyResult<EssemblyResponse> {
        Err(Status::unimplemented("not implemented"))
    }


    async fn bidirectional_streaming_essembly(
        &self,
        _: Request<Streaming<EssemblyRequest>>,
    ) -> EssemblyResult<Self::BidirectionalStreamingEssemblyStream> {
        Err(Status::unimplemented("not implemented"))
    }

    type BidirectionalStreamingEssemblyStream = Stream;
    type ServerStreamingEssemblyStream = Stream;
}

async fn run_server() -> Result<(), Box<dyn std::error::Error>> {

     //Read config file
    let config: Config = Config::new().load();

    // println!(
    //     "loaded Config with local db: {:?}",
    //     config.db_config.primary_db
    // );

    // println!(
    //     "loaded Config with remote db: {:?}",
    //     config.db_config.remote_db
    // );

    //  let cert = File::open("essembly-api/tls/server.pem")?;
    // println!(
    //     "loaded Cert with remote db: {:?}",
    //     config.db_config.remote_db
    // );

    //  let key = File::open("essembly-api/tls/server.key")?;
    // println!(
    //     "loaded Cert with remote db: {:?}",
    //     config.db_config.remote_db
    // );

    //let cert = tokio::fs::read("essembly-api/tls/server.pem").await?;
    //let key = tokio::fs::read("essembly-api/tls/server.key").await?;
    let pem = std::fs::read("essembly-api/tls/server.pem")?;
    let key = &std::fs::read("essembly-api/tls/server.key")?;
    // .map(|data| {
    //     // do something with the contents of the file ...
    //     println!("foo.txt contains {} bytes", data.len());
    // }).map_err(|e| {
    //     // handle errors
    //     eprintln!("IO error: {:?}", e);
    // });


    let identity = Identity::from_pem(pem, key);


    // // let subscriber = FmtSubscriber::builder()
    // //     .with_max_level(Level::TRACE)
    // //     .finish();

    let client_ca_cert = tokio::fs::read("essembly-api/tls/client_ca.pem").await?;
    let client_ca_cert = Certificate::from_pem(client_ca_cert);

    let tls = ServerTlsConfig::with_rustls()
        .identity(identity)
        .client_ca_root(client_ca_cert)
        .clone();


    let addr = "127.0.0.1:50051".parse().unwrap();
    let server = EssemblyServer::default();

        Server::builder()
        .tls_config(&tls)
        .clone()
        .add_service(api::server::EssemblyServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::subscriber::set_global_default(logging::EssemblySubscriber::new(2)).unwrap();

    let app_span = span!(Level::TRACE, "", version = %5.0);
    let _e = app_span.enter();

    let server_span = span!(Level::TRACE, "server", host = "localhost", port = 8080);
    let _e2 = server_span.enter();
    info!("starting");
    info!("listening");
    let peer1 = span!(Level::TRACE, "conn", peer_addr = "82.9.9.9", port = 42381);
    peer1.in_scope(|| {
        debug!("connected");
        debug!(length = 2, "message received");
    });
    let peer2 = span!(Level::TRACE, "conn", peer_addr = "8.8.8.8", port = 18230);
    peer2.in_scope(|| {
        debug!("connected");
    });
    peer1.in_scope(|| {
        warn!(algo = "xor", "weak encryption requested");
        debug!(length = 8, "response sent");
        debug!("disconnected");
    });
    peer2.in_scope(|| {
        debug!(length = 5, "message received");
        debug!(length = 8, "response sent");
        debug!("disconnected");
    });
    warn!("internal error");
    info!("exit");

    run_server().await?;

    Ok(())
}
