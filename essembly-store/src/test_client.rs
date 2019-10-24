#![warn(rust_2018_idioms)]
#[allow(warnings)]
pub use serde_derive::{Deserialize, Serialize};

mod db;
mod susu_error;

use crate::susu_error::{Error, ErrorCode};

use std::path::PathBuf;

use tracing::{debug, instrument, log, Subscriber};
use tracing_attributes;
use tracing_futures;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};

use tokio;
use tokio::codec::{Framed, LinesCodec};
use tokio::net::TcpListener;

use futures::{SinkExt, StreamExt};

static DATABASE_NAME: &str = "susu.db";

pub mod store {
    tonic::include_proto!("api");
}

use sled::Db;

use std::collections::VecDeque;
use std::str;
use store::client::SusuClient;
use store::{SusuRequest, SusuResponse};
use tonic::transport::{Channel, Certificate, ClientTlsConfig, Identity, Server};
use tonic::{body::BoxBody, Request, Response, Status, Streaming};
use tower::Service;
use http::header::HeaderValue;

type SusuResult<T> = Result<Response<T>, Status>;
type Stream = VecDeque<Result<SusuResponse, Status>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let pem = tokio::fs::read("tls/ca.pem").await?;
    let ca = Certificate::from_pem(pem);

    let tls = ClientTlsConfig::with_rustls()
        .ca_certificate(ca)
        .domain_name("example.com")
        .clone();

    let channel = Channel::from_static("http://127.0.0.1:50051")
        .tls_config(&tls)
        .intercept_headers(|headers| {
            headers.insert(
                "authorization",
                HeaderValue::from_static("Bearer some-secret-token"),
            );
        })
        .channel();

    let mut client = SusuClient::new(channel);
    let request = tonic::Request::new(SusuRequest {
        message: "hello".into(),
    });

    let response = client.unary_susu(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
