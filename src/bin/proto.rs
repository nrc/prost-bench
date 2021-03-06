use std::env;
use std::time::Instant;
use std::str::FromStr;
use std::thread;

use bytes::Bytes;
use prost_bench::proto::{
    get_prost,
};
use prost::{
//    BytesString,
    Message,
};

#[global_allocator] 
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

const THREADS: usize = 16;

fn main() {
    let mut args = env::args();
    let _ = args.next();
    if let Some(arg) = args.next() {
        if arg == "threads" {
            bench_with_threads(init_batch_medium, "medium", 1000);
            return;
        }
        if arg == "big" {
            bench(init_batch_medium, "medium", 10_000);
            return;
        }
        if arg == "tiny" {
            bench(init_batch_tiny, "tiny", 1000);
            return;
        }
    }

    bench(init_batch_medium, "medium", 1000);
    

    // if let Some(n) = get_resident() {
    //     let kb = n as f64 / 1_000.0;
    //     println!("rss: {}KB", kb.round() as usize)
    // }
}

fn bench_with_threads(init: impl Fn(usize) -> get_prost::BatchCommandsRequest + Send + 'static + Clone, name: &'static str, count: usize) {
    let mut handles = Vec::new();
    for _ in 0..THREADS {
        let init = init.clone();
        handles.push(thread::spawn(move || {
            bench(init, name, count);
        }));
    }
    handles.into_iter().for_each(|h| h.join().unwrap());
}

fn bench(init: impl Fn(usize) -> get_prost::BatchCommandsRequest, name: &str, count: usize) {
    let mut result = Vec::with_capacity(count);
    let mut buf = Vec::with_capacity(8_000_000);
    let start = Instant::now();
    for i in 0..count {
        let req = init(i);
        req.encode(&mut buf).unwrap();
        let len = buf.len();
        let msg = get_prost::BatchCommandsRequest::decode(&buf).unwrap();
        result.push(len);
    }
    let time = start.elapsed();

    // println!(
    //     "Roundtripped {} {} messages, time: {}ms. {}",
    //     count,
    //     name,
    //     time.as_millis(),
    //     result.len(),
    // );
    println!("{}", time.as_millis());
    println!("{:?}", result);
}

fn init_batch_tiny(i: usize) -> get_prost::BatchCommandsRequest {
    let mut req = get_prost::BatchCommandsRequest::default();
    req.request_ids.push(i as u64);
    req
}

fn init_batch_medium(i: usize) -> get_prost::BatchCommandsRequest {
    let mut req = get_prost::BatchCommandsRequest::default();
    for j in 0 .. 10 {
        req.request_ids.push(j * i as u64);
    }

    for j in 0 .. 10 {
        req.requests.push(init_request(i, j));
    }
    req
}

fn init_request(i: usize, j: usize) -> get_prost::batch_commands_request::Request {
    get_prost::batch_commands_request::Request {
        cmd: Some(get_prost::batch_commands_request::request::Cmd::RawGet(init_raw_get_request(i, j))),
    }
}

fn init_raw_get_request(i: usize, j: usize) -> get_prost::RawGetRequest {
    let mut key = b"usertable:user62837032397602".to_vec();
    key.push(j as u8);
    key.push((i * j) as u8);
    key.push((j * 20) as u8);
    get_prost::RawGetRequest {
        context: Some(init_context(i)),
//        key: Bytes::from(key),
        key,
//        cf: BytesString::from_str("Hello world!").unwrap(),
        cf: "Hello world!".to_owned(),
    }
}

fn init_context(i: usize) -> get_prost::Context {
    get_prost::Context {
        region_id: i as u64,
        region_epoch: Some(get_prost::RegionEpoch {
            conf_ver: 10,
            version: 3,
        }),
        peer: Some(get_prost::Peer {
            id: 342,
            store_id: 49328,
            is_learner: false,
        }),
        term: 458432964,
        priority: get_prost::CommandPri::Normal as i32,
        isolation_level: get_prost::IsolationLevel::Si as i32,
        not_fill_cache: false,
        sync_log: true,
        handle_time: true,
        scan_detail: false,
    }
}
// #[cfg(unix)]
// fn get_resident() -> Option<usize> {
//     use std::fs;

//     let field = 1;
//     let contents = fs::read("/proc/self/statm").ok()?;
//     let contents = String::from_utf8(contents).ok()?;
//     let s = contents.split_whitespace().nth(field)?;
//     let npages = s.parse::<usize>().ok()?;
//     Some(npages * 4096)
// }

// #[cfg(not(unix))]
// fn get_resident() -> Option<usize> {
//     None
// }
