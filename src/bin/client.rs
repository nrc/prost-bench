#![allow(dead_code)]

use futures::stream::FuturesUnordered;
use futures::stream::Stream;
use futures::Future;
use std::default::Default;
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
    //let (req, client) = init_prost_complex(ch);
    let (req, client) = init_protobuf_complex(ch);
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

    if let Some(n) = get_resident() {
        let kb = n as f64 / 1_000.0;
        println!("rss: {}KB", kb.round() as usize)
    }
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
    req.requests = vec![
        complex_prost::batch_commands_request::Request {
            cmd: Some(
                complex_prost::batch_commands_request::request::Cmd::ResolveLock(
                    complex_prost::ResolveLockRequest {
                        context: Some(complex_prost::Context {
                            region_id: 332428498988989080,
                            region_epoch: None,
                            peer: None,
                            term: 325235345234,
                            priority: 34234,
                            isolation_level: 342342,
                            not_fill_cache: true,
                            sync_log: true,
                            handle_time: true,
                            scan_detail: true,
                        }),
                        start_version: 24,
                        commit_version: 324,
                        txn_infos: vec![complex_prost::TxnInfo {
                            txn: 42,
                            status: 53,
                        }],
                    },
                ),
            ),
        },
        complex_prost::batch_commands_request::Request {
            cmd: Some(complex_prost::batch_commands_request::request::Cmd::Scan(
                complex_prost::ScanRequest {
                    context: Some(complex_prost::Context {
                        region_id: 332428498988989080,
                        region_epoch: None,
                        peer: None,
                        term: 325235345234,
                        priority: 34234,
                        isolation_level: 342342,
                        not_fill_cache: true,
                        sync_log: true,
                        handle_time: true,
                        scan_detail: true,
                    }),
                    start_key: vec![1, 3, 5, 76, 0, 2, 84],
                    limit: 324,
                    version: 98908098024,
                    key_only: false,
                    reverse: true,
                    end_key: vec![1, 3, 5, 76, 0, 2, 84],
                },
            )),
        },
        complex_prost::batch_commands_request::Request {
            cmd: Some(
                complex_prost::batch_commands_request::request::Cmd::Prewrite(
                    complex_prost::PrewriteRequest {
                        context: Some(complex_prost::Context {
                            region_id: 332428498988989080,
                            region_epoch: None,
                            peer: None,
                            term: 325235345234,
                            priority: 34234,
                            isolation_level: 342342,
                            not_fill_cache: true,
                            sync_log: true,
                            handle_time: true,
                            scan_detail: true,
                        }),
                        mutations: vec![complex_prost::Mutation {
                            op: 42,
                            key: vec![1, 3, 5, 76, 0, 2, 84],
                            value: vec![1, 3, 5, 76, 0, 2, 84],
                            assertion: 21,
                        }],
                        primary_lock: vec![24, 1, 3, 5, 76, 0, 2, 84],
                        start_version: 24,
                        lock_ttl: 324,
                        skip_constraint_check: true,
                    },
                ),
            ),
        },
        complex_prost::batch_commands_request::Request {
            cmd: Some(
                complex_prost::batch_commands_request::request::Cmd::ResolveLock(
                    complex_prost::ResolveLockRequest {
                        context: Some(complex_prost::Context {
                            region_id: 332428498988989080,
                            region_epoch: None,
                            peer: None,
                            term: 325235345234,
                            priority: 34234,
                            isolation_level: 342342,
                            not_fill_cache: true,
                            sync_log: true,
                            handle_time: true,
                            scan_detail: true,
                        }),
                        start_version: 24,
                        commit_version: 324,
                        txn_infos: vec![complex_prost::TxnInfo {
                            txn: 42,
                            status: 53,
                        }],
                    },
                ),
            ),
        },
        complex_prost::batch_commands_request::Request {
            cmd: Some(complex_prost::batch_commands_request::request::Cmd::Scan(
                complex_prost::ScanRequest {
                    context: Some(complex_prost::Context {
                        region_id: 332428498988989080,
                        region_epoch: None,
                        peer: None,
                        term: 325235345234,
                        priority: 34234,
                        isolation_level: 342342,
                        not_fill_cache: true,
                        sync_log: true,
                        handle_time: true,
                        scan_detail: true,
                    }),
                    start_key: vec![1, 3, 5, 76, 0, 2, 84],
                    limit: 324,
                    version: 98908098024,
                    key_only: false,
                    reverse: true,
                    end_key: vec![1, 3, 5, 76, 0, 2, 84],
                },
            )),
        },
        complex_prost::batch_commands_request::Request {
            cmd: Some(
                complex_prost::batch_commands_request::request::Cmd::Prewrite(
                    complex_prost::PrewriteRequest {
                        context: Some(complex_prost::Context {
                            region_id: 332428498988989080,
                            region_epoch: None,
                            peer: None,
                            term: 325235345234,
                            priority: 34234,
                            isolation_level: 342342,
                            not_fill_cache: true,
                            sync_log: true,
                            handle_time: true,
                            scan_detail: true,
                        }),
                        mutations: vec![complex_prost::Mutation {
                            op: 42,
                            key: vec![1, 3, 5, 76, 0, 2, 84],
                            value: vec![1, 3, 5, 76, 0, 2, 84],
                            assertion: 21,
                        }],
                        primary_lock: vec![24, 1, 3, 5, 76, 0, 2, 84],
                        start_version: 24,
                        lock_ttl: 324,
                        skip_constraint_check: true,
                    },
                ),
            ),
        },
        complex_prost::batch_commands_request::Request {
            cmd: Some(
                complex_prost::batch_commands_request::request::Cmd::ResolveLock(
                    complex_prost::ResolveLockRequest {
                        context: Some(complex_prost::Context {
                            region_id: 332428498988989080,
                            region_epoch: None,
                            peer: None,
                            term: 325235345234,
                            priority: 34234,
                            isolation_level: 342342,
                            not_fill_cache: true,
                            sync_log: true,
                            handle_time: true,
                            scan_detail: true,
                        }),
                        start_version: 24,
                        commit_version: 324,
                        txn_infos: vec![complex_prost::TxnInfo {
                            txn: 42,
                            status: 53,
                        }],
                    },
                ),
            ),
        },
        complex_prost::batch_commands_request::Request {
            cmd: Some(complex_prost::batch_commands_request::request::Cmd::Scan(
                complex_prost::ScanRequest {
                    context: Some(complex_prost::Context {
                        region_id: 332428498988989080,
                        region_epoch: None,
                        peer: None,
                        term: 325235345234,
                        priority: 34234,
                        isolation_level: 342342,
                        not_fill_cache: true,
                        sync_log: true,
                        handle_time: true,
                        scan_detail: true,
                    }),
                    start_key: vec![1, 3, 5, 76, 0, 2, 84],
                    limit: 324,
                    version: 98908098024,
                    key_only: false,
                    reverse: true,
                    end_key: vec![1, 3, 5, 76, 0, 2, 84],
                },
            )),
        },
        complex_prost::batch_commands_request::Request {
            cmd: Some(
                complex_prost::batch_commands_request::request::Cmd::Prewrite(
                    complex_prost::PrewriteRequest {
                        context: Some(complex_prost::Context {
                            region_id: 332428498988989080,
                            region_epoch: None,
                            peer: None,
                            term: 325235345234,
                            priority: 34234,
                            isolation_level: 342342,
                            not_fill_cache: true,
                            sync_log: true,
                            handle_time: true,
                            scan_detail: true,
                        }),
                        mutations: vec![complex_prost::Mutation {
                            op: 42,
                            key: vec![1, 3, 5, 76, 0, 2, 84],
                            value: vec![1, 3, 5, 76, 0, 2, 84],
                            assertion: 21,
                        }],
                        primary_lock: vec![24, 1, 3, 5, 76, 0, 2, 84],
                        start_version: 24,
                        lock_ttl: 324,
                        skip_constraint_check: true,
                    },
                ),
            ),
        },
        complex_prost::batch_commands_request::Request {
            cmd: Some(
                complex_prost::batch_commands_request::request::Cmd::ResolveLock(
                    complex_prost::ResolveLockRequest {
                        context: Some(complex_prost::Context {
                            region_id: 332428498988989080,
                            region_epoch: None,
                            peer: None,
                            term: 325235345234,
                            priority: 34234,
                            isolation_level: 342342,
                            not_fill_cache: true,
                            sync_log: true,
                            handle_time: true,
                            scan_detail: true,
                        }),
                        start_version: 24,
                        commit_version: 324,
                        txn_infos: vec![complex_prost::TxnInfo {
                            txn: 42,
                            status: 53,
                        }],
                    },
                ),
            ),
        },
        complex_prost::batch_commands_request::Request {
            cmd: Some(complex_prost::batch_commands_request::request::Cmd::Scan(
                complex_prost::ScanRequest {
                    context: Some(complex_prost::Context {
                        region_id: 332428498988989080,
                        region_epoch: None,
                        peer: None,
                        term: 325235345234,
                        priority: 34234,
                        isolation_level: 342342,
                        not_fill_cache: true,
                        sync_log: true,
                        handle_time: true,
                        scan_detail: true,
                    }),
                    start_key: vec![1, 3, 5, 76, 0, 2, 84],
                    limit: 324,
                    version: 98908098024,
                    key_only: false,
                    reverse: true,
                    end_key: vec![1, 3, 5, 76, 0, 2, 84],
                },
            )),
        },
        complex_prost::batch_commands_request::Request {
            cmd: Some(
                complex_prost::batch_commands_request::request::Cmd::Prewrite(
                    complex_prost::PrewriteRequest {
                        context: Some(complex_prost::Context {
                            region_id: 332428498988989080,
                            region_epoch: None,
                            peer: None,
                            term: 325235345234,
                            priority: 34234,
                            isolation_level: 342342,
                            not_fill_cache: true,
                            sync_log: true,
                            handle_time: true,
                            scan_detail: true,
                        }),
                        mutations: vec![complex_prost::Mutation {
                            op: 42,
                            key: vec![1, 3, 5, 76, 0, 2, 84],
                            value: vec![1, 3, 5, 76, 0, 2, 84],
                            assertion: 21,
                        }],
                        primary_lock: vec![24, 1, 3, 5, 76, 0, 2, 84],
                        start_version: 24,
                        lock_ttl: 324,
                        skip_constraint_check: true,
                    },
                ),
            ),
        },
    ];
    req.request_ids = vec![63, 64, 21, 123132123, 3234235234423];

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
    req.requests = vec![
        complex_protobuf::BatchCommandsRequest_Request {
            cmd: Some(
                complex_protobuf::BatchCommandsRequest_Request_oneof_cmd::ResolveLock(
                    complex_protobuf::ResolveLockRequest {
                        context: Some(complex_protobuf::Context {
                            region_id: 332428498988989080,
                            region_epoch: None.into(),
                            peer: None.into(),
                            term: 325235345234,
                            priority: complex_protobuf::CommandPri::High,
                            isolation_level: complex_protobuf::IsolationLevel::RC,
                            not_fill_cache: true,
                            sync_log: true,
                            handle_time: true,
                            scan_detail: true,
                            unknown_fields: ::protobuf::UnknownFields::default(),
                            cached_size: ::protobuf::CachedSize::default(),
                        })
                        .into(),
                        start_version: 24,
                        commit_version: 324,
                        txn_infos: vec![complex_protobuf::TxnInfo {
                            txn: 42,
                            status: 53,
                            unknown_fields: ::protobuf::UnknownFields::default(),
                            cached_size: ::protobuf::CachedSize::default(),
                        }]
                        .into(),
                        unknown_fields: ::protobuf::UnknownFields::default(),
                        cached_size: ::protobuf::CachedSize::default(),
                    },
                ),
            )
            .into(),
            unknown_fields: ::protobuf::UnknownFields::default(),
            cached_size: ::protobuf::CachedSize::default(),
        },
        complex_protobuf::BatchCommandsRequest_Request {
            cmd: Some(
                complex_protobuf::BatchCommandsRequest_Request_oneof_cmd::Scan(
                    complex_protobuf::ScanRequest {
                        context: Some(complex_protobuf::Context {
                            region_id: 332428498988989080,
                            region_epoch: None.into(),
                            peer: None.into(),
                            term: 325235345234,
                            priority: complex_protobuf::CommandPri::High,
                            isolation_level: complex_protobuf::IsolationLevel::RC,
                            not_fill_cache: true,
                            sync_log: true,
                            handle_time: true,
                            scan_detail: true,
                            unknown_fields: ::protobuf::UnknownFields::default(),
                            cached_size: ::protobuf::CachedSize::default(),
                        })
                        .into(),
                        start_key: vec![1, 3, 5, 76, 0, 2, 84].into(),
                        limit: 324,
                        version: 98908098024,
                        key_only: false,
                        reverse: true,
                        end_key: vec![1, 3, 5, 76, 0, 2, 84].into(),
                        unknown_fields: ::protobuf::UnknownFields::default(),
                        cached_size: ::protobuf::CachedSize::default(),
                    },
                ),
            ),
            unknown_fields: ::protobuf::UnknownFields::default(),
            cached_size: ::protobuf::CachedSize::default(),
        },
        complex_protobuf::BatchCommandsRequest_Request {
            cmd: Some(
                complex_protobuf::BatchCommandsRequest_Request_oneof_cmd::Prewrite(
                    complex_protobuf::PrewriteRequest {
                        context: Some(complex_protobuf::Context {
                            region_id: 332428498988989080,
                            region_epoch: None.into(),
                            peer: None.into(),
                            term: 325235345234,
                            priority: complex_protobuf::CommandPri::High,
                            isolation_level: complex_protobuf::IsolationLevel::RC,
                            not_fill_cache: true,
                            sync_log: true,
                            handle_time: true,
                            scan_detail: true,
                            unknown_fields: ::protobuf::UnknownFields::default(),
                            cached_size: ::protobuf::CachedSize::default(),
                        })
                        .into(),
                        mutations: vec![complex_protobuf::Mutation {
                            op: complex_protobuf::Op::Rollback,
                            key: vec![1, 3, 5, 76, 0, 2, 84].into(),
                            value: vec![1, 3, 5, 76, 0, 2, 84].into(),
                            assertion: complex_protobuf::Assertion::NotExist,
                            unknown_fields: ::protobuf::UnknownFields::default(),
                            cached_size: ::protobuf::CachedSize::default(),
                        }]
                        .into(),
                        primary_lock: vec![24, 1, 3, 5, 76, 0, 2, 84].into(),
                        start_version: 24,
                        lock_ttl: 324,
                        skip_constraint_check: true,
                        unknown_fields: ::protobuf::UnknownFields::default(),
                        cached_size: ::protobuf::CachedSize::default(),
                    },
                ),
            ),
            unknown_fields: ::protobuf::UnknownFields::default(),
            cached_size: ::protobuf::CachedSize::default(),
        },
        complex_protobuf::BatchCommandsRequest_Request {
            cmd: Some(
                complex_protobuf::BatchCommandsRequest_Request_oneof_cmd::ResolveLock(
                    complex_protobuf::ResolveLockRequest {
                        context: Some(complex_protobuf::Context {
                            region_id: 332428498988989080,
                            region_epoch: None.into(),
                            peer: None.into(),
                            term: 325235345234,
                            priority: complex_protobuf::CommandPri::High,
                            isolation_level: complex_protobuf::IsolationLevel::RC,
                            not_fill_cache: true,
                            sync_log: true,
                            handle_time: true,
                            scan_detail: true,
                            unknown_fields: ::protobuf::UnknownFields::default(),
                            cached_size: ::protobuf::CachedSize::default(),
                        })
                        .into(),
                        start_version: 24,
                        commit_version: 324,
                        txn_infos: vec![complex_protobuf::TxnInfo {
                            txn: 42,
                            status: 53,
                            unknown_fields: ::protobuf::UnknownFields::default(),
                            cached_size: ::protobuf::CachedSize::default(),
                        }]
                        .into(),
                        unknown_fields: ::protobuf::UnknownFields::default(),
                        cached_size: ::protobuf::CachedSize::default(),
                    },
                ),
            )
            .into(),
            unknown_fields: ::protobuf::UnknownFields::default(),
            cached_size: ::protobuf::CachedSize::default(),
        },
        complex_protobuf::BatchCommandsRequest_Request {
            cmd: Some(
                complex_protobuf::BatchCommandsRequest_Request_oneof_cmd::Scan(
                    complex_protobuf::ScanRequest {
                        context: Some(complex_protobuf::Context {
                            region_id: 332428498988989080,
                            region_epoch: None.into(),
                            peer: None.into(),
                            term: 325235345234,
                            priority: complex_protobuf::CommandPri::High,
                            isolation_level: complex_protobuf::IsolationLevel::RC,
                            not_fill_cache: true,
                            sync_log: true,
                            handle_time: true,
                            scan_detail: true,
                            unknown_fields: ::protobuf::UnknownFields::default(),
                            cached_size: ::protobuf::CachedSize::default(),
                        })
                        .into(),
                        start_key: vec![1, 3, 5, 76, 0, 2, 84].into(),
                        limit: 324,
                        version: 98908098024,
                        key_only: false,
                        reverse: true,
                        end_key: vec![1, 3, 5, 76, 0, 2, 84].into(),
                        unknown_fields: ::protobuf::UnknownFields::default(),
                        cached_size: ::protobuf::CachedSize::default(),
                    },
                ),
            ),
            unknown_fields: ::protobuf::UnknownFields::default(),
            cached_size: ::protobuf::CachedSize::default(),
        },
        complex_protobuf::BatchCommandsRequest_Request {
            cmd: Some(
                complex_protobuf::BatchCommandsRequest_Request_oneof_cmd::Prewrite(
                    complex_protobuf::PrewriteRequest {
                        context: Some(complex_protobuf::Context {
                            region_id: 332428498988989080,
                            region_epoch: None.into(),
                            peer: None.into(),
                            term: 325235345234,
                            priority: complex_protobuf::CommandPri::High,
                            isolation_level: complex_protobuf::IsolationLevel::RC,
                            not_fill_cache: true,
                            sync_log: true,
                            handle_time: true,
                            scan_detail: true,
                            unknown_fields: ::protobuf::UnknownFields::default(),
                            cached_size: ::protobuf::CachedSize::default(),
                        })
                        .into(),
                        mutations: vec![complex_protobuf::Mutation {
                            op: complex_protobuf::Op::Rollback,
                            key: vec![1, 3, 5, 76, 0, 2, 84].into(),
                            value: vec![1, 3, 5, 76, 0, 2, 84].into(),
                            assertion: complex_protobuf::Assertion::NotExist,
                            unknown_fields: ::protobuf::UnknownFields::default(),
                            cached_size: ::protobuf::CachedSize::default(),
                        }]
                        .into(),
                        primary_lock: vec![24, 1, 3, 5, 76, 0, 2, 84].into(),
                        start_version: 24,
                        lock_ttl: 324,
                        skip_constraint_check: true,
                        unknown_fields: ::protobuf::UnknownFields::default(),
                        cached_size: ::protobuf::CachedSize::default(),
                    },
                ),
            ),
            unknown_fields: ::protobuf::UnknownFields::default(),
            cached_size: ::protobuf::CachedSize::default(),
        },
        complex_protobuf::BatchCommandsRequest_Request {
            cmd: Some(
                complex_protobuf::BatchCommandsRequest_Request_oneof_cmd::ResolveLock(
                    complex_protobuf::ResolveLockRequest {
                        context: Some(complex_protobuf::Context {
                            region_id: 332428498988989080,
                            region_epoch: None.into(),
                            peer: None.into(),
                            term: 325235345234,
                            priority: complex_protobuf::CommandPri::High,
                            isolation_level: complex_protobuf::IsolationLevel::RC,
                            not_fill_cache: true,
                            sync_log: true,
                            handle_time: true,
                            scan_detail: true,
                            unknown_fields: ::protobuf::UnknownFields::default(),
                            cached_size: ::protobuf::CachedSize::default(),
                        })
                        .into(),
                        start_version: 24,
                        commit_version: 324,
                        txn_infos: vec![complex_protobuf::TxnInfo {
                            txn: 42,
                            status: 53,
                            unknown_fields: ::protobuf::UnknownFields::default(),
                            cached_size: ::protobuf::CachedSize::default(),
                        }]
                        .into(),
                        unknown_fields: ::protobuf::UnknownFields::default(),
                        cached_size: ::protobuf::CachedSize::default(),
                    },
                ),
            )
            .into(),
            unknown_fields: ::protobuf::UnknownFields::default(),
            cached_size: ::protobuf::CachedSize::default(),
        },
        complex_protobuf::BatchCommandsRequest_Request {
            cmd: Some(
                complex_protobuf::BatchCommandsRequest_Request_oneof_cmd::Scan(
                    complex_protobuf::ScanRequest {
                        context: Some(complex_protobuf::Context {
                            region_id: 332428498988989080,
                            region_epoch: None.into(),
                            peer: None.into(),
                            term: 325235345234,
                            priority: complex_protobuf::CommandPri::High,
                            isolation_level: complex_protobuf::IsolationLevel::RC,
                            not_fill_cache: true,
                            sync_log: true,
                            handle_time: true,
                            scan_detail: true,
                            unknown_fields: ::protobuf::UnknownFields::default(),
                            cached_size: ::protobuf::CachedSize::default(),
                        })
                        .into(),
                        start_key: vec![1, 3, 5, 76, 0, 2, 84].into(),
                        limit: 324,
                        version: 98908098024,
                        key_only: false,
                        reverse: true,
                        end_key: vec![1, 3, 5, 76, 0, 2, 84].into(),
                        unknown_fields: ::protobuf::UnknownFields::default(),
                        cached_size: ::protobuf::CachedSize::default(),
                    },
                ),
            ),
            unknown_fields: ::protobuf::UnknownFields::default(),
            cached_size: ::protobuf::CachedSize::default(),
        },
        complex_protobuf::BatchCommandsRequest_Request {
            cmd: Some(
                complex_protobuf::BatchCommandsRequest_Request_oneof_cmd::Prewrite(
                    complex_protobuf::PrewriteRequest {
                        context: Some(complex_protobuf::Context {
                            region_id: 332428498988989080,
                            region_epoch: None.into(),
                            peer: None.into(),
                            term: 325235345234,
                            priority: complex_protobuf::CommandPri::High,
                            isolation_level: complex_protobuf::IsolationLevel::RC,
                            not_fill_cache: true,
                            sync_log: true,
                            handle_time: true,
                            scan_detail: true,
                            unknown_fields: ::protobuf::UnknownFields::default(),
                            cached_size: ::protobuf::CachedSize::default(),
                        })
                        .into(),
                        mutations: vec![complex_protobuf::Mutation {
                            op: complex_protobuf::Op::Rollback,
                            key: vec![1, 3, 5, 76, 0, 2, 84].into(),
                            value: vec![1, 3, 5, 76, 0, 2, 84].into(),
                            assertion: complex_protobuf::Assertion::NotExist,
                            unknown_fields: ::protobuf::UnknownFields::default(),
                            cached_size: ::protobuf::CachedSize::default(),
                        }]
                        .into(),
                        primary_lock: vec![24, 1, 3, 5, 76, 0, 2, 84].into(),
                        start_version: 24,
                        lock_ttl: 324,
                        skip_constraint_check: true,
                        unknown_fields: ::protobuf::UnknownFields::default(),
                        cached_size: ::protobuf::CachedSize::default(),
                    },
                ),
            ),
            unknown_fields: ::protobuf::UnknownFields::default(),
            cached_size: ::protobuf::CachedSize::default(),
        },
        complex_protobuf::BatchCommandsRequest_Request {
            cmd: Some(
                complex_protobuf::BatchCommandsRequest_Request_oneof_cmd::ResolveLock(
                    complex_protobuf::ResolveLockRequest {
                        context: Some(complex_protobuf::Context {
                            region_id: 332428498988989080,
                            region_epoch: None.into(),
                            peer: None.into(),
                            term: 325235345234,
                            priority: complex_protobuf::CommandPri::High,
                            isolation_level: complex_protobuf::IsolationLevel::RC,
                            not_fill_cache: true,
                            sync_log: true,
                            handle_time: true,
                            scan_detail: true,
                            unknown_fields: ::protobuf::UnknownFields::default(),
                            cached_size: ::protobuf::CachedSize::default(),
                        })
                        .into(),
                        start_version: 24,
                        commit_version: 324,
                        txn_infos: vec![complex_protobuf::TxnInfo {
                            txn: 42,
                            status: 53,
                            unknown_fields: ::protobuf::UnknownFields::default(),
                            cached_size: ::protobuf::CachedSize::default(),
                        }]
                        .into(),
                        unknown_fields: ::protobuf::UnknownFields::default(),
                        cached_size: ::protobuf::CachedSize::default(),
                    },
                ),
            )
            .into(),
            unknown_fields: ::protobuf::UnknownFields::default(),
            cached_size: ::protobuf::CachedSize::default(),
        },
        complex_protobuf::BatchCommandsRequest_Request {
            cmd: Some(
                complex_protobuf::BatchCommandsRequest_Request_oneof_cmd::Scan(
                    complex_protobuf::ScanRequest {
                        context: Some(complex_protobuf::Context {
                            region_id: 332428498988989080,
                            region_epoch: None.into(),
                            peer: None.into(),
                            term: 325235345234,
                            priority: complex_protobuf::CommandPri::High,
                            isolation_level: complex_protobuf::IsolationLevel::RC,
                            not_fill_cache: true,
                            sync_log: true,
                            handle_time: true,
                            scan_detail: true,
                            unknown_fields: ::protobuf::UnknownFields::default(),
                            cached_size: ::protobuf::CachedSize::default(),
                        })
                        .into(),
                        start_key: vec![1, 3, 5, 76, 0, 2, 84].into(),
                        limit: 324,
                        version: 98908098024,
                        key_only: false,
                        reverse: true,
                        end_key: vec![1, 3, 5, 76, 0, 2, 84].into(),
                        unknown_fields: ::protobuf::UnknownFields::default(),
                        cached_size: ::protobuf::CachedSize::default(),
                    },
                ),
            ),
            unknown_fields: ::protobuf::UnknownFields::default(),
            cached_size: ::protobuf::CachedSize::default(),
        },
        complex_protobuf::BatchCommandsRequest_Request {
            cmd: Some(
                complex_protobuf::BatchCommandsRequest_Request_oneof_cmd::Prewrite(
                    complex_protobuf::PrewriteRequest {
                        context: Some(complex_protobuf::Context {
                            region_id: 332428498988989080,
                            region_epoch: None.into(),
                            peer: None.into(),
                            term: 325235345234,
                            priority: complex_protobuf::CommandPri::High,
                            isolation_level: complex_protobuf::IsolationLevel::RC,
                            not_fill_cache: true,
                            sync_log: true,
                            handle_time: true,
                            scan_detail: true,
                            unknown_fields: ::protobuf::UnknownFields::default(),
                            cached_size: ::protobuf::CachedSize::default(),
                        })
                        .into(),
                        mutations: vec![complex_protobuf::Mutation {
                            op: complex_protobuf::Op::Rollback,
                            key: vec![1, 3, 5, 76, 0, 2, 84].into(),
                            value: vec![1, 3, 5, 76, 0, 2, 84].into(),
                            assertion: complex_protobuf::Assertion::NotExist,
                            unknown_fields: ::protobuf::UnknownFields::default(),
                            cached_size: ::protobuf::CachedSize::default(),
                        }]
                        .into(),
                        primary_lock: vec![24, 1, 3, 5, 76, 0, 2, 84].into(),
                        start_version: 24,
                        lock_ttl: 324,
                        skip_constraint_check: true,
                        unknown_fields: ::protobuf::UnknownFields::default(),
                        cached_size: ::protobuf::CachedSize::default(),
                    },
                ),
            ),
            unknown_fields: ::protobuf::UnknownFields::default(),
            cached_size: ::protobuf::CachedSize::default(),
        },
    ]
    .into();

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

#[cfg(unix)]
fn get_resident() -> Option<usize> {
    use std::fs;

    let field = 1;
    let contents = fs::read("/proc/self/statm").ok()?;
    let contents = String::from_utf8(contents).ok()?;
    let s = contents.split_whitespace().nth(field)?;
    let npages = s.parse::<usize>().ok()?;
    Some(npages * 4096)
}

#[cfg(not(unix))]
fn get_resident() -> Option<usize> {
    None
}
