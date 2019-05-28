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
        #[prost(oneof = "request::Cmd", tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12")]
        pub cmd: ::std::option::Option<request::Cmd>,
    }
    pub mod request {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Cmd {
            #[prost(message, tag = "1")]
            Get(super::super::GetRequest),
            #[prost(message, tag = "2")]
            Scan(super::super::ScanRequest),
            #[prost(message, tag = "3")]
            Prewrite(super::super::PrewriteRequest),
            #[prost(message, tag = "4")]
            Commit(super::super::CommitRequest),
            #[prost(message, tag = "5")]
            Import(super::super::ImportRequest),
            #[prost(message, tag = "6")]
            Cleanup(super::super::CleanupRequest),
            #[prost(message, tag = "7")]
            BatchGet(super::super::BatchGetRequest),
            #[prost(message, tag = "8")]
            BatchRollback(super::super::BatchRollbackRequest),
            #[prost(message, tag = "9")]
            ScanLock(super::super::ScanLockRequest),
            #[prost(message, tag = "10")]
            ResolveLock(super::super::ResolveLockRequest),
            #[prost(message, tag = "11")]
            Gc(super::super::GcRequest),
            #[prost(message, tag = "12")]
            DeleteRange(super::super::DeleteRangeRequest),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCommandsResponse {
    #[prost(message, repeated, tag = "1")]
    pub responses: ::std::vec::Vec<batch_commands_response::Response>,
    #[prost(uint64, repeated, tag = "2")]
    pub request_ids: ::std::vec::Vec<u64>,
    #[prost(uint64, tag = "3")]
    pub transport_layer_load: u64,
}
pub mod batch_commands_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        #[prost(
            oneof = "response::Cmd",
            tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12"
        )]
        pub cmd: ::std::option::Option<response::Cmd>,
    }
    pub mod response {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Cmd {
            #[prost(message, tag = "1")]
            Get(super::super::GetResponse),
            #[prost(message, tag = "2")]
            Scan(super::super::ScanResponse),
            #[prost(message, tag = "3")]
            Prewrite(super::super::PrewriteResponse),
            #[prost(message, tag = "4")]
            Commit(super::super::CommitResponse),
            #[prost(message, tag = "5")]
            Import(super::super::ImportResponse),
            #[prost(message, tag = "6")]
            Cleanup(super::super::CleanupResponse),
            #[prost(message, tag = "7")]
            BatchGet(super::super::BatchGetResponse),
            #[prost(message, tag = "8")]
            BatchRollback(super::super::BatchRollbackResponse),
            #[prost(message, tag = "9")]
            ScanLock(super::super::ScanLockResponse),
            #[prost(message, tag = "10")]
            ResolveLock(super::super::ResolveLockResponse),
            #[prost(message, tag = "11")]
            Gc(super::super::GcResponse),
            #[prost(message, tag = "12")]
            DeleteRange(super::super::DeleteRangeResponse),
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::std::option::Option<Context>,
    #[prost(bytes, tag = "2")]
    pub key: ::bytes::Bytes,
    #[prost(uint64, tag = "3")]
    pub version: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::std::option::Option<Error>,
    #[prost(message, optional, tag = "2")]
    pub error: ::std::option::Option<KeyError>,
    #[prost(bytes, tag = "3")]
    pub value: ::bytes::Bytes,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScanRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::std::option::Option<Context>,
    #[prost(bytes, tag = "2")]
    pub start_key: ::bytes::Bytes,
    #[prost(uint32, tag = "3")]
    pub limit: u32,
    #[prost(uint64, tag = "4")]
    pub version: u64,
    #[prost(bool, tag = "5")]
    pub key_only: bool,
    #[prost(bool, tag = "6")]
    pub reverse: bool,
    /// For compatibility, when scanning forward, the range to scan is [start_key, end_key), where start_key < end_key;
    /// and when scanning backward, it scans [end_key, start_key) in descending order, where end_key < start_key.
    #[prost(bytes, tag = "7")]
    pub end_key: ::bytes::Bytes,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KvPair {
    #[prost(message, optional, tag = "1")]
    pub error: ::std::option::Option<KeyError>,
    #[prost(bytes, tag = "2")]
    pub key: ::bytes::Bytes,
    #[prost(bytes, tag = "3")]
    pub value: ::bytes::Bytes,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScanResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::std::option::Option<Error>,
    #[prost(message, repeated, tag = "2")]
    pub pairs: ::std::vec::Vec<KvPair>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mutation {
    #[prost(enumeration = "Op", tag = "1")]
    pub op: i32,
    #[prost(bytes, tag = "2")]
    pub key: ::bytes::Bytes,
    #[prost(bytes, tag = "3")]
    pub value: ::bytes::Bytes,
    #[prost(enumeration = "Assertion", tag = "4")]
    pub assertion: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrewriteRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::std::option::Option<Context>,
    #[prost(message, repeated, tag = "2")]
    pub mutations: ::std::vec::Vec<Mutation>,
    /// primary_lock_key
    #[prost(bytes, tag = "3")]
    pub primary_lock: ::bytes::Bytes,
    #[prost(uint64, tag = "4")]
    pub start_version: u64,
    #[prost(uint64, tag = "5")]
    pub lock_ttl: u64,
    #[prost(bool, tag = "6")]
    pub skip_constraint_check: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrewriteResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::std::option::Option<Error>,
    #[prost(message, repeated, tag = "2")]
    pub errors: ::std::vec::Vec<KeyError>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::std::option::Option<Context>,
    #[prost(uint64, tag = "2")]
    pub start_version: u64,
    #[prost(bytes, repeated, tag = "3")]
    pub keys: ::std::vec::Vec<::bytes::Bytes>,
    #[prost(uint64, tag = "4")]
    pub commit_version: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::std::option::Option<Error>,
    #[prost(message, optional, tag = "2")]
    pub error: ::std::option::Option<KeyError>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportRequest {
    #[prost(message, repeated, tag = "1")]
    pub mutations: ::std::vec::Vec<Mutation>,
    #[prost(uint64, tag = "2")]
    pub commit_version: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::std::option::Option<Error>,
    #[prost(string, tag = "2")]
    pub error: ::prost::BytesString,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchRollbackRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::std::option::Option<Context>,
    #[prost(uint64, tag = "2")]
    pub start_version: u64,
    #[prost(bytes, repeated, tag = "3")]
    pub keys: ::std::vec::Vec<::bytes::Bytes>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchRollbackResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::std::option::Option<Error>,
    #[prost(message, optional, tag = "2")]
    pub error: ::std::option::Option<KeyError>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CleanupRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::std::option::Option<Context>,
    #[prost(bytes, tag = "2")]
    pub key: ::bytes::Bytes,
    #[prost(uint64, tag = "3")]
    pub start_version: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CleanupResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::std::option::Option<Error>,
    #[prost(message, optional, tag = "2")]
    pub error: ::std::option::Option<KeyError>,
    /// set this if the key is already committed
    #[prost(uint64, tag = "3")]
    pub commit_version: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::std::option::Option<Context>,
    #[prost(bytes, repeated, tag = "2")]
    pub keys: ::std::vec::Vec<::bytes::Bytes>,
    #[prost(uint64, tag = "3")]
    pub version: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::std::option::Option<Error>,
    #[prost(message, repeated, tag = "2")]
    pub pairs: ::std::vec::Vec<KvPair>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScanLockRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::std::option::Option<Context>,
    #[prost(uint64, tag = "2")]
    pub max_version: u64,
    #[prost(bytes, tag = "3")]
    pub start_key: ::bytes::Bytes,
    #[prost(uint32, tag = "4")]
    pub limit: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScanLockResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::std::option::Option<Error>,
    #[prost(message, optional, tag = "2")]
    pub error: ::std::option::Option<KeyError>,
    #[prost(message, repeated, tag = "3")]
    pub locks: ::std::vec::Vec<LockInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxnInfo {
    #[prost(uint64, tag = "1")]
    pub txn: u64,
    #[prost(uint64, tag = "2")]
    pub status: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveLockRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::std::option::Option<Context>,
    #[prost(uint64, tag = "2")]
    pub start_version: u64,
    /// If the txn is rolled back, do not set it.
    #[prost(uint64, tag = "3")]
    pub commit_version: u64,
    #[prost(message, repeated, tag = "4")]
    pub txn_infos: ::std::vec::Vec<TxnInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveLockResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::std::option::Option<Error>,
    #[prost(message, optional, tag = "2")]
    pub error: ::std::option::Option<KeyError>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::std::option::Option<Context>,
    #[prost(uint64, tag = "2")]
    pub safe_point: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::std::option::Option<Error>,
    #[prost(message, optional, tag = "2")]
    pub error: ::std::option::Option<KeyError>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawGetRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::std::option::Option<Context>,
    #[prost(bytes, tag = "2")]
    pub key: ::bytes::Bytes,
    #[prost(string, tag = "3")]
    pub cf: ::prost::BytesString,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawGetResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::std::option::Option<Error>,
    #[prost(string, tag = "2")]
    pub error: ::prost::BytesString,
    #[prost(bytes, tag = "3")]
    pub value: ::bytes::Bytes,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawPutRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::std::option::Option<Context>,
    #[prost(bytes, tag = "2")]
    pub key: ::bytes::Bytes,
    #[prost(bytes, tag = "3")]
    pub value: ::bytes::Bytes,
    #[prost(string, tag = "4")]
    pub cf: ::prost::BytesString,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawPutResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::std::option::Option<Error>,
    #[prost(string, tag = "2")]
    pub error: ::prost::BytesString,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawBatchPutRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::std::option::Option<Context>,
    #[prost(message, repeated, tag = "2")]
    pub pairs: ::std::vec::Vec<KvPair>,
    #[prost(string, tag = "3")]
    pub cf: ::prost::BytesString,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawBatchPutResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::std::option::Option<Error>,
    #[prost(string, tag = "2")]
    pub error: ::prost::BytesString,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawBatchGetRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::std::option::Option<Context>,
    #[prost(bytes, repeated, tag = "2")]
    pub keys: ::std::vec::Vec<::bytes::Bytes>,
    #[prost(string, tag = "3")]
    pub cf: ::prost::BytesString,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawBatchGetResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::std::option::Option<Error>,
    #[prost(message, repeated, tag = "2")]
    pub pairs: ::std::vec::Vec<KvPair>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawDeleteRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::std::option::Option<Context>,
    #[prost(bytes, tag = "2")]
    pub key: ::bytes::Bytes,
    #[prost(string, tag = "3")]
    pub cf: ::prost::BytesString,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawDeleteResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::std::option::Option<Error>,
    #[prost(string, tag = "2")]
    pub error: ::prost::BytesString,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawBatchDeleteRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::std::option::Option<Context>,
    #[prost(bytes, repeated, tag = "2")]
    pub keys: ::std::vec::Vec<::bytes::Bytes>,
    #[prost(string, tag = "3")]
    pub cf: ::prost::BytesString,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawBatchDeleteResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::std::option::Option<Error>,
    #[prost(string, tag = "2")]
    pub error: ::prost::BytesString,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRangeRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::std::option::Option<Context>,
    #[prost(bytes, tag = "2")]
    pub start_key: ::bytes::Bytes,
    #[prost(bytes, tag = "3")]
    pub end_key: ::bytes::Bytes,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRangeResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::std::option::Option<Error>,
    #[prost(string, tag = "2")]
    pub error: ::prost::BytesString,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotLeader {
    #[prost(uint64, tag = "1")]
    pub region_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreNotMatch {
    #[prost(uint64, tag = "1")]
    pub request_store_id: u64,
    #[prost(uint64, tag = "2")]
    pub actual_store_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionNotFound {
    #[prost(uint64, tag = "1")]
    pub region_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyNotInRegion {
    #[prost(bytes, tag = "1")]
    pub key: ::bytes::Bytes,
    #[prost(uint64, tag = "2")]
    pub region_id: u64,
    #[prost(bytes, tag = "3")]
    pub start_key: ::bytes::Bytes,
    #[prost(bytes, tag = "4")]
    pub end_key: ::bytes::Bytes,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerIsBusy {
    #[prost(string, tag = "1")]
    pub reason: ::prost::BytesString,
    #[prost(uint64, tag = "2")]
    pub backoff_ms: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaleCommand {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaftEntryTooLarge {
    #[prost(uint64, tag = "1")]
    pub region_id: u64,
    #[prost(uint64, tag = "2")]
    pub entry_size: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    #[prost(string, tag = "1")]
    pub message: ::prost::BytesString,
    #[prost(message, optional, tag = "2")]
    pub not_leader: ::std::option::Option<NotLeader>,
    #[prost(message, optional, tag = "3")]
    pub region_not_found: ::std::option::Option<RegionNotFound>,
    #[prost(message, optional, tag = "4")]
    pub key_not_in_region: ::std::option::Option<KeyNotInRegion>,
    #[prost(message, optional, tag = "5")]
    pub server_is_busy: ::std::option::Option<ServerIsBusy>,
    #[prost(message, optional, tag = "6")]
    pub stale_command: ::std::option::Option<StaleCommand>,
    #[prost(message, optional, tag = "7")]
    pub store_not_match: ::std::option::Option<StoreNotMatch>,
    #[prost(message, optional, tag = "8")]
    pub raft_entry_too_large: ::std::option::Option<RaftEntryTooLarge>,
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
    /// true means return handle time detail
    #[prost(bool, tag = "10")]
    pub handle_time: bool,
    /// true means return scan cf's detail
    #[prost(bool, tag = "11")]
    pub scan_detail: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionEpoch {
    /// Conf change version, auto increment when add or remove peer
    #[prost(uint64, tag = "1")]
    pub conf_ver: u64,
    /// Region version, auto increment when split or merge
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
pub struct KeyError {
    /// Client should backoff or cleanup the lock then retry.
    #[prost(message, optional, tag = "1")]
    pub locked: ::std::option::Option<LockInfo>,
    /// Client may restart the txn. e.g write conflict.
    #[prost(string, tag = "2")]
    pub retryable: ::prost::BytesString,
    /// Client should abort the txn.
    #[prost(string, tag = "3")]
    pub abort: ::prost::BytesString,
    /// Write conflict is moved from retryable to here.
    #[prost(message, optional, tag = "4")]
    pub conflict: ::std::option::Option<WriteConflict>,
    /// Key already exists
    #[prost(message, optional, tag = "5")]
    pub already_exist: ::std::option::Option<AlreadyExist>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockInfo {
    #[prost(bytes, tag = "1")]
    pub primary_lock: ::bytes::Bytes,
    #[prost(uint64, tag = "2")]
    pub lock_version: u64,
    #[prost(bytes, tag = "3")]
    pub key: ::bytes::Bytes,
    #[prost(uint64, tag = "4")]
    pub lock_ttl: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlreadyExist {
    #[prost(bytes, tag = "1")]
    pub key: ::bytes::Bytes,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteConflict {
    #[prost(uint64, tag = "1")]
    pub start_ts: u64,
    #[prost(uint64, tag = "2")]
    pub conflict_ts: u64,
    #[prost(bytes, tag = "3")]
    pub key: ::bytes::Bytes,
    #[prost(bytes, tag = "4")]
    pub primary: ::bytes::Bytes,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Op {
    Put = 0,
    Del = 1,
    Lock = 2,
    Rollback = 3,
    /// insert operation has a constraint that key should not exist before.
    Insert = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Assertion {
    None = 0,
    Exist = 1,
    NotExist = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CommandPri {
    /// Normal must the default value
    Normal = 0,
    Low = 1,
    High = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IsolationLevel {
    /// SI = snapshot isolation
    Si = 0,
    /// RC = read committed
    Rc = 1,
}
const METHOD_COMPLEX_UN: ::grpcio::Method<BatchCommandsRequest, BatchCommandsResponse> =
    ::grpcio::Method {
        ty: ::grpcio::MethodType::Unary,
        name: "/complex.Complex/un",
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
pub struct ComplexClient {
    client: ::grpcio::Client,
}
impl ComplexClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ComplexClient {
            client: ::grpcio::Client::new(channel),
        }
    }
    pub fn un_opt(
        &self,
        req: &BatchCommandsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<BatchCommandsResponse> {
        self.client.unary_call(&METHOD_COMPLEX_UN, req, opt)
    }
    pub fn un(&self, req: &BatchCommandsRequest) -> ::grpcio::Result<BatchCommandsResponse> {
        self.un_opt(req, ::grpcio::CallOption::default())
    }
    pub fn un_async_opt(
        &self,
        req: &BatchCommandsRequest,
        opt: ::grpcio::CallOption,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<BatchCommandsResponse>> {
        self.client.unary_call_async(&METHOD_COMPLEX_UN, req, opt)
    }
    pub fn un_async(
        &self,
        req: &BatchCommandsRequest,
    ) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<BatchCommandsResponse>> {
        self.un_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F)
    where
        F: ::futures::Future<Item = (), Error = ()> + Send + 'static,
    {
        self.client.spawn(f)
    }
}
pub trait Complex {
    fn un(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: BatchCommandsRequest,
        sink: ::grpcio::UnarySink<BatchCommandsResponse>,
    );
}
pub fn create_complex<S: Complex + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_COMPLEX_UN, move |ctx, req, resp| {
        instance.un(ctx, req, resp)
    });
    builder.build()
}
