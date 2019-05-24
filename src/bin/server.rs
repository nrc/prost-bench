use std::sync::Arc;

use futures::{future, Future, Sink, Stream};
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink, RequestStream, DuplexSink, WriteFlags};

// use prost_bench::proto::complex_prost::{
//     self, BatchCommandsRequest, BatchCommandsResponse, Complex,
// };
use prost_bench::proto::simple_prost::{self, Data, Simple};
use prost_bench::proto::get_prost::{Tikv, BatchCommandsRequest, BatchCommandsResponse, RawGetRequest, RawGetResponse};

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

impl Tikv for Service {
    fn raw_get(&mut self, ctx: RpcContext<'_>, req: RawGetRequest, sink: UnarySink<RawGetResponse>) {
        self.count += 1;

        let f = sink
            .success(RawGetResponse::default())
            .map_err(move |e| panic!("{:?}", e));
        ctx.spawn(f);
    }

    fn batch_commands(&mut self, ctx: RpcContext<'_>, req: RequestStream<BatchCommandsRequest>, sink: DuplexSink<BatchCommandsResponse>) {
        let handler = req.for_each(move |mut req| {
            //self.count += 1;
            future::ok::<_, _>(())
        });

        ctx.spawn(handler.map_err(move |e| panic!("{:?}", e)));

        // TODO respond
    }
}

// impl Complex for Service {
//     fn un(
//         &mut self,
//         ctx: RpcContext<'_>,
//         req: BatchCommandsRequest,
//         sink: UnarySink<BatchCommandsResponse>,
//     ) {
//         self.count += 1;

//         let f = sink
//             .success(BatchCommandsResponse::new_())
//             .map_err(move |e| panic!("{:?}", e));
//         ctx.spawn(f);
//     }
// }

fn main() {
    let env = Arc::new(Environment::new(1));
    let mut server = ServerBuilder::new(env)
        .register_service(simple_prost::create_simple(Service { count: 0 }))
        //.register_service(complex_prost::create_complex(Service { count: 0 }))
        .bind("127.0.0.1", 50051)
        .build()
        .unwrap();
    server.start();
    loop {
        ::std::thread::yield_now();
    }
}
