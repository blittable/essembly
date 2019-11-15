#![warn(rust_2018_idioms)]
#[allow(warnings)]
#[allow(dead_code)]
pub use serde_derive::{Deserialize, Serialize};


use essembly::interfaces;
use tracing_attributes;
use tracing;

use tokio;

use tonic::{
    transport::{Certificate, Channel, ClientTlsConfig, Identity},
};

use essembly::interfaces::registration::Client;
use essembly::interfaces::api::{ EssemblyClientRegistration };
use essembly::interfaces::api::client::EssemblyClient;
use essembly::logging::*;


#[tokio::main]
#[tracing_attributes::instrument]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Calling client packet send...");
    run_client().await?;
    Ok(())
}

async fn run_client() -> Result<(), Box<dyn std::error::Error>> {

    let mut logger = essembly::logging::simple::SimpleLogger::new();
    logger.initialize(Level::DEBUG);

    println!("Loading Certificates...");

    let server_root_ca_cert = tokio::fs::read("tests/tls/ca.pem").await?;
    let server_root_ca_cert = Certificate::from_pem(server_root_ca_cert);

    println!("Roots loaded...");
    let client_cert = tokio::fs::read("tests/tls/client1.pem").await?;
    let client_key = tokio::fs::read("tests/tls/client1.key").await?;
    let client_identity = Identity::from_pem(client_cert, client_key);

    let tls = ClientTlsConfig::with_rustls()
        .domain_name("localhost")
        .ca_certificate(server_root_ca_cert)
        .identity(client_identity)
        .clone();

        let channel = Channel::from_static("http://[::1]:50051")
        .tls_config(&tls)
        .connect()
        .await?;

        let setup_request = build_registration();

       let request = tonic::Request::new(setup_request);

    let mut client = EssemblyClient::new(channel);

    let response = client.register_client(request).await?;

    println!("RESPONSE={:?}", response);
    println!("Completed Client Packet Send: {:?}", "");

    Ok(())
}

pub fn build_registration() -> interfaces::api::EssemblyClientRegistration {


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

    let new_registration = EssemblyClientRegistration {
        client: Some(new_client),
        address: Some(new_address),
        status: new_registration_status,
    };


    new_registration.clone()
}
