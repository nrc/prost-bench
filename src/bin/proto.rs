use std::time::Instant;
use prost_bench::proto::{
    get_prost,
};
use prost::Message;

const COUNT: usize = 1000;

fn main() {
    //bench(init_batch_tiny, "tiny");
    bench(init_batch_medium, "medium");

    // if let Some(n) = get_resident() {
    //     let kb = n as f64 / 1_000.0;
    //     println!("rss: {}KB", kb.round() as usize)
    // }
}

fn bench(init: impl Fn(usize) -> get_prost::BatchCommandsRequest, name: &str) {
    let mut result = Vec::with_capacity(COUNT);
    let mut buf = Vec::with_capacity(8_000_000);
    let start = Instant::now();
    for i in 0..COUNT {
        let req = init(i);
        req.encode(&mut buf).unwrap();
        let msg = get_prost::BatchCommandsRequest::decode(&buf).unwrap();
        result.push(msg.request_ids[0]);
    }
    let time = start.elapsed();

    println!(
        "Roundtripped {} {} messages, time: {}ms. {}",
        COUNT,
        name,
        time.as_millis(),
        result.len(),
    );    
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
        key,
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
        priority: 42,
        isolation_level: 10,
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
