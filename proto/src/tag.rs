// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tag {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub is_del: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTagRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CreateTagReply {
    #[prost(int32, tag = "1")]
    pub id: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditTagRequest {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct EditTagReply {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(bool, tag = "2")]
    pub ok: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTagsRequest {
    /// 根据名称
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// 根据是否已被删除
    #[prost(bool, optional, tag = "2")]
    pub is_del: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTagsReply {
    #[prost(message, repeated, tag = "1")]
    pub tags: ::prost::alloc::vec::Vec<Tag>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ToggleTagRequest {
    #[prost(int32, tag = "1")]
    pub id: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ToggleTagReply {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(bool, tag = "2")]
    pub is_del: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagExistsRequest {
    #[prost(oneof = "tag_exists_request::Condition", tags = "1, 2")]
    pub condition: ::core::option::Option<tag_exists_request::Condition>,
}
/// Nested message and enum types in `TagExistsRequest`.
pub mod tag_exists_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Condition {
        #[prost(string, tag = "1")]
        Name(::prost::alloc::string::String),
        #[prost(int32, tag = "2")]
        Id(i32),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct TagExistsReply {
    #[prost(bool, tag = "1")]
    pub exists: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct GetTagInfoRequest {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(bool, optional, tag = "2")]
    pub is_del: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTagInfoReply {
    #[prost(message, optional, tag = "1")]
    pub tag: ::core::option::Option<Tag>,
}
/// Generated client implementations.
pub mod tag_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct TagServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TagServiceClient<tonic::transport::Channel> {
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
    impl<T> TagServiceClient<T>
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
        ) -> TagServiceClient<InterceptedService<T, F>>
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
            TagServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// 创建 Tag
        pub async fn create_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTagRequest>,
        ) -> std::result::Result<tonic::Response<super::CreateTagReply>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/tag.TagService/CreateTag");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("tag.TagService", "CreateTag"));
            self.inner.unary(req, path, codec).await
        }
        /// 修改 Tag
        pub async fn edit_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::EditTagRequest>,
        ) -> std::result::Result<tonic::Response<super::EditTagReply>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/tag.TagService/EditTag");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("tag.TagService", "EditTag"));
            self.inner.unary(req, path, codec).await
        }
        /// Tag 列表
        pub async fn list_tags(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTagsRequest>,
        ) -> std::result::Result<tonic::Response<super::ListTagsReply>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/tag.TagService/ListTags");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("tag.TagService", "ListTags"));
            self.inner.unary(req, path, codec).await
        }
        /// 删除/恢复分类
        pub async fn toggle_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::ToggleTagRequest>,
        ) -> std::result::Result<tonic::Response<super::ToggleTagReply>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/tag.TagService/ToggleTag");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("tag.TagService", "ToggleTag"));
            self.inner.unary(req, path, codec).await
        }
        /// 检查分类是否存在
        pub async fn tag_exists(
            &mut self,
            request: impl tonic::IntoRequest<super::TagExistsRequest>,
        ) -> std::result::Result<tonic::Response<super::TagExistsReply>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/tag.TagService/TagExists");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("tag.TagService", "TagExists"));
            self.inner.unary(req, path, codec).await
        }
        /// 获取分类信息
        pub async fn get_tag_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTagInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTagInfoReply>,
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
                "/tag.TagService/GetTagInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("tag.TagService", "GetTagInfo"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod tag_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with TagServiceServer.
    #[async_trait]
    pub trait TagService: Send + Sync + 'static {
        /// 创建 Tag
        async fn create_tag(
            &self,
            request: tonic::Request<super::CreateTagRequest>,
        ) -> std::result::Result<tonic::Response<super::CreateTagReply>, tonic::Status>;
        /// 修改 Tag
        async fn edit_tag(
            &self,
            request: tonic::Request<super::EditTagRequest>,
        ) -> std::result::Result<tonic::Response<super::EditTagReply>, tonic::Status>;
        /// Tag 列表
        async fn list_tags(
            &self,
            request: tonic::Request<super::ListTagsRequest>,
        ) -> std::result::Result<tonic::Response<super::ListTagsReply>, tonic::Status>;
        /// 删除/恢复分类
        async fn toggle_tag(
            &self,
            request: tonic::Request<super::ToggleTagRequest>,
        ) -> std::result::Result<tonic::Response<super::ToggleTagReply>, tonic::Status>;
        /// 检查分类是否存在
        async fn tag_exists(
            &self,
            request: tonic::Request<super::TagExistsRequest>,
        ) -> std::result::Result<tonic::Response<super::TagExistsReply>, tonic::Status>;
        /// 获取分类信息
        async fn get_tag_info(
            &self,
            request: tonic::Request<super::GetTagInfoRequest>,
        ) -> std::result::Result<tonic::Response<super::GetTagInfoReply>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct TagServiceServer<T: TagService> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T: TagService> TagServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for TagServiceServer<T>
    where
        T: TagService,
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
            match req.uri().path() {
                "/tag.TagService/CreateTag" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTagSvc<T: TagService>(pub Arc<T>);
                    impl<
                        T: TagService,
                    > tonic::server::UnaryService<super::CreateTagRequest>
                    for CreateTagSvc<T> {
                        type Response = super::CreateTagReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateTagRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TagService>::create_tag(&inner, request).await
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
                        let method = CreateTagSvc(inner);
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
                "/tag.TagService/EditTag" => {
                    #[allow(non_camel_case_types)]
                    struct EditTagSvc<T: TagService>(pub Arc<T>);
                    impl<
                        T: TagService,
                    > tonic::server::UnaryService<super::EditTagRequest>
                    for EditTagSvc<T> {
                        type Response = super::EditTagReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EditTagRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TagService>::edit_tag(&inner, request).await
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
                        let method = EditTagSvc(inner);
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
                "/tag.TagService/ListTags" => {
                    #[allow(non_camel_case_types)]
                    struct ListTagsSvc<T: TagService>(pub Arc<T>);
                    impl<
                        T: TagService,
                    > tonic::server::UnaryService<super::ListTagsRequest>
                    for ListTagsSvc<T> {
                        type Response = super::ListTagsReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListTagsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TagService>::list_tags(&inner, request).await
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
                        let method = ListTagsSvc(inner);
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
                "/tag.TagService/ToggleTag" => {
                    #[allow(non_camel_case_types)]
                    struct ToggleTagSvc<T: TagService>(pub Arc<T>);
                    impl<
                        T: TagService,
                    > tonic::server::UnaryService<super::ToggleTagRequest>
                    for ToggleTagSvc<T> {
                        type Response = super::ToggleTagReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ToggleTagRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TagService>::toggle_tag(&inner, request).await
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
                        let method = ToggleTagSvc(inner);
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
                "/tag.TagService/TagExists" => {
                    #[allow(non_camel_case_types)]
                    struct TagExistsSvc<T: TagService>(pub Arc<T>);
                    impl<
                        T: TagService,
                    > tonic::server::UnaryService<super::TagExistsRequest>
                    for TagExistsSvc<T> {
                        type Response = super::TagExistsReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TagExistsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TagService>::tag_exists(&inner, request).await
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
                        let method = TagExistsSvc(inner);
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
                "/tag.TagService/GetTagInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetTagInfoSvc<T: TagService>(pub Arc<T>);
                    impl<
                        T: TagService,
                    > tonic::server::UnaryService<super::GetTagInfoRequest>
                    for GetTagInfoSvc<T> {
                        type Response = super::GetTagInfoReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTagInfoRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TagService>::get_tag_info(&inner, request).await
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
                        let method = GetTagInfoSvc(inner);
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
                                .header("grpc-status", tonic::Code::Unimplemented as i32)
                                .header(
                                    http::header::CONTENT_TYPE,
                                    tonic::metadata::GRPC_CONTENT_TYPE,
                                )
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: TagService> Clone for TagServiceServer<T> {
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
    impl<T: TagService> tonic::server::NamedService for TagServiceServer<T> {
        const NAME: &'static str = "tag.TagService";
    }
}
