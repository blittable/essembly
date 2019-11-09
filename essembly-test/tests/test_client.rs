#![warn(rust_2018_idioms)]
#[allow(warnings)]
pub use serde_derive::{Deserialize, Serialize};


use essembly::interfaces;
use essembly::interfaces::api::*;

use std::collections::VecDeque;

use std::time::{Duration, Instant};

use tokio;
use tokio_test::block_on;
use tokio_test::assert_ok;
use tokio::timer::delay;

use tonic::{
    transport::{Identity, Server, ServerTlsConfig},
    Request, Response, Status, Streaming,
};

use essembly::interfaces::api::client::SusuClient;
use essembly::interfaces::api::server::*;
use essembly::interfaces::registration::*;

use http::header::HeaderValue;
use tonic::body::{BoxBody};
use tower::Service;

#[derive(Default)]
pub struct SusuServer;

type SusuResult<T> = Result<Response<T>, Status>;
type Stream = VecDeque<Result<SusuResponse, Status>>;

#[tonic::async_trait]
impl server::Susu for SusuServer {
    async fn register_client(
        &self,
        request: Request<SusuClientRegistration>,
    ) -> SusuResult<SusuResponse> {
        let message = request.into_inner().address.unwrap();

        println!("received message: {:?}", message);

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


//Currently this test just drops through
#[test]
fn run_server_test() {
    run_server();
}

async fn run_server() -> Result<(), Box<dyn std::error::Error>> {

    println!("Server Starting...");

    let cert = tokio::fs::read("tests/tls/server.pem").await?;
    let key = tokio::fs::read("tests/tls/server.key").await?;
    let server_identity = Identity::from_pem(cert, key);

    let addr = "127.0.0.1:50056".parse().unwrap();
    let server = SusuServer::default();

    let tls = ServerTlsConfig::with_rustls()
        .identity(server_identity).clone();

 let fut = Server::builder()
        .tls_config(&tls)
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
        .serve(addr);


    let server_result = fut.await.unwrap();

    println!("Result: {:?}", server_result);

    Ok(())
}

pub fn build_registration() -> interfaces::api::SusuClientRegistration {
    let address_line_1: String = "12/1 Some Soi".to_string();
    let address_line_2: String = "Sukhumvit".to_string();

    let mut addresslines = Vec::new();
    addresslines.push(address_line_1);
    addresslines.push(address_line_2);

    let new_latlng: interfaces::registration::LatLng = interfaces::registration::LatLng {
        latitude: 13.7563,
        longitude: 100.5018,
    };

    let new_address: interfaces::registration::Address = interfaces::registration::Address {
        address_lines: addresslines,
        city: "Bangkok".to_string(),
        province: "Bangkok".to_string(),
        country: "TH".to_string(),
        housing_estate: None,
        image_ref: None,
        latlng: Some(new_latlng),
    };

    let new_client: interfaces::registration::Client = interfaces::registration::Client {
        first_name: "Sompat".to_string(),
        last_name: Some("Sonjai".to_string()),
        middle_name: Some(String::from("")),
        nick_name: Some("อ้วน".to_string()),
    };

    let new_registration_status = 1;

    let new_registration = SusuClientRegistration {
        client: Some(new_client),
        address: Some(new_address),
        status: new_registration_status,
    };


    new_registration.clone()
}
