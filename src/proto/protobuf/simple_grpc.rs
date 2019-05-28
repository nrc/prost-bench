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

const METHOD_SIMPLE_FOO: ::grpcio::Method<super::simple::Data, super::simple::Data> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/simple.Simple/Foo",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct SimpleClient {
    client: ::grpcio::Client,
}

impl SimpleClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        SimpleClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn foo_opt(&self, req: &super::simple::Data, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::simple::Data> {
        self.client.unary_call(&METHOD_SIMPLE_FOO, req, opt)
    }

    pub fn foo(&self, req: &super::simple::Data) -> ::grpcio::Result<super::simple::Data> {
        self.foo_opt(req, ::grpcio::CallOption::default())
    }

    pub fn foo_async_opt(&self, req: &super::simple::Data, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::simple::Data>> {
        self.client.unary_call_async(&METHOD_SIMPLE_FOO, req, opt)
    }

    pub fn foo_async(&self, req: &super::simple::Data) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::simple::Data>> {
        self.foo_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Simple {
    fn foo(&mut self, ctx: ::grpcio::RpcContext, req: super::simple::Data, sink: ::grpcio::UnarySink<super::simple::Data>);
}

pub fn create_simple<S: Simple + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_SIMPLE_FOO, move |ctx, req, resp| {
        instance.foo(ctx, req, resp)
    });
    builder.build()
}
