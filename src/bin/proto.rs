use std::time::Instant;
use prost_bench::proto::{
    get_prost,
};
use prost::Message;

const COUNT: usize = 10000;

fn main() {
    let mut result = Vec::with_capacity(COUNT);
    let mut buf = Vec::with_capacity(8_000_000);
    let start = Instant::now();
    for i in 0..COUNT {
        let req = init_batch(i);
        req.encode(&mut buf).unwrap();
        let msg = get_prost::BatchCommandsRequest::decode(&buf).unwrap();
        result.push(msg.request_ids[0]);
    }
    let time = start.elapsed();

    println!(
        "Roundtripped {} messages, time: {}ms. {}",
        COUNT,
        time.as_millis(),
        result.len(),
    );

    if let Some(n) = get_resident() {
        let kb = n as f64 / 1_000.0;
        println!("rss: {}KB", kb.round() as usize)
    }
}

fn init_batch(i: usize) -> get_prost::BatchCommandsRequest {
    let mut req = get_prost::BatchCommandsRequest::default();
    req.request_ids.push(i as u64);
    req
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
