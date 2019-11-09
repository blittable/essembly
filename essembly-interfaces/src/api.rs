/// Essembly. Client Registration
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EssemblyClientRegistration {
    #[prost(message, optional, tag = "1")]
    pub client: ::std::option::Option<super::registration::Client>,
    #[prost(message, optional, tag = "2")]
    pub address: ::std::option::Option<super::registration::Address>,
    #[prost(
        enumeration = "super::registration::ClientRegistrationStatus",
        tag = "3"
    )]
    pub status: i32,
}
/// EssemblyRequest is the request for echo.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EssemblyRequest {
    #[prost(string, tag = "1")]
    pub message: std::string::String,
}
/// EssemblyResponse is the response for echo.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EssemblyResponse {
    #[prost(string, tag = "1")]
    pub message: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Essembly is the echo service."]
    pub struct EssemblyClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl EssemblyClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            tonic::transport::Endpoint::new(dst).map(|c| Self::new(c.channel()))
        }
    }
    impl<T> EssemblyClient<T>
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
        pub async fn register_client(
            &mut self,
            request: impl tonic::IntoRequest<super::EssemblyClientRegistration>,
        ) -> Result<tonic::Response<super::EssemblyResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.Essembly/RegisterClient");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " UnaryEssembly is unary echo."]
        pub async fn unary_essembly(
            &mut self,
            request: impl tonic::IntoRequest<super::EssemblyRequest>,
        ) -> Result<tonic::Response<super::EssemblyResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.Essembly/UnaryEssembly");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " ServerStreamingEssembly is server side streaming."]
        pub async fn server_streaming_essembly(
            &mut self,
            request: impl tonic::IntoRequest<super::EssemblyRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::EssemblyResponse>>, tonic::Status>
        {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/api.Essembly/ServerStreamingEssembly");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " ClientStreamingEssembly is client side streaming."]
        pub async fn client_streaming_essembly(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::EssemblyRequest>,
        ) -> Result<tonic::Response<super::EssemblyResponse>, tonic::Status> {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/api.Essembly/ClientStreamingEssembly");
            self.inner
                .client_streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = " BidirectionalStreamingEssembly is bidirectional streaming."]
        pub async fn bidirectional_streaming_essembly(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::EssemblyRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::EssemblyResponse>>, tonic::Status>
        {
            self.ready().await?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.Essembly/BidirectionalStreamingEssembly",
            );
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
    }
    impl<T: Clone> Clone for EssemblyClient<T> {
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
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with EssemblyServer."]
    #[async_trait]
    pub trait Essembly: Send + Sync + 'static {
        async fn register_client(
            &self,
            request: tonic::Request<super::EssemblyClientRegistration>,
        ) -> Result<tonic::Response<super::EssemblyResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        #[doc = " UnaryEssembly is unary echo."]
        async fn unary_essembly(
            &self,
            request: tonic::Request<super::EssemblyRequest>,
        ) -> Result<tonic::Response<super::EssemblyResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        #[doc = "Server streaming response type for the ServerStreamingEssembly method."]
        type ServerStreamingEssemblyStream: Stream<Item = Result<super::EssemblyResponse, tonic::Status>>
            + Send
            + 'static;
        #[doc = " ServerStreamingEssembly is server side streaming."]
        async fn server_streaming_essembly(
            &self,
            request: tonic::Request<super::EssemblyRequest>,
        ) -> Result<tonic::Response<Self::ServerStreamingEssemblyStream>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        #[doc = " ClientStreamingEssembly is client side streaming."]
        async fn client_streaming_essembly(
            &self,
            request: tonic::Request<tonic::Streaming<super::EssemblyRequest>>,
        ) -> Result<tonic::Response<super::EssemblyResponse>, tonic::Status> {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
        #[doc = "Server streaming response type for the BidirectionalStreamingEssembly method."]
        type BidirectionalStreamingEssemblyStream: Stream<Item = Result<super::EssemblyResponse, tonic::Status>>
            + Send
            + 'static;
        #[doc = " BidirectionalStreamingEssembly is bidirectional streaming."]
        async fn bidirectional_streaming_essembly(
            &self,
            request: tonic::Request<tonic::Streaming<super::EssemblyRequest>>,
        ) -> Result<tonic::Response<Self::BidirectionalStreamingEssemblyStream>, tonic::Status>
        {
            Err(tonic::Status::unimplemented("Not yet implemented"))
        }
    }
    #[doc = " Essembly is the echo service."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct EssemblyServer<T: Essembly> {
        inner: Arc<T>,
    }
    impl<T: Essembly> EssemblyServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            Self { inner }
        }
    }
    impl<T: Essembly> Service<http::Request<HyperBody>> for EssemblyServer<T> {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<HyperBody>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/api.Essembly/RegisterClient" => {
                    struct RegisterClientSvc<T: Essembly>(pub Arc<T>);
                    impl<T: Essembly> tonic::server::UnaryService<super::EssemblyClientRegistration>
                        for RegisterClientSvc<T>
                    {
                        type Response = super::EssemblyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EssemblyClientRegistration>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.register_client(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = RegisterClientSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.Essembly/UnaryEssembly" => {
                    struct UnaryEssemblySvc<T: Essembly>(pub Arc<T>);
                    impl<T: Essembly> tonic::server::UnaryService<super::EssemblyRequest> for UnaryEssemblySvc<T> {
                        type Response = super::EssemblyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EssemblyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.unary_essembly(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UnaryEssemblySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.Essembly/ServerStreamingEssembly" => {
                    struct ServerStreamingEssemblySvc<T: Essembly>(pub Arc<T>);
                    impl<T: Essembly> tonic::server::ServerStreamingService<super::EssemblyRequest>
                        for ServerStreamingEssemblySvc<T>
                    {
                        type Response = super::EssemblyResponse;
                        type ResponseStream = T::ServerStreamingEssemblyStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EssemblyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.server_streaming_essembly(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ServerStreamingEssemblySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.Essembly/ClientStreamingEssembly" => {
                    struct ClientStreamingEssemblySvc<T: Essembly>(pub Arc<T>);
                    impl<T: Essembly> tonic::server::ClientStreamingService<super::EssemblyRequest>
                        for ClientStreamingEssemblySvc<T>
                    {
                        type Response = super::EssemblyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::EssemblyRequest>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.client_streaming_essembly(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ClientStreamingEssemblySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec);
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.Essembly/BidirectionalStreamingEssembly" => {
                    struct BidirectionalStreamingEssemblySvc<T: Essembly>(pub Arc<T>);
                    impl<T: Essembly> tonic::server::StreamingService<super::EssemblyRequest>
                        for BidirectionalStreamingEssemblySvc<T>
                    {
                        type Response = super::EssemblyResponse;
                        type ResponseStream = T::BidirectionalStreamingEssemblyStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::EssemblyRequest>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                inner.bidirectional_streaming_essembly(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = BidirectionalStreamingEssemblySvc(inner);
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
    impl<T: Essembly> Clone for EssemblyServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Essembly> tonic::transport::ServiceName for EssemblyServer<T> {
        const NAME: &'static str = "api.Essembly";
    }
}
