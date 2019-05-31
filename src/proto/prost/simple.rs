#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
    #[prost(bytes, tag = "1")]
    pub payload: std::vec::Vec<u8>,
}
const METHOD_SIMPLE_FOO: ::grpcio::Method<Data, Data> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/simple.Simple/Foo",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
};
#[derive(Clone)]
pub struct SimpleClient {
    client: ::grpcio::Client,
}
impl SimpleClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        SimpleClient {
            client: ::grpcio::Client::new(channel),
        }
    }
    pub fn foo_opt(&self, req: &Data, opt: ::grpcio::CallOption) -> ::grpcio::Result<Data> {
        self.client.unary_call(&METHOD_SIMPLE_FOO, req, opt)
    }
    pub fn foo(&self, req: &Data) -> ::grpcio::Result<Data> {
        self.foo_opt(req, ::grpcio::CallOption::default())
    }
    pub fn foo_async_opt(
        &self,
        req: &Data,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<Data>> {
        self.client.unary_call_async(&METHOD_SIMPLE_FOO, req, opt)
    }
    pub fn foo_async(&self, req: &Data) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<Data>> {
        self.foo_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F)
    where
        F: ::futures::Future<Item = (), Error = ()> + Send + 'static,
    {
        self.client.spawn(f)
    }
}
pub trait Simple {
    fn foo(&mut self, ctx: ::grpcio::RpcContext, req: Data, sink: ::grpcio::UnarySink<Data>);
}
pub fn create_simple<S: Simple + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SIMPLE_FOO, move |ctx, req, resp| {
        instance.foo(ctx, req, resp)
    });
    builder.build()
}
