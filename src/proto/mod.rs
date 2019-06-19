#![allow(dead_code)]

// pub mod simple_prost {
//     include!("prost/simple.rs");
//     include!("prost/wrapper_simple.rs");
// }

#[path = "protobuf/get.rs"]
pub mod get;
#[path = "protobuf/get_grpc.rs"]
pub mod get_grpc;


// pub mod complex_prost {
//     include!("prost/complex.rs");
//     include!("prost/wrapper_complex.rs");
// }

// #[path = "protobuf/complex.rs"]
// pub mod complex;
// #[path = "protobuf/complex_grpc.rs"]
// pub mod complex_grpc;

// pub mod get_prost {
//     include!("prost/get.rs");
//     include!("prost/wrapper_get.rs");
// }
