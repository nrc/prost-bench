#![allow(dead_code)]

use futures::stream::FuturesUnordered;
use futures::stream::Stream;
use futures::Future;
use std::sync::Arc;
use std::time::Instant;

use grpcio::{Channel, ChannelBuilder, EnvBuilder};
use prost_bench::proto::{
    complex as complex_protobuf, complex_grpc, complex_prost, simple as simple_protobuf,
    simple_grpc, simple_prost,
};

const MSGS: usize = 1_0_000;
const MSG_SIZE: usize = 32_000;

fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");

    //let (req, client) = init_prost_simple(ch);
    //let (req, client) = init_protobuf_simple(ch);
    let (req, client) = init_prost_complex(ch);
    //let (req, client) = init_protobuf_complex(ch);
    let mut buffer = FuturesUnordered::new();

    let start = Instant::now();
    for _ in 0..MSGS {
        //let reply = client.foo_async(&req).expect("rpc");
        let reply = client.un_async(&req).expect("rpc");
        buffer.push(reply);
    }
    buffer.wait().for_each(|r| {
        r.expect("future");
    });
    let time = start.elapsed();

    terminate();

    println!(
        "Sent {} messages, size: {}B, time: {}ms",
        MSGS,
        MSG_SIZE,
        time.as_millis()
    );
}

#[inline]
fn init_prost_simple(ch: Channel) -> (simple_prost::Data, simple_prost::SimpleClient) {
    let client = simple_prost::SimpleClient::new(ch);

    let mut req = simple_prost::Data::new_();
    req.payload = vec![42; MSG_SIZE];

    (req, client)
}

#[inline]
fn init_protobuf_simple(ch: Channel) -> (simple_protobuf::Data, simple_grpc::SimpleClient) {
    let client = simple_grpc::SimpleClient::new(ch);

    let mut req = simple_protobuf::Data::new();
    req.set_payload(vec![42; MSG_SIZE]);

    (req, client)
}

#[inline]
fn init_prost_complex(
    ch: Channel,
) -> (
    complex_prost::BatchCommandsRequest,
    complex_prost::ComplexClient,
) {
    let client = complex_prost::ComplexClient::new(ch);

    let mut req = complex_prost::BatchCommandsRequest::new_();

    (req, client)
}

#[inline]
fn init_protobuf_complex(
    ch: Channel,
) -> (
    complex_protobuf::BatchCommandsRequest,
    complex_grpc::ComplexClient,
) {
    let client = complex_grpc::ComplexClient::new(ch);

    let mut req = complex_protobuf::BatchCommandsRequest::new();

    (req, client)
}

fn terminate() {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = simple_prost::SimpleClient::new(ch);
    let mut req = simple_prost::Data::new_();
    req.set_payload(vec![1]);
    let reply = client.foo_async(&req).expect("rpc");
    let _ = reply.wait().expect("future");
}
