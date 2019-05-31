#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCommandsRequest {
    #[prost(message, repeated, tag = "1")]
    pub requests: ::std::vec::Vec<batch_commands_request::Request>,
    #[prost(uint64, repeated, tag = "2")]
    pub request_ids: ::std::vec::Vec<u64>,
}
pub mod batch_commands_request {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Request {
        #[prost(oneof = "request::Cmd", tags = "13")]
        pub cmd: ::std::option::Option<request::Cmd>,
    }
    pub mod request {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Cmd {
            #[prost(message, tag = "13")]
            RawGet(super::super::RawGetRequest),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCommandsResponse {
    #[prost(message, repeated, tag = "1")]
    pub responses: ::std::vec::Vec<batch_commands_response::Response>,
    #[prost(uint64, repeated, tag = "2")]
    pub request_ids: ::std::vec::Vec<u64>,
}
pub mod batch_commands_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(oneof = "response::Cmd", tags = "13")]
        pub cmd: ::std::option::Option<response::Cmd>,
    }
    pub mod response {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Cmd {
            #[prost(message, tag = "13")]
            RawGet(super::super::RawGetResponse),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawGetRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::std::option::Option<Context>,
    #[prost(bytes, tag = "2")]
    pub key: std::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub cf: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawGetResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::std::option::Option<Error>,
    #[prost(string, tag = "2")]
    pub error: std::string::String,
    #[prost(bytes, tag = "3")]
    pub value: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Context {
    #[prost(uint64, tag = "1")]
    pub region_id: u64,
    #[prost(message, optional, tag = "2")]
    pub region_epoch: ::std::option::Option<RegionEpoch>,
    #[prost(message, optional, tag = "3")]
    pub peer: ::std::option::Option<Peer>,
    #[prost(uint64, tag = "5")]
    pub term: u64,
    #[prost(enumeration = "CommandPri", tag = "6")]
    pub priority: i32,
    #[prost(enumeration = "IsolationLevel", tag = "7")]
    pub isolation_level: i32,
    #[prost(bool, tag = "8")]
    pub not_fill_cache: bool,
    #[prost(bool, tag = "9")]
    pub sync_log: bool,
    #[prost(bool, tag = "10")]
    pub handle_time: bool,
    #[prost(bool, tag = "11")]
    pub scan_detail: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionEpoch {
    #[prost(uint64, tag = "1")]
    pub conf_ver: u64,
    #[prost(uint64, tag = "2")]
    pub version: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Peer {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, tag = "2")]
    pub store_id: u64,
    #[prost(bool, tag = "3")]
    pub is_learner: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    #[prost(string, tag = "1")]
    pub message: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub not_leader: ::std::option::Option<NotLeader>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotLeader {
    #[prost(uint64, tag = "1")]
    pub region_id: u64,
    #[prost(message, optional, tag = "2")]
    pub leader: ::std::option::Option<Peer>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CommandPri {
    Normal = 0,
    Low = 1,
    High = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IsolationLevel {
    Si = 0,
    Rc = 1,
}
const METHOD_TIKV_RAW_GET: ::grpcio::Method<RawGetRequest, RawGetResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/get.Tikv/RawGet",
    req_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
    resp_mar: ::grpcio::Marshaller {
        ser: ::grpcio::pr_ser,
        de: ::grpcio::pr_de,
    },
};
const METHOD_TIKV_BATCH_COMMANDS: ::grpcio::Method<BatchCommandsRequest, BatchCommandsResponse> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Duplex,
        name: "/get.Tikv/BatchCommands",
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
pub struct TikvClient {
    client: ::grpcio::Client,
}
impl TikvClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        TikvClient {
            client: ::grpcio::Client::new(channel),
        }
    }
    pub fn raw_get_opt(
        &self,
        req: &RawGetRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<RawGetResponse> {
        self.client.unary_call(&METHOD_TIKV_RAW_GET, req, opt)
    }
    pub fn raw_get(&self, req: &RawGetRequest) -> ::grpcio::Result<RawGetResponse> {
        self.raw_get_opt(req, ::grpcio::CallOption::default())
    }
    pub fn raw_get_async_opt(
        &self,
        req: &RawGetRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<RawGetResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_RAW_GET, req, opt)
    }
    pub fn raw_get_async(
        &self,
        req: &RawGetRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<RawGetResponse>> {
        self.raw_get_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn batch_commands_opt(
        &self,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientDuplexSender<BatchCommandsRequest>,
        ::grpcio::ClientDuplexReceiver<BatchCommandsResponse>,
    )> {
        self.client
            .duplex_streaming(&METHOD_TIKV_BATCH_COMMANDS, opt)
    }
    pub fn batch_commands(
        &self,
    ) -> ::grpcio::Result<(
        ::grpcio::ClientDuplexSender<BatchCommandsRequest>,
        ::grpcio::ClientDuplexReceiver<BatchCommandsResponse>,
    )> {
        self.batch_commands_opt(::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F)
    where
        F: ::futures::Future<Item = (), Error = ()> + Send + 'static,
    {
        self.client.spawn(f)
    }
}
pub trait Tikv {
    fn raw_get(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: RawGetRequest,
        sink: ::grpcio::UnarySink<RawGetResponse>,
    );
    fn batch_commands(
        &mut self,
        ctx: ::grpcio::RpcContext,
        stream: ::grpcio::RequestStream<BatchCommandsRequest>,
        sink: ::grpcio::DuplexSink<BatchCommandsResponse>,
    );
}
pub fn create_tikv<S: Tikv + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_RAW_GET, move |ctx, req, resp| {
        instance.raw_get(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder
        .add_duplex_streaming_handler(&METHOD_TIKV_BATCH_COMMANDS, move |ctx, req, resp| {
            instance.batch_commands(ctx, req, resp)
        });
    builder.build()
}
