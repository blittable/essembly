#![warn(rust_2018_idioms)]
#[allow(warnings)]
pub use serde_derive::{Deserialize, Serialize};

pub mod api {
    tonic::include_proto!("api");
}

use tokio;

use api::client::EssemblyClient;
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

    let mut client = EssemblyClient::new(channel);

    let registration = build_registration();
    let request = tonic::Request::new(registration);

    let response = client.register_client(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

pub fn build_registration() -> api::EssemblyClientRegistration {
    let address_line_1: String = "12/1 Some Soi".to_string();
    let address_line_2: String = "Sukhumvit".to_string();

    let mut addresslines = Vec::new();
    addresslines.push(address_line_1);
    addresslines.push(address_line_2);

    let new_latlng: api::LatLng = api::LatLng {
        latitude: 13.7563,
        longitude: 100.5018,
    };

    let new_address: api::Address = api::Address {
        address_lines: addresslines,
        city: "Bangkok".to_string(),
        province: "Bangkok".to_string(),
        country: "TH".to_string(),
        housing_estate: None,
        image_ref: None,
        latlng: Some(new_latlng),
    };

    let new_client: api::Client = api::Client {
        first_name: "Sompat".to_string(),
        last_name: Some("Sonjai".to_string()),
        middle_name: Some(String::from("")),
        nick_name: Some("อ้วน".to_string()),
    };

    let new_registration_status = 1;

    api::EssemblyClientRegistration {
        client: Some(new_client),
        address: Some(new_address),
        status: new_registration_status,
    }
}
