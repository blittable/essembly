#![deny(rust_2018_idioms)]
#[allow(dead_code)]
#[allow(warnings)]
pub use serde_derive::{Deserialize, Serialize};

use tokio;
use tracing;
use essembly::interfaces::*;

#[allow(unused_imports)]
use tracing::{debug, error, info, event, warn, span, Level};

use essembly::logging;
use essembly::core::*;

#[allow(dead_code)]
static DATABASE_NAME: &str = "susu.db";

use std::collections::VecDeque;
use std::str;
use essembly::interfaces::api::{SusuRequest, SusuResponse};
use tonic::transport::{ Server, ServerTlsConfig};
use tonic::{Request, Response, Status, Streaming};
use essembly::core;

type SusuResult<T> = Result<Response<T>, Status>;
type Stream = VecDeque<Result<SusuResponse, Status>>;

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
    let latitude = location.latitude.to_string();
    let longitude = location.longitude.to_string();

    Ok(())
}

#[derive(Default)]
pub struct SusuServer;

#[tonic::async_trait]
impl api::server::Susu for SusuServer {
    async fn register_chef(
        &self,
        request: Request<api::SusuChefRegistration>,
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

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {

//     let subscriber = SloggishSubscriber::new(2);
//     tracing::subscriber::set_global_default(subscriber).unwrap();

//     debug!("started");
//     warn!("started");
//     info!("started");

//     let cert = tokio::fs::read("tls/server.pem").await?;
//     let key = tokio::fs::read("tls/server.key").await?;

//     let identity = Identity::from_pem(cert, key);

//     let addr = "127.0.0.1:50051".parse().unwrap();
//     let server = SusuServer::default();

//     // let subscriber = FmtSubscriber::builder() 
//     //     .with_max_level(Level::TRACE)
//     //     .finish();


//     Server::builder()
//         .tls_config(ServerTlsConfig::with_rustls().identity(identity))
//         .interceptor_fn(move |svc, req| {
//             let auth_header = req.headers().get("authorization").clone();

//             let authed = if let Some(auth_header) = auth_header {
//                 auth_header == "Bearer some-secret-token"
//             } else {
//                 false
//             };

//             let fut = svc.call(req);

//             async move {
//                 if authed {
//                     fut.await
//                 } else {
//                     // Cancel the inner future since we never await it
//                     // the IO never gets registered.
//                     drop(fut);
//                     let res = http::Response::builder()
//                         .header("grpc-status", "16")
//                         .body(BoxBody::empty())
//                         .unwrap();
//                     Ok(res)
//                 }
//             }
//         })
//         .clone()
//         .add_service(api::server::SusuServer::new(server))
//         .serve(addr)
//         .await?;
//     Ok(())
// }

fn main() {
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
}