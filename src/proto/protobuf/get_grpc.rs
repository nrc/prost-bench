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

const METHOD_TIKV_RAW_GET: ::grpcio::Method<super::get::RawGetRequest, super::get::RawGetResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/get.Tikv/RawGet",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_BATCH_COMMANDS: ::grpcio::Method<super::get::BatchCommandsRequest, super::get::BatchCommandsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/get.Tikv/BatchCommands",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct TikvClient {
    client: ::grpcio::Client,
}

impl TikvClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        TikvClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn raw_get_opt(&self, req: &super::get::RawGetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::get::RawGetResponse> {
        self.client.unary_call(&METHOD_TIKV_RAW_GET, req, opt)
    }

    pub fn raw_get(&self, req: &super::get::RawGetRequest) -> ::grpcio::Result<super::get::RawGetResponse> {
        self.raw_get_opt(req, ::grpcio::CallOption::default())
    }

    pub fn raw_get_async_opt(&self, req: &super::get::RawGetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::get::RawGetResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_RAW_GET, req, opt)
    }

    pub fn raw_get_async(&self, req: &super::get::RawGetRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::get::RawGetResponse>> {
        self.raw_get_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn batch_commands_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::get::BatchCommandsRequest>, ::grpcio::ClientDuplexReceiver<super::get::BatchCommandsResponse>)> {
        self.client.duplex_streaming(&METHOD_TIKV_BATCH_COMMANDS, opt)
    }

    pub fn batch_commands(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::get::BatchCommandsRequest>, ::grpcio::ClientDuplexReceiver<super::get::BatchCommandsResponse>)> {
        self.batch_commands_opt(::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Tikv {
    fn raw_get(&mut self, ctx: ::grpcio::RpcContext, req: super::get::RawGetRequest, sink: ::grpcio::UnarySink<super::get::RawGetResponse>);
    fn batch_commands(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::get::BatchCommandsRequest>, sink: ::grpcio::DuplexSink<super::get::BatchCommandsResponse>);
}

pub fn create_tikv<S: Tikv + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_RAW_GET, move |ctx, req, resp| {
        instance.raw_get(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_duplex_streaming_handler(&METHOD_TIKV_BATCH_COMMANDS, move |ctx, req, resp| {
        instance.batch_commands(ctx, req, resp)
    });
    builder.build()
}
