use std::sync::Arc;
use std::thread;

use futures::Future;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

use prost_bench::proto::simple_prost::{self, Simple, Data};

#[derive(Clone)]
struct Service {
    count: i64,
}

impl Simple for Service {
    fn foo(&mut self, ctx: RpcContext<'_>, req: Data, sink: UnarySink<Data>) {
        if req.payload[0] > 0 {
            println!("Handled {} requests", self.count);
            self.count = 0;
        } else {
            self.count += 1;
        }

        let f = sink
            .success(req)
            .map_err(move |e| panic!("{:?}", e));
        ctx.spawn(f);
    }
}

fn main() {
    let env = Arc::new(Environment::new(1));
    let service = simple_prost::create_simple(Service { count: 0 });
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 50_051)
        .build()
        .unwrap();
    server.start();
    loop {
        ::std::thread::yield_now();
    }
}
