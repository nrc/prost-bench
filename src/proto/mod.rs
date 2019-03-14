#![allow(dead_code)]

pub mod simple_prost {
    include!("prost/simple.rs");
    include!("prost/wrapper_simple.rs");
}

#[path = "protobuf/simple.rs"]
pub mod simple;
#[path = "protobuf/simple_grpc.rs"]
pub mod simple_grpc;
