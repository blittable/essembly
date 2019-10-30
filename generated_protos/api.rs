#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Chef {
    #[prost(string, tag = "1")]
    pub first_name: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub middle_name: ::std::option::Option<::std::string::String>,
    #[prost(message, optional, tag = "3")]
    pub last_name: ::std::option::Option<::std::string::String>,
    #[prost(message, optional, tag = "4")]
    pub nick_name: ::std::option::Option<::std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Address {
    #[prost(string, repeated, tag = "1")]
    pub address_lines: ::std::vec::Vec<std::string::String>,
    #[prost(string, tag = "2")]
    pub city: std::string::String,
    #[prost(string, tag = "3")]
    pub province: std::string::String,
    #[prost(string, tag = "4")]
    pub country: std::string::String,
    #[prost(message, optional, tag = "5")]
    pub image_ref: ::std::option::Option<::std::string::String>,
    #[prost(message, optional, tag = "6")]
    pub housing_estate: ::std::option::Option<::std::string::String>,
    #[prost(message, optional, tag = "7")]
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
    #[prost(double, tag = "1")]
    pub latitude: f64,
    /// The longitude in degrees. It must be in the range [-180.0, +180.0].
    #[prost(double, tag = "2")]
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
/// Susu. Chef Registration
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SusuChefRegistration {
    #[prost(message, optional, tag = "1")]
    pub chef: ::std::option::Option<Chef>,
    #[prost(message, optional, tag = "2")]
    pub address: ::std::option::Option<Address>,
    #[prost(enumeration = "ChefRegistrationStatus", tag = "3")]
    pub status: i32,
}
/// SusuRequest is the request for echo.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SusuRequest {
    #[prost(string, tag = "1")]
    pub message: std::string::String,
}
/// SusuResponse is the response for echo.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SusuResponse {
    #[prost(string, tag = "1")]
    pub message: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Susu is the echo service."]
    pub struct SusuClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SusuClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            tonic::transport::Endpoint::new(dst).map(|c| Self::new(c.channel()))
        }
    }
    impl<T> SusuClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
        <T::ResponseBody as HttpBody>::Data: Into<bytes::Bytes> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        #[doc = r" Check if the service is ready."]
        pub async fn ready(&mut self) -> Result<(), tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })
        }
        pub async fn register_chef(
            &mut self,
            request: tonic::Request<super::SusuChefRegistration>,
        ) -> Result<tonic::Response<super::SusuResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path = http::uri::PathAndQuery::from_static("/api.Susu/RegisterChef");
            self.inner.unary(request, path, codec).await
        }
        #[doc = " UnarySusu is unary echo."]
        pub async fn unary_susu(
            &mut self,
            request: tonic::Request<super::SusuRequest>,
        ) -> Result<tonic::Response<super::SusuResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path = http::uri::PathAndQuery::from_static("/api.Susu/UnarySusu");
            self.inner.unary(request, path, codec).await
        }
        #[doc = " ServerStreamingSusu is server side streaming."]
        pub async fn server_streaming_susu(
            &mut self,
            request: tonic::Request<super::SusuRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::SusuResponse>>, tonic::Status>
        {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path = http::uri::PathAndQuery::from_static("/api.Susu/ServerStreamingSusu");
            self.inner.server_streaming(request, path, codec).await
        }
        #[doc = " ClientStreamingSusu is client side streaming."]
        pub async fn client_streaming_susu<S>(
            &mut self,
            request: tonic::Request<S>,
        ) -> Result<tonic::Response<super::SusuResponse>, tonic::Status>
        where
            S: Stream<Item = super::SusuRequest> + Send + 'static,
        {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path = http::uri::PathAndQuery::from_static("/api.Susu/ClientStreamingSusu");
            self.inner.client_streaming(request, path, codec).await
        }
        #[doc = " BidirectionalStreamingSusu is bidirectional streaming."]
        pub async fn bidirectional_streaming_susu<S>(
            &mut self,
            request: tonic::Request<S>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::SusuResponse>>, tonic::Status>
        where
            S: Stream<Item = super::SusuRequest> + Send + 'static,
        {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::new();
            let path = http::uri::PathAndQuery::from_static("/api.Susu/BidirectionalStreamingSusu");
            self.inner.streaming(request, path, codec).await
        }
    }
    impl<T: Clone> Clone for SusuClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with SusuServer."]
    #[async_trait]
    pub trait Susu: Send + Sync + 'static {
        async fn register_chef(
            &self,
            request: tonic::Request<super::SusuChefRegistration>,
        ) -> Result<tonic::Response<super::SusuResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        #[doc = " UnarySusu is unary echo."]
        async fn unary_susu(
            &self,
            request: tonic::Request<super::SusuRequest>,
        ) -> Result<tonic::Response<super::SusuResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        #[doc = "Server streaming response type for the ServerStreamingSusu method."]
        type ServerStreamingSusuStream: Stream<Item = Result<super::SusuResponse, tonic::Status>>
            + Send
            + 'static;
        #[doc = " ServerStreamingSusu is server side streaming."]
        async fn server_streaming_susu(
            &self,
            request: tonic::Request<super::SusuRequest>,
        ) -> Result<tonic::Response<Self::ServerStreamingSusuStream>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        #[doc = " ClientStreamingSusu is client side streaming."]
        async fn client_streaming_susu(
            &self,
            request: tonic::Request<tonic::Streaming<super::SusuRequest>>,
        ) -> Result<tonic::Response<super::SusuResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        #[doc = "Server streaming response type for the BidirectionalStreamingSusu method."]
        type BidirectionalStreamingSusuStream: Stream<Item = Result<super::SusuResponse, tonic::Status>>
            + Send
            + 'static;
        #[doc = " BidirectionalStreamingSusu is bidirectional streaming."]
        async fn bidirectional_streaming_susu(
            &self,
            request: tonic::Request<tonic::Streaming<super::SusuRequest>>,
        ) -> Result<tonic::Response<Self::BidirectionalStreamingSusuStream>, tonic::Status>
        {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
    }
    #[doc = " Susu is the echo service."]
    #[derive(Clone, Debug)]
    pub struct SusuServer<T: Susu> {
        inner: Arc<T>,
    }
    #[derive(Clone, Debug)]
    #[doc(hidden)]
    pub struct SusuServerSvc<T: Susu> {
        inner: Arc<T>,
    }
    impl<T: Susu> SusuServer<T> {
        #[doc = "Create a new SusuServer from a type that implements Susu."]
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            Self::from_shared(inner)
        }
        pub fn from_shared(inner: Arc<T>) -> Self {
            Self { inner }
        }
    }
    impl<T: Susu> SusuServerSvc<T> {
        pub fn new(inner: Arc<T>) -> Self {
            Self { inner }
        }
    }
    impl<T: Susu, R> Service<R> for SusuServer<T> {
        type Response = SusuServerSvc<T>;
        type Error = Never;
        type Future = Ready<Result<Self::Response, Self::Error>>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, _: R) -> Self::Future {
            ok(SusuServerSvc::new(self.inner.clone()))
        }
    }
    impl<T: Susu> Service<http::Request<HyperBody>> for SusuServerSvc<T> {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<HyperBody>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/api.Susu/RegisterChef" => {
                    struct RegisterChef<T: Susu>(pub Arc<T>);
                    impl<T: Susu> tonic::server::UnaryService<super::SusuChefRegistration> for RegisterChef<T> {
                        type Response = super::SusuResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SusuChefRegistration>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.register_chef(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RegisterChef(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.Susu/UnarySusu" => {
                    struct UnarySusu<T: Susu>(pub Arc<T>);
                    impl<T: Susu> tonic::server::UnaryService<super::SusuRequest> for UnarySusu<T> {
                        type Response = super::SusuResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SusuRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.unary_susu(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UnarySusu(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.Susu/ServerStreamingSusu" => {
                    struct ServerStreamingSusu<T: Susu>(pub Arc<T>);
                    impl<T: Susu> tonic::server::ServerStreamingService<super::SusuRequest> for ServerStreamingSusu<T> {
                        type Response = super::SusuResponse;
                        type ResponseStream = T::ServerStreamingSusuStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SusuRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.server_streaming_susu(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ServerStreamingSusu(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.Susu/ClientStreamingSusu" => {
                    struct ClientStreamingSusu<T: Susu>(pub Arc<T>);
                    impl<T: Susu> tonic::server::ClientStreamingService<super::SusuRequest> for ClientStreamingSusu<T> {
                        type Response = super::SusuResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::SusuRequest>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.client_streaming_susu(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ClientStreamingSusu(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.Susu/BidirectionalStreamingSusu" => {
                    struct BidirectionalStreamingSusu<T: Susu>(pub Arc<T>);
                    impl<T: Susu> tonic::server::StreamingService<super::SusuRequest>
                        for BidirectionalStreamingSusu<T>
                    {
                        type Response = super::SusuResponse;
                        type ResponseStream = T::BidirectionalStreamingSusuStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::SusuRequest>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.bidirectional_streaming_susu(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = BidirectionalStreamingSusu(inner);
                        let codec = tonic::codec::ProstCodec::new();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
}
