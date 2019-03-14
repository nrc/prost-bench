#![allow(dead_code)]

use std::sync::Arc;
use std::time::Instant;
use futures::Future;
use futures::stream::FuturesUnordered;
use futures::stream::Stream;

use grpcio::{Channel, ChannelBuilder, EnvBuilder};
use prost_bench::proto::{simple_prost, simple_grpc, simple as simple_protobuf};

const MSGS: usize = 1_0_000;
const MSG_SIZE: usize = 32_000;

fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");

    let (req, client) = init_prost(ch);
    //let (req, client) = init_protobuf(ch);
    let mut buffer = FuturesUnordered::new();

    let start = Instant::now();
    for _ in 0..MSGS {
        let reply = client.foo_async(&req).expect("rpc");
        buffer.push(reply);
    }   
    buffer.wait().for_each(|r| { r.expect("future"); });
    let time = start.elapsed();

    terminate();

    println!("Sent {} messages, size: {}B, time: {}ms", MSGS, MSG_SIZE, time.as_millis());
}

#[inline]
fn init_prost(ch: Channel) -> (simple_prost::Data, simple_prost::SimpleClient) {
    let client = simple_prost::SimpleClient::new(ch);

    let mut req = simple_prost::Data::new_();
    req.payload = vec![0; MSG_SIZE];

    (req, client)
}

#[inline]
fn init_protobuf(ch: Channel) -> (simple_protobuf::Data, simple_grpc::SimpleClient) {
    let client = simple_grpc::SimpleClient::new(ch);

    let mut req = simple_protobuf::Data::new();
    req.set_payload(vec![0; MSG_SIZE]);

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
