use std::sync::Arc;

use futures::Future;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

use prost_bench::proto::complex_prost::{
    self, BatchCommandsRequest, BatchCommandsResponse, Complex,
};
use prost_bench::proto::simple_prost::{self, Data, Simple};

#[derive(Clone)]
struct Service {
    count: i64,
}

impl Simple for Service {
    fn foo(&mut self, ctx: RpcContext<'_>, req: Data, sink: UnarySink<Data>) {
        if req.payload[0] == 1 {
            println!("Handled {} requests", self.count);
            self.count = 0;
        } else {
            self.count += 1;
        }

        let f = sink.success(req).map_err(move |e| panic!("{:?}", e));
        ctx.spawn(f);
    }
}

impl Complex for Service {
    fn un(
        &mut self,
        ctx: RpcContext<'_>,
        req: BatchCommandsRequest,
        sink: UnarySink<BatchCommandsResponse>,
    ) {
        self.count += 1;

        let f = sink
            .success(BatchCommandsResponse::new_())
            .map_err(move |e| panic!("{:?}", e));
        ctx.spawn(f);
    }
}

fn main() {
    let env = Arc::new(Environment::new(1));
    let mut server = ServerBuilder::new(env)
        .register_service(simple_prost::create_simple(Service { count: 0 }))
        .register_service(complex_prost::create_complex(Service { count: 0 }))
        .bind("127.0.0.1", 50051)
        .build()
        .unwrap();
    server.start();
    loop {
        ::std::thread::yield_now();
    }
}
