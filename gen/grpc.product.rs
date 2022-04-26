#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProductRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub currency: i32,
    #[prost(double, tag="3")]
    pub price: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProductRequest {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag="3")]
    pub currency: ::core::option::Option<i32>,
    #[prost(double, optional, tag="4")]
    pub price: ::core::option::Option<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProductRequest {
    #[prost(uint64, tag="1")]
    pub id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProductResponse {
    #[prost(message, optional, tag="1")]
    pub product: ::core::option::Option<Product>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProductRequest {
    #[prost(string, optional, tag="1")]
    pub query: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag="2")]
    pub cursor: ::core::option::Option<u64>,
    #[prost(uint32, tag="3")]
    pub page_size: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProductResponse {
    #[prost(message, repeated, tag="1")]
    pub products: ::prost::alloc::vec::Vec<Product>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Product {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(int32, tag="3")]
    pub currency: i32,
    #[prost(double, tag="4")]
    pub price: f64,
    #[prost(uint64, tag="5")]
    pub created_at: u64,
    #[prost(uint64, optional, tag="6")]
    pub updated_at: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="7")]
    pub deleted_at: ::core::option::Option<u64>,
}
/// Generated server implementations.
pub mod product_services_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with ProductServicesServer.
    #[async_trait]
    pub trait ProductServices: Send + Sync + 'static {
        async fn create(
            &self,
            request: tonic::Request<super::CreateProductRequest>,
        ) -> Result<tonic::Response<super::Product>, tonic::Status>;
        async fn update(
            &self,
            request: tonic::Request<super::UpdateProductRequest>,
        ) -> Result<tonic::Response<super::Product>, tonic::Status>;
        async fn get(
            &self,
            request: tonic::Request<super::GetProductRequest>,
        ) -> Result<tonic::Response<super::GetProductResponse>, tonic::Status>;
        async fn list(
            &self,
            request: tonic::Request<super::ListProductRequest>,
        ) -> Result<tonic::Response<super::ListProductResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ProductServicesServer<T: ProductServices> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ProductServices> ProductServicesServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
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
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ProductServicesServer<T>
    where
        T: ProductServices,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/grpc.product.ProductServices/create" => {
                    #[allow(non_camel_case_types)]
                    struct createSvc<T: ProductServices>(pub Arc<T>);
                    impl<
                        T: ProductServices,
                    > tonic::server::UnaryService<super::CreateProductRequest>
                    for createSvc<T> {
                        type Response = super::Product;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateProductRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = createSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grpc.product.ProductServices/update" => {
                    #[allow(non_camel_case_types)]
                    struct updateSvc<T: ProductServices>(pub Arc<T>);
                    impl<
                        T: ProductServices,
                    > tonic::server::UnaryService<super::UpdateProductRequest>
                    for updateSvc<T> {
                        type Response = super::Product;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateProductRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = updateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grpc.product.ProductServices/get" => {
                    #[allow(non_camel_case_types)]
                    struct getSvc<T: ProductServices>(pub Arc<T>);
                    impl<
                        T: ProductServices,
                    > tonic::server::UnaryService<super::GetProductRequest>
                    for getSvc<T> {
                        type Response = super::GetProductResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetProductRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = getSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grpc.product.ProductServices/list" => {
                    #[allow(non_camel_case_types)]
                    struct listSvc<T: ProductServices>(pub Arc<T>);
                    impl<
                        T: ProductServices,
                    > tonic::server::UnaryService<super::ListProductRequest>
                    for listSvc<T> {
                        type Response = super::ListProductResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListProductRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = listSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
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
    impl<T: ProductServices> Clone for ProductServicesServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ProductServices> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ProductServices> tonic::transport::NamedService
    for ProductServicesServer<T> {
        const NAME: &'static str = "grpc.product.ProductServices";
    }
}
