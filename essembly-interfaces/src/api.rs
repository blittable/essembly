/// Susu. Chef Registration
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SusuChefRegistration {
    #[prost(message, optional, tag = "1")]
    pub chef: ::std::option::Option<super::registration::Chef>,
    #[prost(message, optional, tag = "2")]
    pub address: ::std::option::Option<super::registration::Address>,
    #[prost(enumeration = "super::registration::ChefRegistrationStatus", tag = "3")]
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
            request: impl tonic::IntoRequest<super::SusuChefRegistration>,
        ) -> Result<tonic::Response<super::SusuResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.Susu/RegisterChef");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " UnarySusu is unary echo."]
        pub async fn unary_susu(
            &mut self,
            request: impl tonic::IntoRequest<super::SusuRequest>,
        ) -> Result<tonic::Response<super::SusuResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.Susu/UnarySusu");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " ServerStreamingSusu is server side streaming."]
        pub async fn server_streaming_susu(
            &mut self,
            request: impl tonic::IntoRequest<super::SusuRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::SusuResponse>>, tonic::Status>
        {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.Susu/ServerStreamingSusu");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " ClientStreamingSusu is client side streaming."]
        pub async fn client_streaming_susu(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::SusuRequest>,
        ) -> Result<tonic::Response<super::SusuResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.Susu/ClientStreamingSusu");
            self.inner
                .client_streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = " BidirectionalStreamingSusu is bidirectional streaming."]
        pub async fn bidirectional_streaming_susu(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::SusuRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::SusuResponse>>, tonic::Status>
        {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.Susu/BidirectionalStreamingSusu");
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
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
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct SusuServer<T: Susu> {
        inner: Arc<T>,
    }
    impl<T: Susu> SusuServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            Self { inner }
        }
    }
    impl<T: Susu> Service<http::Request<HyperBody>> for SusuServer<T> {
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
                    struct RegisterChefSvc<T: Susu>(pub Arc<T>);
                    impl<T: Susu> tonic::server::UnaryService<super::SusuChefRegistration> for RegisterChefSvc<T> {
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
                        let method = RegisterChefSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.Susu/UnarySusu" => {
                    struct UnarySusuSvc<T: Susu>(pub Arc<T>);
                    impl<T: Susu> tonic::server::UnaryService<super::SusuRequest> for UnarySusuSvc<T> {
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
                        let method = UnarySusuSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.Susu/ServerStreamingSusu" => {
                    struct ServerStreamingSusuSvc<T: Susu>(pub Arc<T>);
                    impl<T: Susu> tonic::server::ServerStreamingService<super::SusuRequest>
                        for ServerStreamingSusuSvc<T>
                    {
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
                        let method = ServerStreamingSusuSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.Susu/ClientStreamingSusu" => {
                    struct ClientStreamingSusuSvc<T: Susu>(pub Arc<T>);
                    impl<T: Susu> tonic::server::ClientStreamingService<super::SusuRequest>
                        for ClientStreamingSusuSvc<T>
                    {
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
                        let method = ClientStreamingSusuSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.Susu/BidirectionalStreamingSusu" => {
                    struct BidirectionalStreamingSusuSvc<T: Susu>(pub Arc<T>);
                    impl<T: Susu> tonic::server::StreamingService<super::SusuRequest>
                        for BidirectionalStreamingSusuSvc<T>
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
                        let method = BidirectionalStreamingSusuSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
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
    impl<T: Susu> Clone for SusuServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Susu> tonic::transport::ServiceName for SusuServer<T> {
        const NAME: &'static str = "api.Susu";
    }
}
