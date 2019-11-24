#![deny(rust_2018_idioms)]
#[allow(dead_code)]
#[allow(warnings)]
pub use serde_derive::{Deserialize, Serialize};

use essembly_interfaces::*;

#[allow(unused_imports)]
use tracing::{debug, error, event, info, span, trace, warn, Level};

#[allow(dead_code)]
static DATABASE_NAME: &str = "essembly.db";

use essembly_config::Config;
use essembly_interfaces;
use essembly_interfaces::api::*;

use std::str;
use tonic::{
    transport::{Certificate, Identity, Server, ServerTlsConfig},
    Request, Response, Status, 
};

type EssemblyResult<T> = Result<Response<T>, Status>;

#[allow(dead_code)]
fn save_to_db(message: registration::Address) -> Result<(), Box<dyn std::error::Error>> {
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
impl essembly_interfaces::api::server::Essembly for EssemblyServer {
    async fn register_client(
        &self,
        request: Request<api::EssemblyClientRegistration>,
    ) -> EssemblyResult<EssemblyResponse> {
        event!(Level::INFO, "Server request received.");

        let message = request.into_inner().address.unwrap();
        let peer = span!(Level::INFO, "request", peer_addr = "82.9.9.9", port = 42381);

        peer.in_scope(|| {
            debug!("connected");
            debug!(length = 2, "message received");
            info!("received message: {:?}", message);
        });

        Ok(Response::new(EssemblyResponse {
            message: "Received Registration".to_string(),
        }))
    }

}
async fn run_server() -> Result<(), Box<dyn std::error::Error>> {

    let pem = std::fs::read("essembly-api/tls/server.pem")?;
    let key = std::fs::read("essembly-api/tls/server.key")?;

    let identity = Identity::from_pem(pem, key);

    let client_ca_cert_file = tokio::fs::read("essembly-api/tls/client_ca.pem").await?;
    let client_ca_cert = Certificate::from_pem(client_ca_cert_file);

    let tls = ServerTlsConfig::with_rustls()
        .identity(identity)
        .client_ca_root(client_ca_cert)
        .clone();

    let addr = "[::1]:50051".parse().unwrap();
    let server = EssemblyServer::default();

    Server::builder()
        .tls_config(&tls)
        .add_service(essembly_interfaces::api::server::EssemblyServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Initialize the client with its configuration
    let config = &Config::new().load();

    //logging
    let s = essembly_logging::trace::EssemblySubscriber::new(4);
    tracing::subscriber::set_global_default(s).unwrap();

    info!("cli runtime started");
    debug!("cli config: {:?}", config.cli);
    debug!("api config: {:?}", config.cli);

   // let mut logger: essembly_logging::simple::SimpleLogger = essembly_logging::simple::SimpleLogger::new(); 
   // logger.initialize(essembly_logging::Level::DEBUG);

    let span = span!(
        Level::TRACE,
        "starting",
        direct_to_db = ?config.cli.details.direct_to_db,
        log_level = ?config.cli.details.logging,
    );


    debug!("API configuration: {:?}", config.api);
    debug!("API logging configuration: {:?}", config.logger);
    tracing::debug!("API db configuration: {:?}", config.db);


    tracing::debug!("API db configuration: {:?}", config.db);

    trace!("server started");

    run_server().await?;

    Ok(())
}
