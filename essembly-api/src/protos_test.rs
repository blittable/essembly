#[test]
fn test_chef_registration_status() {
    pub mod api {
        tonic::include_proto!("api");
    }

    let status: api::ChefRegistrationStatus = api::ChefRegistrationStatus::Unknown;
    assert_eq!(status, api::ChefRegistrationStatus::Unknown);
}

#[test]
fn test_chef_registration() {
    pub mod api {
        tonic::include_proto!("api");
    }

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

    let new_chef: api::Chef = api::Chef {
        first_name: "Sompat".to_string(),
        last_name: Some("Sonjai".to_string()),new_address
        middle_name: Some(String::from("")),
        nick_name: Some("อ้วน".to_string()),
    };

    let new_registration_status = 1;

    let new_registration = api::SusuChefRegistration {
        chef: Some(new_chef),
        address: Some(new_address),
        status: new_registration_status,
    };
}
