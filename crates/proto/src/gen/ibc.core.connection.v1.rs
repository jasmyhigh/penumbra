/// Generated client implementations.
#[cfg(feature = "rpc")]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Query provides defines the gRPC querier service
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QueryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> QueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QueryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            QueryClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Connection queries an IBC connection end.
        pub async fn connection(
            &mut self,
            request: impl tonic::IntoRequest<
                ::ibc_proto::ibc::core::connection::v1::QueryConnectionRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                ::ibc_proto::ibc::core::connection::v1::QueryConnectionResponse,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.connection.v1.Query/Connection",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ibc.core.connection.v1.Query", "Connection"));
            self.inner.unary(req, path, codec).await
        }
        /// Connections queries all the IBC connections of a chain.
        pub async fn connections(
            &mut self,
            request: impl tonic::IntoRequest<
                ::ibc_proto::ibc::core::connection::v1::QueryConnectionsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                ::ibc_proto::ibc::core::connection::v1::QueryConnectionsResponse,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.connection.v1.Query/Connections",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("ibc.core.connection.v1.Query", "Connections"));
            self.inner.unary(req, path, codec).await
        }
        /// ClientConnections queries the connection paths associated with a client
        /// state.
        pub async fn client_connections(
            &mut self,
            request: impl tonic::IntoRequest<
                ::ibc_proto::ibc::core::connection::v1::QueryClientConnectionsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                ::ibc_proto::ibc::core::connection::v1::QueryClientConnectionsResponse,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.connection.v1.Query/ClientConnections",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ibc.core.connection.v1.Query", "ClientConnections"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// ConnectionClientState queries the client state associated with the
        /// connection.
        pub async fn connection_client_state(
            &mut self,
            request: impl tonic::IntoRequest<
                ::ibc_proto::ibc::core::connection::v1::QueryConnectionClientStateRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                ::ibc_proto::ibc::core::connection::v1::QueryConnectionClientStateResponse,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.connection.v1.Query/ConnectionClientState",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "ibc.core.connection.v1.Query",
                        "ConnectionClientState",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// ConnectionConsensusState queries the consensus state associated with the
        /// connection.
        pub async fn connection_consensus_state(
            &mut self,
            request: impl tonic::IntoRequest<
                ::ibc_proto::ibc::core::connection::v1::QueryConnectionConsensusStateRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                ::ibc_proto::ibc::core::connection::v1::QueryConnectionConsensusStateResponse,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.connection.v1.Query/ConnectionConsensusState",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "ibc.core.connection.v1.Query",
                        "ConnectionConsensusState",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// ConnectionParams queries all parameters of the ibc connection submodule.
        pub async fn connection_params(
            &mut self,
            request: impl tonic::IntoRequest<
                ::ibc_proto::ibc::core::connection::v1::QueryConnectionParamsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                ::ibc_proto::ibc::core::connection::v1::QueryConnectionParamsResponse,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ibc.core.connection.v1.Query/ConnectionParams",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("ibc.core.connection.v1.Query", "ConnectionParams"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "rpc")]
pub mod query_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with QueryServer.
    #[async_trait]
    pub trait Query: Send + Sync + 'static {
        /// Connection queries an IBC connection end.
        async fn connection(
            &self,
            request: tonic::Request<
                ::ibc_proto::ibc::core::connection::v1::QueryConnectionRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                ::ibc_proto::ibc::core::connection::v1::QueryConnectionResponse,
            >,
            tonic::Status,
        >;
        /// Connections queries all the IBC connections of a chain.
        async fn connections(
            &self,
            request: tonic::Request<
                ::ibc_proto::ibc::core::connection::v1::QueryConnectionsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                ::ibc_proto::ibc::core::connection::v1::QueryConnectionsResponse,
            >,
            tonic::Status,
        >;
        /// ClientConnections queries the connection paths associated with a client
        /// state.
        async fn client_connections(
            &self,
            request: tonic::Request<
                ::ibc_proto::ibc::core::connection::v1::QueryClientConnectionsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                ::ibc_proto::ibc::core::connection::v1::QueryClientConnectionsResponse,
            >,
            tonic::Status,
        >;
        /// ConnectionClientState queries the client state associated with the
        /// connection.
        async fn connection_client_state(
            &self,
            request: tonic::Request<
                ::ibc_proto::ibc::core::connection::v1::QueryConnectionClientStateRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                ::ibc_proto::ibc::core::connection::v1::QueryConnectionClientStateResponse,
            >,
            tonic::Status,
        >;
        /// ConnectionConsensusState queries the consensus state associated with the
        /// connection.
        async fn connection_consensus_state(
            &self,
            request: tonic::Request<
                ::ibc_proto::ibc::core::connection::v1::QueryConnectionConsensusStateRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                ::ibc_proto::ibc::core::connection::v1::QueryConnectionConsensusStateResponse,
            >,
            tonic::Status,
        >;
        /// ConnectionParams queries all parameters of the ibc connection submodule.
        async fn connection_params(
            &self,
            request: tonic::Request<
                ::ibc_proto::ibc::core::connection::v1::QueryConnectionParamsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                ::ibc_proto::ibc::core::connection::v1::QueryConnectionParamsResponse,
            >,
            tonic::Status,
        >;
    }
    /// Query provides defines the gRPC querier service
    #[derive(Debug)]
    pub struct QueryServer<T: Query> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Query> QueryServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for QueryServer<T>
    where
        T: Query,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/ibc.core.connection.v1.Query/Connection" => {
                    #[allow(non_camel_case_types)]
                    struct ConnectionSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        ::ibc_proto::ibc::core::connection::v1::QueryConnectionRequest,
                    > for ConnectionSvc<T> {
                        type Response = ::ibc_proto::ibc::core::connection::v1::QueryConnectionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::ibc_proto::ibc::core::connection::v1::QueryConnectionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::connection(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ConnectionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.connection.v1.Query/Connections" => {
                    #[allow(non_camel_case_types)]
                    struct ConnectionsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        ::ibc_proto::ibc::core::connection::v1::QueryConnectionsRequest,
                    > for ConnectionsSvc<T> {
                        type Response = ::ibc_proto::ibc::core::connection::v1::QueryConnectionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::ibc_proto::ibc::core::connection::v1::QueryConnectionsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::connections(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ConnectionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.connection.v1.Query/ClientConnections" => {
                    #[allow(non_camel_case_types)]
                    struct ClientConnectionsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        ::ibc_proto::ibc::core::connection::v1::QueryClientConnectionsRequest,
                    > for ClientConnectionsSvc<T> {
                        type Response = ::ibc_proto::ibc::core::connection::v1::QueryClientConnectionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::ibc_proto::ibc::core::connection::v1::QueryClientConnectionsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::client_connections(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ClientConnectionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.connection.v1.Query/ConnectionClientState" => {
                    #[allow(non_camel_case_types)]
                    struct ConnectionClientStateSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        ::ibc_proto::ibc::core::connection::v1::QueryConnectionClientStateRequest,
                    > for ConnectionClientStateSvc<T> {
                        type Response = ::ibc_proto::ibc::core::connection::v1::QueryConnectionClientStateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::ibc_proto::ibc::core::connection::v1::QueryConnectionClientStateRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::connection_client_state(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ConnectionClientStateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.connection.v1.Query/ConnectionConsensusState" => {
                    #[allow(non_camel_case_types)]
                    struct ConnectionConsensusStateSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        ::ibc_proto::ibc::core::connection::v1::QueryConnectionConsensusStateRequest,
                    > for ConnectionConsensusStateSvc<T> {
                        type Response = ::ibc_proto::ibc::core::connection::v1::QueryConnectionConsensusStateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::ibc_proto::ibc::core::connection::v1::QueryConnectionConsensusStateRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::connection_consensus_state(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ConnectionConsensusStateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/ibc.core.connection.v1.Query/ConnectionParams" => {
                    #[allow(non_camel_case_types)]
                    struct ConnectionParamsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        ::ibc_proto::ibc::core::connection::v1::QueryConnectionParamsRequest,
                    > for ConnectionParamsSvc<T> {
                        type Response = ::ibc_proto::ibc::core::connection::v1::QueryConnectionParamsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                ::ibc_proto::ibc::core::connection::v1::QueryConnectionParamsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Query>::connection_params(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ConnectionParamsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Query> Clone for QueryServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Query> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Query> tonic::server::NamedService for QueryServer<T> {
        const NAME: &'static str = "ibc.core.connection.v1.Query";
    }
}
