#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Chef {
    #[prost(string, tag="1")]
    pub first_name: std::string::String,
    #[prost(message, optional, tag="2")]
    pub middle_name: ::std::option::Option<::std::string::String>,
    #[prost(message, optional, tag="3")]
    pub last_name: ::std::option::Option<::std::string::String>,
    #[prost(message, optional, tag="4")]
    pub nick_name: ::std::option::Option<::std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Address {
    #[prost(string, repeated, tag="1")]
    pub address_lines: ::std::vec::Vec<std::string::String>,
    #[prost(string, tag="2")]
    pub city: std::string::String,
    #[prost(string, tag="3")]
    pub province: std::string::String,
    #[prost(string, tag="4")]
    pub country: std::string::String,
    #[prost(message, optional, tag="5")]
    pub image_ref: ::std::option::Option<::std::string::String>,
    #[prost(message, optional, tag="6")]
    pub housing_estate: ::std::option::Option<::std::string::String>,
    #[prost(message, optional, tag="7")]
    pub latlng: ::std::option::Option<LatLng>,
}
/// An object representing a latitude/longitude pair. This is expressed as a pair
/// of doubles representing degrees latitude and degrees longitude. Unless
/// specified otherwise, this must conform to the
/// <a href="http://www.unoosa.org/pdf/icg/2012/template/WGS_84.pdf">WGS84
/// standard</a>. Values must be within normalized ranges.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LatLng {
    /// The latitude in degrees. It must be in the range [-90.0, +90.0].
    #[prost(double, tag="1")]
    pub latitude: f64,
    /// The longitude in degrees. It must be in the range [-180.0, +180.0].
    #[prost(double, tag="2")]
    pub longitude: f64,
}
/// Status of a chef.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChefRegistrationStatus {
    Unknown = 0,
    RegistrationInProgress = 1,
    RegistrationCompleted = 2,
    Active = 4,
    Rejected = 5,
    Deactivated = 6,
    Cancelled = 7,
}
