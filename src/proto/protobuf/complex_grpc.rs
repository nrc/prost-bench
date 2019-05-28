// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_COMPLEX_UN: ::grpcio::Method<super::complex::BatchCommandsRequest, super::complex::BatchCommandsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/complex.Complex/un",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct ComplexClient {
    client: ::grpcio::Client,
}

impl ComplexClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ComplexClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn un_opt(&self, req: &super::complex::BatchCommandsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::complex::BatchCommandsResponse> {
        self.client.unary_call(&METHOD_COMPLEX_UN, req, opt)
    }

    pub fn un(&self, req: &super::complex::BatchCommandsRequest) -> ::grpcio::Result<super::complex::BatchCommandsResponse> {
        self.un_opt(req, ::grpcio::CallOption::default())
    }

    pub fn un_async_opt(&self, req: &super::complex::BatchCommandsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::complex::BatchCommandsResponse>> {
        self.client.unary_call_async(&METHOD_COMPLEX_UN, req, opt)
    }

    pub fn un_async(&self, req: &super::complex::BatchCommandsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::complex::BatchCommandsResponse>> {
        self.un_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Complex {
    fn un(&mut self, ctx: ::grpcio::RpcContext, req: super::complex::BatchCommandsRequest, sink: ::grpcio::UnarySink<super::complex::BatchCommandsResponse>);
}

pub fn create_complex<S: Complex + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_COMPLEX_UN, move |ctx, req, resp| {
        instance.un(ctx, req, resp)
    });
    builder.build()
}
