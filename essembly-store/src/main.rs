#[allow(dead_code)]
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
    tonic::include_proto!("mycos.essembly.susu");
}

use sled::Db;

use std::collections::VecDeque;
use std::str;
use store::{SusuRequest, SusuResponse};
use tonic::transport::{Identity, Server, ServerTlsConfig};
use tonic::{body::BoxBody, Request, Response, Status, Streaming};
use tower::Service;

type SusuResult<T> = Result<Response<T>, Status>;
type Stream = VecDeque<Result<SusuResponse, Status>>;

fn save_to_db() -> Result<(), Box<dyn std::error::Error>> {
    let path = "./foo.db";
    let tree = Db::open(path)?;
    let options = tree.open_tree("options")?;

    for i in 0..10 {
        options.insert(b"save", "doo");
    }

    let mut counter: i32 = 0;

    for node in &tree.tree_names() {
        println!("tree: {:?}", str::from_utf8(&node));
    }

    tree.flush();

    Ok(())
}

#[derive(Default)]
pub struct SusuServer;

#[tonic::async_trait]
impl store::server::Susu for SusuServer {
    async fn unary_susu(&self, request: Request<SusuRequest>) -> SusuResult<SusuResponse> {
        let message = request.into_inner().message;
        println!("Message");

        save_to_db();

        Ok(Response::new(SusuResponse { message }))
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
        .serve(addr, store::server::SusuServer::new(server))
        .await?;

    Ok(())
}
