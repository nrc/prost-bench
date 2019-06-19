// Generated file, please don't edit manually.

impl BatchCommandsRequest {
    pub fn new_() -> BatchCommandsRequest {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn clear_requests(&mut self) {
        self.requests.clear();
    }
    #[inline]
    pub fn set_requests(&mut self, v: ::std::vec::Vec<batch_commands_request::Request>) {
        self.requests = v;
    }
    #[inline]
    pub fn get_requests(&self) -> &::std::vec::Vec<batch_commands_request::Request> {
        &self.requests
    }
    #[inline]
    pub fn mut_requests(&mut self) -> &mut ::std::vec::Vec<batch_commands_request::Request> {
        &mut self.requests
    }
    #[inline]
    pub fn take_requests(&mut self) -> ::std::vec::Vec<batch_commands_request::Request> {
        ::std::mem::replace(&mut self.requests, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_request_ids(&mut self) {
        self.request_ids.clear();
    }
    #[inline]
    pub fn set_request_ids(&mut self, v: ::std::vec::Vec<u64>) {
        self.request_ids = v;
    }
    #[inline]
    pub fn get_request_ids(&self) -> &::std::vec::Vec<u64> {
        &self.request_ids
    }
    #[inline]
    pub fn mut_request_ids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.request_ids
    }
    #[inline]
    pub fn take_request_ids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.request_ids, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for BatchCommandsRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for BatchCommandsRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn new() -> Self {
        Self::new_()
    }
    fn default_instance() -> &'static BatchCommandsRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: BatchCommandsRequest = BatchCommandsRequest::new_();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if let Err(_) = ::prost::Message::encode(self, &mut buf) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if let Err(_) = ::prost::Message::merge(self, bytes) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl batch_commands_request::Request {
    pub fn new_() -> batch_commands_request::Request {
        ::std::default::Default::default()
    }
}
impl ::protobuf::Clear for batch_commands_request::Request {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for batch_commands_request::Request {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn new() -> Self {
        Self::new_()
    }
    fn default_instance() -> &'static batch_commands_request::Request {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: batch_commands_request::Request = batch_commands_request::Request::new_();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if let Err(_) = ::prost::Message::encode(self, &mut buf) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if let Err(_) = ::prost::Message::merge(self, bytes) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl BatchCommandsResponse {
    pub fn new_() -> BatchCommandsResponse {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn clear_responses(&mut self) {
        self.responses.clear();
    }
    #[inline]
    pub fn set_responses(&mut self, v: ::std::vec::Vec<batch_commands_response::Response>) {
        self.responses = v;
    }
    #[inline]
    pub fn get_responses(&self) -> &::std::vec::Vec<batch_commands_response::Response> {
        &self.responses
    }
    #[inline]
    pub fn mut_responses(&mut self) -> &mut ::std::vec::Vec<batch_commands_response::Response> {
        &mut self.responses
    }
    #[inline]
    pub fn take_responses(&mut self) -> ::std::vec::Vec<batch_commands_response::Response> {
        ::std::mem::replace(&mut self.responses, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_request_ids(&mut self) {
        self.request_ids.clear();
    }
    #[inline]
    pub fn set_request_ids(&mut self, v: ::std::vec::Vec<u64>) {
        self.request_ids = v;
    }
    #[inline]
    pub fn get_request_ids(&self) -> &::std::vec::Vec<u64> {
        &self.request_ids
    }
    #[inline]
    pub fn mut_request_ids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.request_ids
    }
    #[inline]
    pub fn take_request_ids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.request_ids, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for BatchCommandsResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for BatchCommandsResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn new() -> Self {
        Self::new_()
    }
    fn default_instance() -> &'static BatchCommandsResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: BatchCommandsResponse = BatchCommandsResponse::new_();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if let Err(_) = ::prost::Message::encode(self, &mut buf) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if let Err(_) = ::prost::Message::merge(self, bytes) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl batch_commands_response::Response {
    pub fn new_() -> batch_commands_response::Response {
        ::std::default::Default::default()
    }
}
impl ::protobuf::Clear for batch_commands_response::Response {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for batch_commands_response::Response {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn new() -> Self {
        Self::new_()
    }
    fn default_instance() -> &'static batch_commands_response::Response {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: batch_commands_response::Response = batch_commands_response::Response::new_();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if let Err(_) = ::prost::Message::encode(self, &mut buf) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if let Err(_) = ::prost::Message::merge(self, bytes) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl RawGetRequest {
    pub fn new_() -> RawGetRequest {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    #[inline]
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    #[inline]
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    #[inline]
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    #[inline]
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    #[inline]
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    #[inline]
    pub fn set_key(&mut self, v: std::vec::Vec<u8>) {
        self.key = v;
    }
    #[inline]
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    #[inline]
    pub fn mut_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.key
    }
    #[inline]
    pub fn take_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }
    #[inline]
    pub fn set_cf(&mut self, v: std::string::String) {
        self.cf = v;
    }
    #[inline]
    pub fn get_cf(&self) -> &str {
        &self.cf
    }
    #[inline]
    pub fn mut_cf(&mut self) -> &mut std::string::String {
        &mut self.cf
    }
    #[inline]
    pub fn take_cf(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.cf, ::std::string::String::new())
    }
}
impl ::protobuf::Clear for RawGetRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RawGetRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn new() -> Self {
        Self::new_()
    }
    fn default_instance() -> &'static RawGetRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawGetRequest = RawGetRequest::new_();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if let Err(_) = ::prost::Message::encode(self, &mut buf) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if let Err(_) = ::prost::Message::merge(self, bytes) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl RawGetResponse {
    pub fn new_() -> RawGetResponse {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    #[inline]
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_error(&mut self, v: Error) {
        self.region_error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_error(&self) -> &Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <Error as ::protobuf::Message>::default_instance(),
        }
    }
    #[inline]
    pub fn mut_region_error(&mut self) -> &mut Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_error(&mut self) -> Error {
        self.region_error.take().unwrap_or_else(Error::default)
    }
    #[inline]
    pub fn clear_error(&mut self) {
        self.error.clear();
    }
    #[inline]
    pub fn set_error(&mut self, v: std::string::String) {
        self.error = v;
    }
    #[inline]
    pub fn get_error(&self) -> &str {
        &self.error
    }
    #[inline]
    pub fn mut_error(&mut self) -> &mut std::string::String {
        &mut self.error
    }
    #[inline]
    pub fn take_error(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }
    #[inline]
    pub fn clear_value(&mut self) {
        self.value.clear();
    }
    #[inline]
    pub fn set_value(&mut self, v: std::vec::Vec<u8>) {
        self.value = v;
    }
    #[inline]
    pub fn get_value(&self) -> &[u8] {
        &self.value
    }
    #[inline]
    pub fn mut_value(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.value
    }
    #[inline]
    pub fn take_value(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for RawGetResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RawGetResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn new() -> Self {
        Self::new_()
    }
    fn default_instance() -> &'static RawGetResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawGetResponse = RawGetResponse::new_();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if let Err(_) = ::prost::Message::encode(self, &mut buf) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if let Err(_) = ::prost::Message::merge(self, bytes) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl Context {
    pub fn new_() -> Context {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn clear_region_id(&mut self) {
        self.region_id = 0
    }
    #[inline]
    pub fn set_region_id(&mut self, v: u64) {
        self.region_id = v;
    }
    #[inline]
    pub fn get_region_id(&self) -> u64 {
        self.region_id
    }
    #[inline]
    pub fn has_region_epoch(&self) -> bool {
        self.region_epoch.is_some()
    }
    #[inline]
    pub fn clear_region_epoch(&mut self) {
        self.region_epoch = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_epoch(&mut self, v: RegionEpoch) {
        self.region_epoch = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_epoch(&self) -> &RegionEpoch {
        match self.region_epoch.as_ref() {
            Some(v) => v,
            None => <RegionEpoch as ::protobuf::Message>::default_instance(),
        }
    }
    #[inline]
    pub fn mut_region_epoch(&mut self) -> &mut RegionEpoch {
        if self.region_epoch.is_none() {
            self.region_epoch = ::std::option::Option::Some(RegionEpoch::default());
        }
        self.region_epoch.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_epoch(&mut self) -> RegionEpoch {
        self.region_epoch
            .take()
            .unwrap_or_else(RegionEpoch::default)
    }
    #[inline]
    pub fn has_peer(&self) -> bool {
        self.peer.is_some()
    }
    #[inline]
    pub fn clear_peer(&mut self) {
        self.peer = ::std::option::Option::None
    }
    #[inline]
    pub fn set_peer(&mut self, v: Peer) {
        self.peer = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_peer(&self) -> &Peer {
        match self.peer.as_ref() {
            Some(v) => v,
            None => <Peer as ::protobuf::Message>::default_instance(),
        }
    }
    #[inline]
    pub fn mut_peer(&mut self) -> &mut Peer {
        if self.peer.is_none() {
            self.peer = ::std::option::Option::Some(Peer::default());
        }
        self.peer.as_mut().unwrap()
    }
    #[inline]
    pub fn take_peer(&mut self) -> Peer {
        self.peer.take().unwrap_or_else(Peer::default)
    }
    #[inline]
    pub fn clear_term(&mut self) {
        self.term = 0
    }
    #[inline]
    pub fn set_term(&mut self, v: u64) {
        self.term = v;
    }
    #[inline]
    pub fn get_term(&self) -> u64 {
        self.term
    }
    #[inline]
    pub fn clear_priority(&mut self) {
        self.priority = 0
    }
    #[inline]
    pub fn set_priority_(&mut self, v: CommandPri) {
        self.priority = unsafe { ::std::mem::transmute::<CommandPri, i32>(v) };
    }
    #[inline]
    pub fn get_priority(&self) -> CommandPri {
        unsafe { ::std::mem::transmute::<i32, CommandPri>(self.priority) }
    }
    #[inline]
    pub fn clear_isolation_level(&mut self) {
        self.isolation_level = 0
    }
    #[inline]
    pub fn set_isolation_level_(&mut self, v: IsolationLevel) {
        self.isolation_level = unsafe { ::std::mem::transmute::<IsolationLevel, i32>(v) };
    }
    #[inline]
    pub fn get_isolation_level(&self) -> IsolationLevel {
        unsafe { ::std::mem::transmute::<i32, IsolationLevel>(self.isolation_level) }
    }
    #[inline]
    pub fn clear_not_fill_cache(&mut self) {
        self.not_fill_cache = false
    }
    #[inline]
    pub fn set_not_fill_cache(&mut self, v: bool) {
        self.not_fill_cache = v;
    }
    #[inline]
    pub fn get_not_fill_cache(&self) -> bool {
        self.not_fill_cache
    }
    #[inline]
    pub fn clear_sync_log(&mut self) {
        self.sync_log = false
    }
    #[inline]
    pub fn set_sync_log(&mut self, v: bool) {
        self.sync_log = v;
    }
    #[inline]
    pub fn get_sync_log(&self) -> bool {
        self.sync_log
    }
    #[inline]
    pub fn clear_handle_time(&mut self) {
        self.handle_time = false
    }
    #[inline]
    pub fn set_handle_time(&mut self, v: bool) {
        self.handle_time = v;
    }
    #[inline]
    pub fn get_handle_time(&self) -> bool {
        self.handle_time
    }
    #[inline]
    pub fn clear_scan_detail(&mut self) {
        self.scan_detail = false
    }
    #[inline]
    pub fn set_scan_detail(&mut self, v: bool) {
        self.scan_detail = v;
    }
    #[inline]
    pub fn get_scan_detail(&self) -> bool {
        self.scan_detail
    }
}
impl ::protobuf::Clear for Context {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Context {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn new() -> Self {
        Self::new_()
    }
    fn default_instance() -> &'static Context {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Context = Context::new_();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if let Err(_) = ::prost::Message::encode(self, &mut buf) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if let Err(_) = ::prost::Message::merge(self, bytes) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl RegionEpoch {
    pub fn new_() -> RegionEpoch {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn clear_conf_ver(&mut self) {
        self.conf_ver = 0
    }
    #[inline]
    pub fn set_conf_ver(&mut self, v: u64) {
        self.conf_ver = v;
    }
    #[inline]
    pub fn get_conf_ver(&self) -> u64 {
        self.conf_ver
    }
    #[inline]
    pub fn clear_version(&mut self) {
        self.version = 0
    }
    #[inline]
    pub fn set_version(&mut self, v: u64) {
        self.version = v;
    }
    #[inline]
    pub fn get_version(&self) -> u64 {
        self.version
    }
}
impl ::protobuf::Clear for RegionEpoch {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RegionEpoch {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn new() -> Self {
        Self::new_()
    }
    fn default_instance() -> &'static RegionEpoch {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RegionEpoch = RegionEpoch::new_();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if let Err(_) = ::prost::Message::encode(self, &mut buf) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if let Err(_) = ::prost::Message::merge(self, bytes) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl Peer {
    pub fn new_() -> Peer {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn clear_id(&mut self) {
        self.id = 0
    }
    #[inline]
    pub fn set_id(&mut self, v: u64) {
        self.id = v;
    }
    #[inline]
    pub fn get_id(&self) -> u64 {
        self.id
    }
    #[inline]
    pub fn clear_store_id(&mut self) {
        self.store_id = 0
    }
    #[inline]
    pub fn set_store_id(&mut self, v: u64) {
        self.store_id = v;
    }
    #[inline]
    pub fn get_store_id(&self) -> u64 {
        self.store_id
    }
    #[inline]
    pub fn clear_is_learner(&mut self) {
        self.is_learner = false
    }
    #[inline]
    pub fn set_is_learner(&mut self, v: bool) {
        self.is_learner = v;
    }
    #[inline]
    pub fn get_is_learner(&self) -> bool {
        self.is_learner
    }
}
impl ::protobuf::Clear for Peer {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Peer {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn new() -> Self {
        Self::new_()
    }
    fn default_instance() -> &'static Peer {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Peer = Peer::new_();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if let Err(_) = ::prost::Message::encode(self, &mut buf) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if let Err(_) = ::prost::Message::merge(self, bytes) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl Error {
    pub fn new_() -> Error {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn clear_message(&mut self) {
        self.message.clear();
    }
    #[inline]
    pub fn set_message(&mut self, v: std::string::String) {
        self.message = v;
    }
    #[inline]
    pub fn get_message(&self) -> &str {
        &self.message
    }
    #[inline]
    pub fn mut_message(&mut self) -> &mut std::string::String {
        &mut self.message
    }
    #[inline]
    pub fn take_message(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.message, ::std::string::String::new())
    }
    #[inline]
    pub fn has_not_leader(&self) -> bool {
        self.not_leader.is_some()
    }
    #[inline]
    pub fn clear_not_leader(&mut self) {
        self.not_leader = ::std::option::Option::None
    }
    #[inline]
    pub fn set_not_leader(&mut self, v: NotLeader) {
        self.not_leader = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_not_leader(&self) -> &NotLeader {
        match self.not_leader.as_ref() {
            Some(v) => v,
            None => <NotLeader as ::protobuf::Message>::default_instance(),
        }
    }
    #[inline]
    pub fn mut_not_leader(&mut self) -> &mut NotLeader {
        if self.not_leader.is_none() {
            self.not_leader = ::std::option::Option::Some(NotLeader::default());
        }
        self.not_leader.as_mut().unwrap()
    }
    #[inline]
    pub fn take_not_leader(&mut self) -> NotLeader {
        self.not_leader.take().unwrap_or_else(NotLeader::default)
    }
}
impl ::protobuf::Clear for Error {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Error {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn new() -> Self {
        Self::new_()
    }
    fn default_instance() -> &'static Error {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Error = Error::new_();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if let Err(_) = ::prost::Message::encode(self, &mut buf) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if let Err(_) = ::prost::Message::merge(self, bytes) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl NotLeader {
    pub fn new_() -> NotLeader {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn clear_region_id(&mut self) {
        self.region_id = 0
    }
    #[inline]
    pub fn set_region_id(&mut self, v: u64) {
        self.region_id = v;
    }
    #[inline]
    pub fn get_region_id(&self) -> u64 {
        self.region_id
    }
    #[inline]
    pub fn has_leader(&self) -> bool {
        self.leader.is_some()
    }
    #[inline]
    pub fn clear_leader(&mut self) {
        self.leader = ::std::option::Option::None
    }
    #[inline]
    pub fn set_leader(&mut self, v: Peer) {
        self.leader = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_leader(&self) -> &Peer {
        match self.leader.as_ref() {
            Some(v) => v,
            None => <Peer as ::protobuf::Message>::default_instance(),
        }
    }
    #[inline]
    pub fn mut_leader(&mut self) -> &mut Peer {
        if self.leader.is_none() {
            self.leader = ::std::option::Option::Some(Peer::default());
        }
        self.leader.as_mut().unwrap()
    }
    #[inline]
    pub fn take_leader(&mut self) -> Peer {
        self.leader.take().unwrap_or_else(Peer::default)
    }
}
impl ::protobuf::Clear for NotLeader {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for NotLeader {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn new() -> Self {
        Self::new_()
    }
    fn default_instance() -> &'static NotLeader {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: NotLeader = NotLeader::new_();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if let Err(_) = ::prost::Message::encode(self, &mut buf) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if let Err(_) = ::prost::Message::merge(self, bytes) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl CommandPri {
    pub fn values() -> &'static [Self] {
        static VALUES: &'static [CommandPri] =
            &[CommandPri::Normal, CommandPri::Low, CommandPri::High];
        VALUES
    }
}
impl IsolationLevel {
    pub fn values() -> &'static [Self] {
        static VALUES: &'static [IsolationLevel] = &[IsolationLevel::Si, IsolationLevel::Rc];
        VALUES
    }
}
