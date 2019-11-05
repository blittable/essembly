#![warn(rust_2018_idioms)]
#[allow(warnings)]
pub use serde_derive::{Deserialize, Serialize};


use essembly::interfaces;
use essembly::interfaces::api::*;
use essembly::interfaces::registration::*;


use tokio;

use essembly::interfaces::api::client::SusuClient;
use http::header::HeaderValue;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};

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

    let registration = build_registration();
    let request = tonic::Request::new(registration);

    let response = client.register_chef(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

pub fn build_registration() -> interfaces::api::SusuChefRegistration {
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

    let new_chef: interfaces::registration::Chef = interfaces::registration::Chef {
        first_name: "Sompat".to_string(),
        last_name: Some("Sonjai".to_string()),
        middle_name: Some(String::from("")),
        nick_name: Some("อ้วน".to_string()),
    };

    let new_registration_status = 1;

    registration::SusuChefRegistration {
        chef: Some(new_chef),
        address: Some(new_address),
        status: new_registration_status,
    };


    let read_back = new_registration.chef.unwrap().first_name;
    assert_eq!(read_back, String::new());
}
