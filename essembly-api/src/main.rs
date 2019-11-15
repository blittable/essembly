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
use essembly_core::*;
use essembly_logging::*;

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
    //subscriber::set_global_default(essembly::logging::trace::EssemblySubscriber::new(2)).unwrap();
    info!("Starting up essembly api server...");
    //Read config file

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
    let config = &Config::new().load();

    let mut logger: essembly_logging::simple::SimpleLogger = essembly_logging::simple::SimpleLogger::new();

    logger.initialize(essembly_logging::Level::DEBUG);

    logger.log(essembly_logging::Level::DEBUG, "foo".to_string());

    let subscriber = tracing_subscriber::fmt::Subscriber::builder().finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("server started");
    trace!("tracing...");
    warn!("tracing...");

    let span = span!(
        Level::DEBUG,
        "starting",
        ip = ?config.cli.primary.ip,
        port = ?config.cli.primary.port,
        log_level = ?config.cli.primary.logging,
    );

    debug!("API configuration: {:?}", config.api);
    debug!("API logging configuration: {:?}", config.logger);
    tracing::debug!("API db configuration: {:?}", config.db);

    event!(Level::DEBUG, "something happened");

    let _enter = span.enter();
    run_server().await?;

    Ok(())
}
