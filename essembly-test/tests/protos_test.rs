#[test]
fn test_chef_registration_status() {
    use essembly::interfaces::*;

    let status: registration::ChefRegistrationStatus = registration::ChefRegistrationStatus::Unknown;
    assert_eq!(status, registration::ChefRegistrationStatus::Unknown);
}

#[test]
fn test_chef_registration() {
    use essembly::interfaces::*;

    let address_line_1: String = "12/1 Some Soi".to_string();
    let address_line_2: String = "Sukhumvit".to_string();

    let mut addresslines = Vec::new();
    addresslines.push(address_line_1);
    addresslines.push(address_line_2);

    let new_latlng: registration::LatLng = registration::LatLng {
        latitude: 13.7563,
        longitude: 100.5018,
    };

    let new_address: registration::Address = registration::Address {
        address_lines: addresslines,
        city: "Bangkok".to_string(),
        province: "Bangkok".to_string(),
        country: "TH".to_string(),
        housing_estate: None,
        image_ref: None,
        latlng: Some(new_latlng),
    };

    let new_chef: registration::Chef = registration::Chef {
        first_name: "Sompat".to_string(),
        last_name: Some("Sonjai".to_string()),
        middle_name: Some(String::from("")),
        nick_name: Some("อ้วน".to_string()),
    };

    let new_registration_status = 1;

    let new_registration = api::SusuChefRegistration {
        chef: Some(new_chef),
        address: Some(new_address),
        status: new_registration_status,
    };

    let read_back = new_registration.chef.unwrap().first_name;
    assert_ne!(read_back, String::new());
}
