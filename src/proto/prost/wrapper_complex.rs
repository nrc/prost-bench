impl BatchCommandsRequest {
    pub fn new_() -> BatchCommandsRequest {
        ::std::default::Default::default()
    }
    pub fn clear_requests(&mut self) {
        self.requests.clear();
    }
    pub fn set_requests(&mut self, v: ::std::vec::Vec<batch_commands_request::Request>) {
        self.requests = v;
    }
    pub fn get_requests(&self) -> &::std::vec::Vec<batch_commands_request::Request> {
        &self.requests
    }
    pub fn mut_requests(&mut self) -> &mut ::std::vec::Vec<batch_commands_request::Request> {
        &mut self.requests
    }
    pub fn take_requests(&mut self) -> ::std::vec::Vec<batch_commands_request::Request> {
        ::std::mem::replace(&mut self.requests, ::std::vec::Vec::new())
    }
    pub fn clear_request_ids(&mut self) {
        self.request_ids.clear();
    }
    pub fn set_request_ids(&mut self, v: ::std::vec::Vec<u64>) {
        self.request_ids = v;
    }
    pub fn get_request_ids(&self) -> &::std::vec::Vec<u64> {
        &self.request_ids
    }
    pub fn mut_request_ids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.request_ids
    }
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
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static BatchCommandsRequest {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
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
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static batch_commands_request::Request {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl BatchCommandsResponse {
    pub fn new_() -> BatchCommandsResponse {
        ::std::default::Default::default()
    }
    pub fn clear_responses(&mut self) {
        self.responses.clear();
    }
    pub fn set_responses(&mut self, v: ::std::vec::Vec<batch_commands_response::Response>) {
        self.responses = v;
    }
    pub fn get_responses(&self) -> &::std::vec::Vec<batch_commands_response::Response> {
        &self.responses
    }
    pub fn mut_responses(&mut self) -> &mut ::std::vec::Vec<batch_commands_response::Response> {
        &mut self.responses
    }
    pub fn take_responses(&mut self) -> ::std::vec::Vec<batch_commands_response::Response> {
        ::std::mem::replace(&mut self.responses, ::std::vec::Vec::new())
    }
    pub fn clear_request_ids(&mut self) {
        self.request_ids.clear();
    }
    pub fn set_request_ids(&mut self, v: ::std::vec::Vec<u64>) {
        self.request_ids = v;
    }
    pub fn get_request_ids(&self) -> &::std::vec::Vec<u64> {
        &self.request_ids
    }
    pub fn mut_request_ids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.request_ids
    }
    pub fn take_request_ids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.request_ids, ::std::vec::Vec::new())
    }
    pub fn clear_transport_layer_load(&mut self) {
        self.transport_layer_load = 0
    }
    pub fn set_transport_layer_load(&mut self, v: u64) {
        self.transport_layer_load = v;
    }
    pub fn get_transport_layer_load(&self) -> u64 {
        self.transport_layer_load
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
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static BatchCommandsResponse {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
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
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static batch_commands_response::Response {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl GetRequest {
    pub fn new_() -> GetRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    pub fn set_key(&mut self, v: std::vec::Vec<u8>) {
        self.key = v;
    }
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn mut_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.key
    }
    pub fn take_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    pub fn clear_version(&mut self) {
        self.version = 0
    }
    pub fn set_version(&mut self, v: u64) {
        self.version = v;
    }
    pub fn get_version(&self) -> u64 {
        self.version
    }
}
impl ::protobuf::Clear for GetRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for GetRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static GetRequest {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl GetResponse {
    pub fn new_() -> GetResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> Error {
        self.region_error.take().unwrap_or_else(Error::default)
    }
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None
    }
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::std::option::Option::Some(v);;    }
    pub fn get_error(&self) -> &KeyError {
        match self.error.as_ref() {
            Some(v) => v,
            None => <KeyError as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(KeyError::default());
        }
        self.error.as_mut().unwrap()
    }
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(KeyError::default)
    }
    pub fn clear_value(&mut self) {
        self.value.clear();
    }
    pub fn set_value(&mut self, v: std::vec::Vec<u8>) {
        self.value = v;
    }
    pub fn get_value(&self) -> &[u8] {
        &self.value
    }
    pub fn mut_value(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.value
    }
    pub fn take_value(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for GetResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for GetResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static GetResponse {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl ScanRequest {
    pub fn new_() -> ScanRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    pub fn clear_start_key(&mut self) {
        self.start_key.clear();
    }
    pub fn set_start_key(&mut self, v: std::vec::Vec<u8>) {
        self.start_key = v;
    }
    pub fn get_start_key(&self) -> &[u8] {
        &self.start_key
    }
    pub fn mut_start_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.start_key
    }
    pub fn take_start_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.start_key, ::std::vec::Vec::new())
    }
    pub fn clear_limit(&mut self) {
        self.limit = 0
    }
    pub fn set_limit(&mut self, v: u32) {
        self.limit = v;
    }
    pub fn get_limit(&self) -> u32 {
        self.limit
    }
    pub fn clear_version(&mut self) {
        self.version = 0
    }
    pub fn set_version(&mut self, v: u64) {
        self.version = v;
    }
    pub fn get_version(&self) -> u64 {
        self.version
    }
    pub fn clear_key_only(&mut self) {
        self.key_only = false
    }
    pub fn set_key_only(&mut self, v: bool) {
        self.key_only = v;
    }
    pub fn get_key_only(&self) -> bool {
        self.key_only
    }
    pub fn clear_reverse(&mut self) {
        self.reverse = false
    }
    pub fn set_reverse(&mut self, v: bool) {
        self.reverse = v;
    }
    pub fn get_reverse(&self) -> bool {
        self.reverse
    }
    pub fn clear_end_key(&mut self) {
        self.end_key.clear();
    }
    pub fn set_end_key(&mut self, v: std::vec::Vec<u8>) {
        self.end_key = v;
    }
    pub fn get_end_key(&self) -> &[u8] {
        &self.end_key
    }
    pub fn mut_end_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.end_key
    }
    pub fn take_end_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.end_key, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for ScanRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ScanRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static ScanRequest {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl KvPair {
    pub fn new_() -> KvPair {
        ::std::default::Default::default()
    }
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None
    }
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::std::option::Option::Some(v);;    }
    pub fn get_error(&self) -> &KeyError {
        match self.error.as_ref() {
            Some(v) => v,
            None => <KeyError as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(KeyError::default());
        }
        self.error.as_mut().unwrap()
    }
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(KeyError::default)
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    pub fn set_key(&mut self, v: std::vec::Vec<u8>) {
        self.key = v;
    }
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn mut_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.key
    }
    pub fn take_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    pub fn clear_value(&mut self) {
        self.value.clear();
    }
    pub fn set_value(&mut self, v: std::vec::Vec<u8>) {
        self.value = v;
    }
    pub fn get_value(&self) -> &[u8] {
        &self.value
    }
    pub fn mut_value(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.value
    }
    pub fn take_value(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for KvPair {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for KvPair {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static KvPair {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl ScanResponse {
    pub fn new_() -> ScanResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> Error {
        self.region_error.take().unwrap_or_else(Error::default)
    }
    pub fn clear_pairs(&mut self) {
        self.pairs.clear();
    }
    pub fn set_pairs(&mut self, v: ::std::vec::Vec<KvPair>) {
        self.pairs = v;
    }
    pub fn get_pairs(&self) -> &::std::vec::Vec<KvPair> {
        &self.pairs
    }
    pub fn mut_pairs(&mut self) -> &mut ::std::vec::Vec<KvPair> {
        &mut self.pairs
    }
    pub fn take_pairs(&mut self) -> ::std::vec::Vec<KvPair> {
        ::std::mem::replace(&mut self.pairs, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for ScanResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ScanResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static ScanResponse {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl Mutation {
    pub fn new_() -> Mutation {
        ::std::default::Default::default()
    }
    pub fn clear_op(&mut self) {
        self.op = 0
    }
    pub fn set_op_(&mut self, v: Op) {
        self.op = unsafe { ::std::mem::transmute::<Op, i32>(v) };
    }
    pub fn get_op(&self) -> Op {
        unsafe { ::std::mem::transmute::<i32, Op>(self.op) }
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    pub fn set_key(&mut self, v: std::vec::Vec<u8>) {
        self.key = v;
    }
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn mut_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.key
    }
    pub fn take_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    pub fn clear_value(&mut self) {
        self.value.clear();
    }
    pub fn set_value(&mut self, v: std::vec::Vec<u8>) {
        self.value = v;
    }
    pub fn get_value(&self) -> &[u8] {
        &self.value
    }
    pub fn mut_value(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.value
    }
    pub fn take_value(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }
    pub fn clear_assertion(&mut self) {
        self.assertion = 0
    }
    pub fn set_assertion_(&mut self, v: Assertion) {
        self.assertion = unsafe { ::std::mem::transmute::<Assertion, i32>(v) };
    }
    pub fn get_assertion(&self) -> Assertion {
        unsafe { ::std::mem::transmute::<i32, Assertion>(self.assertion) }
    }
}
impl ::protobuf::Clear for Mutation {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Mutation {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static Mutation {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl PrewriteRequest {
    pub fn new_() -> PrewriteRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    pub fn clear_mutations(&mut self) {
        self.mutations.clear();
    }
    pub fn set_mutations(&mut self, v: ::std::vec::Vec<Mutation>) {
        self.mutations = v;
    }
    pub fn get_mutations(&self) -> &::std::vec::Vec<Mutation> {
        &self.mutations
    }
    pub fn mut_mutations(&mut self) -> &mut ::std::vec::Vec<Mutation> {
        &mut self.mutations
    }
    pub fn take_mutations(&mut self) -> ::std::vec::Vec<Mutation> {
        ::std::mem::replace(&mut self.mutations, ::std::vec::Vec::new())
    }
    pub fn clear_primary_lock(&mut self) {
        self.primary_lock.clear();
    }
    pub fn set_primary_lock(&mut self, v: std::vec::Vec<u8>) {
        self.primary_lock = v;
    }
    pub fn get_primary_lock(&self) -> &[u8] {
        &self.primary_lock
    }
    pub fn mut_primary_lock(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.primary_lock
    }
    pub fn take_primary_lock(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.primary_lock, ::std::vec::Vec::new())
    }
    pub fn clear_start_version(&mut self) {
        self.start_version = 0
    }
    pub fn set_start_version(&mut self, v: u64) {
        self.start_version = v;
    }
    pub fn get_start_version(&self) -> u64 {
        self.start_version
    }
    pub fn clear_lock_ttl(&mut self) {
        self.lock_ttl = 0
    }
    pub fn set_lock_ttl(&mut self, v: u64) {
        self.lock_ttl = v;
    }
    pub fn get_lock_ttl(&self) -> u64 {
        self.lock_ttl
    }
    pub fn clear_skip_constraint_check(&mut self) {
        self.skip_constraint_check = false
    }
    pub fn set_skip_constraint_check(&mut self, v: bool) {
        self.skip_constraint_check = v;
    }
    pub fn get_skip_constraint_check(&self) -> bool {
        self.skip_constraint_check
    }
}
impl ::protobuf::Clear for PrewriteRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for PrewriteRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static PrewriteRequest {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl PrewriteResponse {
    pub fn new_() -> PrewriteResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> Error {
        self.region_error.take().unwrap_or_else(Error::default)
    }
    pub fn clear_errors(&mut self) {
        self.errors.clear();
    }
    pub fn set_errors(&mut self, v: ::std::vec::Vec<KeyError>) {
        self.errors = v;
    }
    pub fn get_errors(&self) -> &::std::vec::Vec<KeyError> {
        &self.errors
    }
    pub fn mut_errors(&mut self) -> &mut ::std::vec::Vec<KeyError> {
        &mut self.errors
    }
    pub fn take_errors(&mut self) -> ::std::vec::Vec<KeyError> {
        ::std::mem::replace(&mut self.errors, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for PrewriteResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for PrewriteResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static PrewriteResponse {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl CommitRequest {
    pub fn new_() -> CommitRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    pub fn clear_start_version(&mut self) {
        self.start_version = 0
    }
    pub fn set_start_version(&mut self, v: u64) {
        self.start_version = v;
    }
    pub fn get_start_version(&self) -> u64 {
        self.start_version
    }
    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }
    pub fn set_keys(&mut self, v: ::std::vec::Vec<std::vec::Vec<u8>>) {
        self.keys = v;
    }
    pub fn get_keys(&self) -> &::std::vec::Vec<std::vec::Vec<u8>> {
        &self.keys
    }
    pub fn mut_keys(&mut self) -> &mut ::std::vec::Vec<std::vec::Vec<u8>> {
        &mut self.keys
    }
    pub fn take_keys(&mut self) -> ::std::vec::Vec<std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.keys, ::std::vec::Vec::new())
    }
    pub fn clear_commit_version(&mut self) {
        self.commit_version = 0
    }
    pub fn set_commit_version(&mut self, v: u64) {
        self.commit_version = v;
    }
    pub fn get_commit_version(&self) -> u64 {
        self.commit_version
    }
}
impl ::protobuf::Clear for CommitRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for CommitRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static CommitRequest {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl CommitResponse {
    pub fn new_() -> CommitResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> Error {
        self.region_error.take().unwrap_or_else(Error::default)
    }
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None
    }
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::std::option::Option::Some(v);;    }
    pub fn get_error(&self) -> &KeyError {
        match self.error.as_ref() {
            Some(v) => v,
            None => <KeyError as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(KeyError::default());
        }
        self.error.as_mut().unwrap()
    }
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(KeyError::default)
    }
}
impl ::protobuf::Clear for CommitResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for CommitResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static CommitResponse {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl ImportRequest {
    pub fn new_() -> ImportRequest {
        ::std::default::Default::default()
    }
    pub fn clear_mutations(&mut self) {
        self.mutations.clear();
    }
    pub fn set_mutations(&mut self, v: ::std::vec::Vec<Mutation>) {
        self.mutations = v;
    }
    pub fn get_mutations(&self) -> &::std::vec::Vec<Mutation> {
        &self.mutations
    }
    pub fn mut_mutations(&mut self) -> &mut ::std::vec::Vec<Mutation> {
        &mut self.mutations
    }
    pub fn take_mutations(&mut self) -> ::std::vec::Vec<Mutation> {
        ::std::mem::replace(&mut self.mutations, ::std::vec::Vec::new())
    }
    pub fn clear_commit_version(&mut self) {
        self.commit_version = 0
    }
    pub fn set_commit_version(&mut self, v: u64) {
        self.commit_version = v;
    }
    pub fn get_commit_version(&self) -> u64 {
        self.commit_version
    }
}
impl ::protobuf::Clear for ImportRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ImportRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static ImportRequest {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl ImportResponse {
    pub fn new_() -> ImportResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> Error {
        self.region_error.take().unwrap_or_else(Error::default)
    }
    pub fn clear_error(&mut self) {
        self.error.clear();
    }
    pub fn set_error(&mut self, v: std::string::String) {
        self.error = v;
    }
    pub fn get_error(&self) -> &str {
        &self.error
    }
    pub fn mut_error(&mut self) -> &mut std::string::String {
        &mut self.error
    }
    pub fn take_error(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }
}
impl ::protobuf::Clear for ImportResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ImportResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static ImportResponse {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl BatchRollbackRequest {
    pub fn new_() -> BatchRollbackRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    pub fn clear_start_version(&mut self) {
        self.start_version = 0
    }
    pub fn set_start_version(&mut self, v: u64) {
        self.start_version = v;
    }
    pub fn get_start_version(&self) -> u64 {
        self.start_version
    }
    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }
    pub fn set_keys(&mut self, v: ::std::vec::Vec<std::vec::Vec<u8>>) {
        self.keys = v;
    }
    pub fn get_keys(&self) -> &::std::vec::Vec<std::vec::Vec<u8>> {
        &self.keys
    }
    pub fn mut_keys(&mut self) -> &mut ::std::vec::Vec<std::vec::Vec<u8>> {
        &mut self.keys
    }
    pub fn take_keys(&mut self) -> ::std::vec::Vec<std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.keys, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for BatchRollbackRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for BatchRollbackRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static BatchRollbackRequest {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl BatchRollbackResponse {
    pub fn new_() -> BatchRollbackResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> Error {
        self.region_error.take().unwrap_or_else(Error::default)
    }
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None
    }
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::std::option::Option::Some(v);;    }
    pub fn get_error(&self) -> &KeyError {
        match self.error.as_ref() {
            Some(v) => v,
            None => <KeyError as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(KeyError::default());
        }
        self.error.as_mut().unwrap()
    }
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(KeyError::default)
    }
}
impl ::protobuf::Clear for BatchRollbackResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for BatchRollbackResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static BatchRollbackResponse {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl CleanupRequest {
    pub fn new_() -> CleanupRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    pub fn set_key(&mut self, v: std::vec::Vec<u8>) {
        self.key = v;
    }
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn mut_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.key
    }
    pub fn take_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    pub fn clear_start_version(&mut self) {
        self.start_version = 0
    }
    pub fn set_start_version(&mut self, v: u64) {
        self.start_version = v;
    }
    pub fn get_start_version(&self) -> u64 {
        self.start_version
    }
}
impl ::protobuf::Clear for CleanupRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for CleanupRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static CleanupRequest {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl CleanupResponse {
    pub fn new_() -> CleanupResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> Error {
        self.region_error.take().unwrap_or_else(Error::default)
    }
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None
    }
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::std::option::Option::Some(v);;    }
    pub fn get_error(&self) -> &KeyError {
        match self.error.as_ref() {
            Some(v) => v,
            None => <KeyError as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(KeyError::default());
        }
        self.error.as_mut().unwrap()
    }
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(KeyError::default)
    }
    pub fn clear_commit_version(&mut self) {
        self.commit_version = 0
    }
    pub fn set_commit_version(&mut self, v: u64) {
        self.commit_version = v;
    }
    pub fn get_commit_version(&self) -> u64 {
        self.commit_version
    }
}
impl ::protobuf::Clear for CleanupResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for CleanupResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static CleanupResponse {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl BatchGetRequest {
    pub fn new_() -> BatchGetRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }
    pub fn set_keys(&mut self, v: ::std::vec::Vec<std::vec::Vec<u8>>) {
        self.keys = v;
    }
    pub fn get_keys(&self) -> &::std::vec::Vec<std::vec::Vec<u8>> {
        &self.keys
    }
    pub fn mut_keys(&mut self) -> &mut ::std::vec::Vec<std::vec::Vec<u8>> {
        &mut self.keys
    }
    pub fn take_keys(&mut self) -> ::std::vec::Vec<std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.keys, ::std::vec::Vec::new())
    }
    pub fn clear_version(&mut self) {
        self.version = 0
    }
    pub fn set_version(&mut self, v: u64) {
        self.version = v;
    }
    pub fn get_version(&self) -> u64 {
        self.version
    }
}
impl ::protobuf::Clear for BatchGetRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for BatchGetRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static BatchGetRequest {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl BatchGetResponse {
    pub fn new_() -> BatchGetResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> Error {
        self.region_error.take().unwrap_or_else(Error::default)
    }
    pub fn clear_pairs(&mut self) {
        self.pairs.clear();
    }
    pub fn set_pairs(&mut self, v: ::std::vec::Vec<KvPair>) {
        self.pairs = v;
    }
    pub fn get_pairs(&self) -> &::std::vec::Vec<KvPair> {
        &self.pairs
    }
    pub fn mut_pairs(&mut self) -> &mut ::std::vec::Vec<KvPair> {
        &mut self.pairs
    }
    pub fn take_pairs(&mut self) -> ::std::vec::Vec<KvPair> {
        ::std::mem::replace(&mut self.pairs, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for BatchGetResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for BatchGetResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static BatchGetResponse {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl ScanLockRequest {
    pub fn new_() -> ScanLockRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    pub fn clear_max_version(&mut self) {
        self.max_version = 0
    }
    pub fn set_max_version(&mut self, v: u64) {
        self.max_version = v;
    }
    pub fn get_max_version(&self) -> u64 {
        self.max_version
    }
    pub fn clear_start_key(&mut self) {
        self.start_key.clear();
    }
    pub fn set_start_key(&mut self, v: std::vec::Vec<u8>) {
        self.start_key = v;
    }
    pub fn get_start_key(&self) -> &[u8] {
        &self.start_key
    }
    pub fn mut_start_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.start_key
    }
    pub fn take_start_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.start_key, ::std::vec::Vec::new())
    }
    pub fn clear_limit(&mut self) {
        self.limit = 0
    }
    pub fn set_limit(&mut self, v: u32) {
        self.limit = v;
    }
    pub fn get_limit(&self) -> u32 {
        self.limit
    }
}
impl ::protobuf::Clear for ScanLockRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ScanLockRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static ScanLockRequest {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl ScanLockResponse {
    pub fn new_() -> ScanLockResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> Error {
        self.region_error.take().unwrap_or_else(Error::default)
    }
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None
    }
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::std::option::Option::Some(v);;    }
    pub fn get_error(&self) -> &KeyError {
        match self.error.as_ref() {
            Some(v) => v,
            None => <KeyError as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(KeyError::default());
        }
        self.error.as_mut().unwrap()
    }
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(KeyError::default)
    }
    pub fn clear_locks(&mut self) {
        self.locks.clear();
    }
    pub fn set_locks(&mut self, v: ::std::vec::Vec<LockInfo>) {
        self.locks = v;
    }
    pub fn get_locks(&self) -> &::std::vec::Vec<LockInfo> {
        &self.locks
    }
    pub fn mut_locks(&mut self) -> &mut ::std::vec::Vec<LockInfo> {
        &mut self.locks
    }
    pub fn take_locks(&mut self) -> ::std::vec::Vec<LockInfo> {
        ::std::mem::replace(&mut self.locks, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for ScanLockResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ScanLockResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static ScanLockResponse {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl TxnInfo {
    pub fn new_() -> TxnInfo {
        ::std::default::Default::default()
    }
    pub fn clear_txn(&mut self) {
        self.txn = 0
    }
    pub fn set_txn(&mut self, v: u64) {
        self.txn = v;
    }
    pub fn get_txn(&self) -> u64 {
        self.txn
    }
    pub fn clear_status(&mut self) {
        self.status = 0
    }
    pub fn set_status(&mut self, v: u64) {
        self.status = v;
    }
    pub fn get_status(&self) -> u64 {
        self.status
    }
}
impl ::protobuf::Clear for TxnInfo {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for TxnInfo {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static TxnInfo {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl ResolveLockRequest {
    pub fn new_() -> ResolveLockRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    pub fn clear_start_version(&mut self) {
        self.start_version = 0
    }
    pub fn set_start_version(&mut self, v: u64) {
        self.start_version = v;
    }
    pub fn get_start_version(&self) -> u64 {
        self.start_version
    }
    pub fn clear_commit_version(&mut self) {
        self.commit_version = 0
    }
    pub fn set_commit_version(&mut self, v: u64) {
        self.commit_version = v;
    }
    pub fn get_commit_version(&self) -> u64 {
        self.commit_version
    }
    pub fn clear_txn_infos(&mut self) {
        self.txn_infos.clear();
    }
    pub fn set_txn_infos(&mut self, v: ::std::vec::Vec<TxnInfo>) {
        self.txn_infos = v;
    }
    pub fn get_txn_infos(&self) -> &::std::vec::Vec<TxnInfo> {
        &self.txn_infos
    }
    pub fn mut_txn_infos(&mut self) -> &mut ::std::vec::Vec<TxnInfo> {
        &mut self.txn_infos
    }
    pub fn take_txn_infos(&mut self) -> ::std::vec::Vec<TxnInfo> {
        ::std::mem::replace(&mut self.txn_infos, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for ResolveLockRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ResolveLockRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static ResolveLockRequest {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl ResolveLockResponse {
    pub fn new_() -> ResolveLockResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> Error {
        self.region_error.take().unwrap_or_else(Error::default)
    }
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None
    }
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::std::option::Option::Some(v);;    }
    pub fn get_error(&self) -> &KeyError {
        match self.error.as_ref() {
            Some(v) => v,
            None => <KeyError as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(KeyError::default());
        }
        self.error.as_mut().unwrap()
    }
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(KeyError::default)
    }
}
impl ::protobuf::Clear for ResolveLockResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ResolveLockResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static ResolveLockResponse {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl GcRequest {
    pub fn new_() -> GcRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    pub fn clear_safe_point(&mut self) {
        self.safe_point = 0
    }
    pub fn set_safe_point(&mut self, v: u64) {
        self.safe_point = v;
    }
    pub fn get_safe_point(&self) -> u64 {
        self.safe_point
    }
}
impl ::protobuf::Clear for GcRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for GcRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static GcRequest {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl GcResponse {
    pub fn new_() -> GcResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> Error {
        self.region_error.take().unwrap_or_else(Error::default)
    }
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None
    }
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::std::option::Option::Some(v);;    }
    pub fn get_error(&self) -> &KeyError {
        match self.error.as_ref() {
            Some(v) => v,
            None => <KeyError as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(KeyError::default());
        }
        self.error.as_mut().unwrap()
    }
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(KeyError::default)
    }
}
impl ::protobuf::Clear for GcResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for GcResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static GcResponse {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl RawGetRequest {
    pub fn new_() -> RawGetRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    pub fn set_key(&mut self, v: std::vec::Vec<u8>) {
        self.key = v;
    }
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn mut_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.key
    }
    pub fn take_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }
    pub fn set_cf(&mut self, v: std::string::String) {
        self.cf = v;
    }
    pub fn get_cf(&self) -> &str {
        &self.cf
    }
    pub fn mut_cf(&mut self) -> &mut std::string::String {
        &mut self.cf
    }
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
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static RawGetRequest {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl RawGetResponse {
    pub fn new_() -> RawGetResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> Error {
        self.region_error.take().unwrap_or_else(Error::default)
    }
    pub fn clear_error(&mut self) {
        self.error.clear();
    }
    pub fn set_error(&mut self, v: std::string::String) {
        self.error = v;
    }
    pub fn get_error(&self) -> &str {
        &self.error
    }
    pub fn mut_error(&mut self) -> &mut std::string::String {
        &mut self.error
    }
    pub fn take_error(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }
    pub fn clear_value(&mut self) {
        self.value.clear();
    }
    pub fn set_value(&mut self, v: std::vec::Vec<u8>) {
        self.value = v;
    }
    pub fn get_value(&self) -> &[u8] {
        &self.value
    }
    pub fn mut_value(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.value
    }
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
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static RawGetResponse {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl RawPutRequest {
    pub fn new_() -> RawPutRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    pub fn set_key(&mut self, v: std::vec::Vec<u8>) {
        self.key = v;
    }
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn mut_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.key
    }
    pub fn take_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    pub fn clear_value(&mut self) {
        self.value.clear();
    }
    pub fn set_value(&mut self, v: std::vec::Vec<u8>) {
        self.value = v;
    }
    pub fn get_value(&self) -> &[u8] {
        &self.value
    }
    pub fn mut_value(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.value
    }
    pub fn take_value(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }
    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }
    pub fn set_cf(&mut self, v: std::string::String) {
        self.cf = v;
    }
    pub fn get_cf(&self) -> &str {
        &self.cf
    }
    pub fn mut_cf(&mut self) -> &mut std::string::String {
        &mut self.cf
    }
    pub fn take_cf(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.cf, ::std::string::String::new())
    }
}
impl ::protobuf::Clear for RawPutRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RawPutRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static RawPutRequest {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl RawPutResponse {
    pub fn new_() -> RawPutResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> Error {
        self.region_error.take().unwrap_or_else(Error::default)
    }
    pub fn clear_error(&mut self) {
        self.error.clear();
    }
    pub fn set_error(&mut self, v: std::string::String) {
        self.error = v;
    }
    pub fn get_error(&self) -> &str {
        &self.error
    }
    pub fn mut_error(&mut self) -> &mut std::string::String {
        &mut self.error
    }
    pub fn take_error(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }
}
impl ::protobuf::Clear for RawPutResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RawPutResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static RawPutResponse {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl RawBatchPutRequest {
    pub fn new_() -> RawBatchPutRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    pub fn clear_pairs(&mut self) {
        self.pairs.clear();
    }
    pub fn set_pairs(&mut self, v: ::std::vec::Vec<KvPair>) {
        self.pairs = v;
    }
    pub fn get_pairs(&self) -> &::std::vec::Vec<KvPair> {
        &self.pairs
    }
    pub fn mut_pairs(&mut self) -> &mut ::std::vec::Vec<KvPair> {
        &mut self.pairs
    }
    pub fn take_pairs(&mut self) -> ::std::vec::Vec<KvPair> {
        ::std::mem::replace(&mut self.pairs, ::std::vec::Vec::new())
    }
    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }
    pub fn set_cf(&mut self, v: std::string::String) {
        self.cf = v;
    }
    pub fn get_cf(&self) -> &str {
        &self.cf
    }
    pub fn mut_cf(&mut self) -> &mut std::string::String {
        &mut self.cf
    }
    pub fn take_cf(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.cf, ::std::string::String::new())
    }
}
impl ::protobuf::Clear for RawBatchPutRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RawBatchPutRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static RawBatchPutRequest {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl RawBatchPutResponse {
    pub fn new_() -> RawBatchPutResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> Error {
        self.region_error.take().unwrap_or_else(Error::default)
    }
    pub fn clear_error(&mut self) {
        self.error.clear();
    }
    pub fn set_error(&mut self, v: std::string::String) {
        self.error = v;
    }
    pub fn get_error(&self) -> &str {
        &self.error
    }
    pub fn mut_error(&mut self) -> &mut std::string::String {
        &mut self.error
    }
    pub fn take_error(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }
}
impl ::protobuf::Clear for RawBatchPutResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RawBatchPutResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static RawBatchPutResponse {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl RawBatchGetRequest {
    pub fn new_() -> RawBatchGetRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }
    pub fn set_keys(&mut self, v: ::std::vec::Vec<std::vec::Vec<u8>>) {
        self.keys = v;
    }
    pub fn get_keys(&self) -> &::std::vec::Vec<std::vec::Vec<u8>> {
        &self.keys
    }
    pub fn mut_keys(&mut self) -> &mut ::std::vec::Vec<std::vec::Vec<u8>> {
        &mut self.keys
    }
    pub fn take_keys(&mut self) -> ::std::vec::Vec<std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.keys, ::std::vec::Vec::new())
    }
    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }
    pub fn set_cf(&mut self, v: std::string::String) {
        self.cf = v;
    }
    pub fn get_cf(&self) -> &str {
        &self.cf
    }
    pub fn mut_cf(&mut self) -> &mut std::string::String {
        &mut self.cf
    }
    pub fn take_cf(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.cf, ::std::string::String::new())
    }
}
impl ::protobuf::Clear for RawBatchGetRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RawBatchGetRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static RawBatchGetRequest {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl RawBatchGetResponse {
    pub fn new_() -> RawBatchGetResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> Error {
        self.region_error.take().unwrap_or_else(Error::default)
    }
    pub fn clear_pairs(&mut self) {
        self.pairs.clear();
    }
    pub fn set_pairs(&mut self, v: ::std::vec::Vec<KvPair>) {
        self.pairs = v;
    }
    pub fn get_pairs(&self) -> &::std::vec::Vec<KvPair> {
        &self.pairs
    }
    pub fn mut_pairs(&mut self) -> &mut ::std::vec::Vec<KvPair> {
        &mut self.pairs
    }
    pub fn take_pairs(&mut self) -> ::std::vec::Vec<KvPair> {
        ::std::mem::replace(&mut self.pairs, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for RawBatchGetResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RawBatchGetResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static RawBatchGetResponse {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl RawDeleteRequest {
    pub fn new_() -> RawDeleteRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    pub fn set_key(&mut self, v: std::vec::Vec<u8>) {
        self.key = v;
    }
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn mut_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.key
    }
    pub fn take_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }
    pub fn set_cf(&mut self, v: std::string::String) {
        self.cf = v;
    }
    pub fn get_cf(&self) -> &str {
        &self.cf
    }
    pub fn mut_cf(&mut self) -> &mut std::string::String {
        &mut self.cf
    }
    pub fn take_cf(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.cf, ::std::string::String::new())
    }
}
impl ::protobuf::Clear for RawDeleteRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RawDeleteRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static RawDeleteRequest {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl RawDeleteResponse {
    pub fn new_() -> RawDeleteResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> Error {
        self.region_error.take().unwrap_or_else(Error::default)
    }
    pub fn clear_error(&mut self) {
        self.error.clear();
    }
    pub fn set_error(&mut self, v: std::string::String) {
        self.error = v;
    }
    pub fn get_error(&self) -> &str {
        &self.error
    }
    pub fn mut_error(&mut self) -> &mut std::string::String {
        &mut self.error
    }
    pub fn take_error(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }
}
impl ::protobuf::Clear for RawDeleteResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RawDeleteResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static RawDeleteResponse {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl RawBatchDeleteRequest {
    pub fn new_() -> RawBatchDeleteRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }
    pub fn set_keys(&mut self, v: ::std::vec::Vec<std::vec::Vec<u8>>) {
        self.keys = v;
    }
    pub fn get_keys(&self) -> &::std::vec::Vec<std::vec::Vec<u8>> {
        &self.keys
    }
    pub fn mut_keys(&mut self) -> &mut ::std::vec::Vec<std::vec::Vec<u8>> {
        &mut self.keys
    }
    pub fn take_keys(&mut self) -> ::std::vec::Vec<std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.keys, ::std::vec::Vec::new())
    }
    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }
    pub fn set_cf(&mut self, v: std::string::String) {
        self.cf = v;
    }
    pub fn get_cf(&self) -> &str {
        &self.cf
    }
    pub fn mut_cf(&mut self) -> &mut std::string::String {
        &mut self.cf
    }
    pub fn take_cf(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.cf, ::std::string::String::new())
    }
}
impl ::protobuf::Clear for RawBatchDeleteRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RawBatchDeleteRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static RawBatchDeleteRequest {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl RawBatchDeleteResponse {
    pub fn new_() -> RawBatchDeleteResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> Error {
        self.region_error.take().unwrap_or_else(Error::default)
    }
    pub fn clear_error(&mut self) {
        self.error.clear();
    }
    pub fn set_error(&mut self, v: std::string::String) {
        self.error = v;
    }
    pub fn get_error(&self) -> &str {
        &self.error
    }
    pub fn mut_error(&mut self) -> &mut std::string::String {
        &mut self.error
    }
    pub fn take_error(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }
}
impl ::protobuf::Clear for RawBatchDeleteResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RawBatchDeleteResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static RawBatchDeleteResponse {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl DeleteRangeRequest {
    pub fn new_() -> DeleteRangeRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    pub fn clear_start_key(&mut self) {
        self.start_key.clear();
    }
    pub fn set_start_key(&mut self, v: std::vec::Vec<u8>) {
        self.start_key = v;
    }
    pub fn get_start_key(&self) -> &[u8] {
        &self.start_key
    }
    pub fn mut_start_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.start_key
    }
    pub fn take_start_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.start_key, ::std::vec::Vec::new())
    }
    pub fn clear_end_key(&mut self) {
        self.end_key.clear();
    }
    pub fn set_end_key(&mut self, v: std::vec::Vec<u8>) {
        self.end_key = v;
    }
    pub fn get_end_key(&self) -> &[u8] {
        &self.end_key
    }
    pub fn mut_end_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.end_key
    }
    pub fn take_end_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.end_key, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for DeleteRangeRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for DeleteRangeRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static DeleteRangeRequest {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl DeleteRangeResponse {
    pub fn new_() -> DeleteRangeResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> Error {
        self.region_error.take().unwrap_or_else(Error::default)
    }
    pub fn clear_error(&mut self) {
        self.error.clear();
    }
    pub fn set_error(&mut self, v: std::string::String) {
        self.error = v;
    }
    pub fn get_error(&self) -> &str {
        &self.error
    }
    pub fn mut_error(&mut self) -> &mut std::string::String {
        &mut self.error
    }
    pub fn take_error(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }
}
impl ::protobuf::Clear for DeleteRangeResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for DeleteRangeResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static DeleteRangeResponse {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl NotLeader {
    pub fn new_() -> NotLeader {
        ::std::default::Default::default()
    }
    pub fn clear_region_id(&mut self) {
        self.region_id = 0
    }
    pub fn set_region_id(&mut self, v: u64) {
        self.region_id = v;
    }
    pub fn get_region_id(&self) -> u64 {
        self.region_id
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
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static NotLeader {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl StoreNotMatch {
    pub fn new_() -> StoreNotMatch {
        ::std::default::Default::default()
    }
    pub fn clear_request_store_id(&mut self) {
        self.request_store_id = 0
    }
    pub fn set_request_store_id(&mut self, v: u64) {
        self.request_store_id = v;
    }
    pub fn get_request_store_id(&self) -> u64 {
        self.request_store_id
    }
    pub fn clear_actual_store_id(&mut self) {
        self.actual_store_id = 0
    }
    pub fn set_actual_store_id(&mut self, v: u64) {
        self.actual_store_id = v;
    }
    pub fn get_actual_store_id(&self) -> u64 {
        self.actual_store_id
    }
}
impl ::protobuf::Clear for StoreNotMatch {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for StoreNotMatch {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static StoreNotMatch {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl RegionNotFound {
    pub fn new_() -> RegionNotFound {
        ::std::default::Default::default()
    }
    pub fn clear_region_id(&mut self) {
        self.region_id = 0
    }
    pub fn set_region_id(&mut self, v: u64) {
        self.region_id = v;
    }
    pub fn get_region_id(&self) -> u64 {
        self.region_id
    }
}
impl ::protobuf::Clear for RegionNotFound {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RegionNotFound {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static RegionNotFound {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl KeyNotInRegion {
    pub fn new_() -> KeyNotInRegion {
        ::std::default::Default::default()
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    pub fn set_key(&mut self, v: std::vec::Vec<u8>) {
        self.key = v;
    }
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn mut_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.key
    }
    pub fn take_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    pub fn clear_region_id(&mut self) {
        self.region_id = 0
    }
    pub fn set_region_id(&mut self, v: u64) {
        self.region_id = v;
    }
    pub fn get_region_id(&self) -> u64 {
        self.region_id
    }
    pub fn clear_start_key(&mut self) {
        self.start_key.clear();
    }
    pub fn set_start_key(&mut self, v: std::vec::Vec<u8>) {
        self.start_key = v;
    }
    pub fn get_start_key(&self) -> &[u8] {
        &self.start_key
    }
    pub fn mut_start_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.start_key
    }
    pub fn take_start_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.start_key, ::std::vec::Vec::new())
    }
    pub fn clear_end_key(&mut self) {
        self.end_key.clear();
    }
    pub fn set_end_key(&mut self, v: std::vec::Vec<u8>) {
        self.end_key = v;
    }
    pub fn get_end_key(&self) -> &[u8] {
        &self.end_key
    }
    pub fn mut_end_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.end_key
    }
    pub fn take_end_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.end_key, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for KeyNotInRegion {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for KeyNotInRegion {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static KeyNotInRegion {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl ServerIsBusy {
    pub fn new_() -> ServerIsBusy {
        ::std::default::Default::default()
    }
    pub fn clear_reason(&mut self) {
        self.reason.clear();
    }
    pub fn set_reason(&mut self, v: std::string::String) {
        self.reason = v;
    }
    pub fn get_reason(&self) -> &str {
        &self.reason
    }
    pub fn mut_reason(&mut self) -> &mut std::string::String {
        &mut self.reason
    }
    pub fn take_reason(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.reason, ::std::string::String::new())
    }
    pub fn clear_backoff_ms(&mut self) {
        self.backoff_ms = 0
    }
    pub fn set_backoff_ms(&mut self, v: u64) {
        self.backoff_ms = v;
    }
    pub fn get_backoff_ms(&self) -> u64 {
        self.backoff_ms
    }
}
impl ::protobuf::Clear for ServerIsBusy {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ServerIsBusy {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static ServerIsBusy {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl StaleCommand {
    pub fn new_() -> StaleCommand {
        ::std::default::Default::default()
    }
}
impl ::protobuf::Clear for StaleCommand {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for StaleCommand {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static StaleCommand {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl RaftEntryTooLarge {
    pub fn new_() -> RaftEntryTooLarge {
        ::std::default::Default::default()
    }
    pub fn clear_region_id(&mut self) {
        self.region_id = 0
    }
    pub fn set_region_id(&mut self, v: u64) {
        self.region_id = v;
    }
    pub fn get_region_id(&self) -> u64 {
        self.region_id
    }
    pub fn clear_entry_size(&mut self) {
        self.entry_size = 0
    }
    pub fn set_entry_size(&mut self, v: u64) {
        self.entry_size = v;
    }
    pub fn get_entry_size(&self) -> u64 {
        self.entry_size
    }
}
impl ::protobuf::Clear for RaftEntryTooLarge {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RaftEntryTooLarge {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static RaftEntryTooLarge {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl Error {
    pub fn new_() -> Error {
        ::std::default::Default::default()
    }
    pub fn clear_message(&mut self) {
        self.message.clear();
    }
    pub fn set_message(&mut self, v: std::string::String) {
        self.message = v;
    }
    pub fn get_message(&self) -> &str {
        &self.message
    }
    pub fn mut_message(&mut self) -> &mut std::string::String {
        &mut self.message
    }
    pub fn take_message(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.message, ::std::string::String::new())
    }
    pub fn has_not_leader(&self) -> bool {
        self.not_leader.is_some()
    }
    pub fn clear_not_leader(&mut self) {
        self.not_leader = ::std::option::Option::None
    }
    pub fn set_not_leader(&mut self, v: NotLeader) {
        self.not_leader = ::std::option::Option::Some(v);;    }
    pub fn get_not_leader(&self) -> &NotLeader {
        match self.not_leader.as_ref() {
            Some(v) => v,
            None => <NotLeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_not_leader(&mut self) -> &mut NotLeader {
        if self.not_leader.is_none() {
            self.not_leader = ::std::option::Option::Some(NotLeader::default());
        }
        self.not_leader.as_mut().unwrap()
    }
    pub fn take_not_leader(&mut self) -> NotLeader {
        self.not_leader.take().unwrap_or_else(NotLeader::default)
    }
    pub fn has_region_not_found(&self) -> bool {
        self.region_not_found.is_some()
    }
    pub fn clear_region_not_found(&mut self) {
        self.region_not_found = ::std::option::Option::None
    }
    pub fn set_region_not_found(&mut self, v: RegionNotFound) {
        self.region_not_found = ::std::option::Option::Some(v);;    }
    pub fn get_region_not_found(&self) -> &RegionNotFound {
        match self.region_not_found.as_ref() {
            Some(v) => v,
            None => <RegionNotFound as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_not_found(&mut self) -> &mut RegionNotFound {
        if self.region_not_found.is_none() {
            self.region_not_found = ::std::option::Option::Some(RegionNotFound::default());
        }
        self.region_not_found.as_mut().unwrap()
    }
    pub fn take_region_not_found(&mut self) -> RegionNotFound {
        self.region_not_found
            .take()
            .unwrap_or_else(RegionNotFound::default)
    }
    pub fn has_key_not_in_region(&self) -> bool {
        self.key_not_in_region.is_some()
    }
    pub fn clear_key_not_in_region(&mut self) {
        self.key_not_in_region = ::std::option::Option::None
    }
    pub fn set_key_not_in_region(&mut self, v: KeyNotInRegion) {
        self.key_not_in_region = ::std::option::Option::Some(v);;    }
    pub fn get_key_not_in_region(&self) -> &KeyNotInRegion {
        match self.key_not_in_region.as_ref() {
            Some(v) => v,
            None => <KeyNotInRegion as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_key_not_in_region(&mut self) -> &mut KeyNotInRegion {
        if self.key_not_in_region.is_none() {
            self.key_not_in_region = ::std::option::Option::Some(KeyNotInRegion::default());
        }
        self.key_not_in_region.as_mut().unwrap()
    }
    pub fn take_key_not_in_region(&mut self) -> KeyNotInRegion {
        self.key_not_in_region
            .take()
            .unwrap_or_else(KeyNotInRegion::default)
    }
    pub fn has_server_is_busy(&self) -> bool {
        self.server_is_busy.is_some()
    }
    pub fn clear_server_is_busy(&mut self) {
        self.server_is_busy = ::std::option::Option::None
    }
    pub fn set_server_is_busy(&mut self, v: ServerIsBusy) {
        self.server_is_busy = ::std::option::Option::Some(v);;    }
    pub fn get_server_is_busy(&self) -> &ServerIsBusy {
        match self.server_is_busy.as_ref() {
            Some(v) => v,
            None => <ServerIsBusy as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_server_is_busy(&mut self) -> &mut ServerIsBusy {
        if self.server_is_busy.is_none() {
            self.server_is_busy = ::std::option::Option::Some(ServerIsBusy::default());
        }
        self.server_is_busy.as_mut().unwrap()
    }
    pub fn take_server_is_busy(&mut self) -> ServerIsBusy {
        self.server_is_busy
            .take()
            .unwrap_or_else(ServerIsBusy::default)
    }
    pub fn has_stale_command(&self) -> bool {
        self.stale_command.is_some()
    }
    pub fn clear_stale_command(&mut self) {
        self.stale_command = ::std::option::Option::None
    }
    pub fn set_stale_command(&mut self, v: StaleCommand) {
        self.stale_command = ::std::option::Option::Some(v);;    }
    pub fn get_stale_command(&self) -> &StaleCommand {
        match self.stale_command.as_ref() {
            Some(v) => v,
            None => <StaleCommand as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_stale_command(&mut self) -> &mut StaleCommand {
        if self.stale_command.is_none() {
            self.stale_command = ::std::option::Option::Some(StaleCommand::default());
        }
        self.stale_command.as_mut().unwrap()
    }
    pub fn take_stale_command(&mut self) -> StaleCommand {
        self.stale_command
            .take()
            .unwrap_or_else(StaleCommand::default)
    }
    pub fn has_store_not_match(&self) -> bool {
        self.store_not_match.is_some()
    }
    pub fn clear_store_not_match(&mut self) {
        self.store_not_match = ::std::option::Option::None
    }
    pub fn set_store_not_match(&mut self, v: StoreNotMatch) {
        self.store_not_match = ::std::option::Option::Some(v);;    }
    pub fn get_store_not_match(&self) -> &StoreNotMatch {
        match self.store_not_match.as_ref() {
            Some(v) => v,
            None => <StoreNotMatch as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_store_not_match(&mut self) -> &mut StoreNotMatch {
        if self.store_not_match.is_none() {
            self.store_not_match = ::std::option::Option::Some(StoreNotMatch::default());
        }
        self.store_not_match.as_mut().unwrap()
    }
    pub fn take_store_not_match(&mut self) -> StoreNotMatch {
        self.store_not_match
            .take()
            .unwrap_or_else(StoreNotMatch::default)
    }
    pub fn has_raft_entry_too_large(&self) -> bool {
        self.raft_entry_too_large.is_some()
    }
    pub fn clear_raft_entry_too_large(&mut self) {
        self.raft_entry_too_large = ::std::option::Option::None
    }
    pub fn set_raft_entry_too_large(&mut self, v: RaftEntryTooLarge) {
        self.raft_entry_too_large = ::std::option::Option::Some(v);;    }
    pub fn get_raft_entry_too_large(&self) -> &RaftEntryTooLarge {
        match self.raft_entry_too_large.as_ref() {
            Some(v) => v,
            None => <RaftEntryTooLarge as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_raft_entry_too_large(&mut self) -> &mut RaftEntryTooLarge {
        if self.raft_entry_too_large.is_none() {
            self.raft_entry_too_large = ::std::option::Option::Some(RaftEntryTooLarge::default());
        }
        self.raft_entry_too_large.as_mut().unwrap()
    }
    pub fn take_raft_entry_too_large(&mut self) -> RaftEntryTooLarge {
        self.raft_entry_too_large
            .take()
            .unwrap_or_else(RaftEntryTooLarge::default)
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
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static Error {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl Context {
    pub fn new_() -> Context {
        ::std::default::Default::default()
    }
    pub fn clear_region_id(&mut self) {
        self.region_id = 0
    }
    pub fn set_region_id(&mut self, v: u64) {
        self.region_id = v;
    }
    pub fn get_region_id(&self) -> u64 {
        self.region_id
    }
    pub fn has_region_epoch(&self) -> bool {
        self.region_epoch.is_some()
    }
    pub fn clear_region_epoch(&mut self) {
        self.region_epoch = ::std::option::Option::None
    }
    pub fn set_region_epoch(&mut self, v: RegionEpoch) {
        self.region_epoch = ::std::option::Option::Some(v);;    }
    pub fn get_region_epoch(&self) -> &RegionEpoch {
        match self.region_epoch.as_ref() {
            Some(v) => v,
            None => <RegionEpoch as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_epoch(&mut self) -> &mut RegionEpoch {
        if self.region_epoch.is_none() {
            self.region_epoch = ::std::option::Option::Some(RegionEpoch::default());
        }
        self.region_epoch.as_mut().unwrap()
    }
    pub fn take_region_epoch(&mut self) -> RegionEpoch {
        self.region_epoch
            .take()
            .unwrap_or_else(RegionEpoch::default)
    }
    pub fn has_peer(&self) -> bool {
        self.peer.is_some()
    }
    pub fn clear_peer(&mut self) {
        self.peer = ::std::option::Option::None
    }
    pub fn set_peer(&mut self, v: Peer) {
        self.peer = ::std::option::Option::Some(v);;    }
    pub fn get_peer(&self) -> &Peer {
        match self.peer.as_ref() {
            Some(v) => v,
            None => <Peer as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_peer(&mut self) -> &mut Peer {
        if self.peer.is_none() {
            self.peer = ::std::option::Option::Some(Peer::default());
        }
        self.peer.as_mut().unwrap()
    }
    pub fn take_peer(&mut self) -> Peer {
        self.peer.take().unwrap_or_else(Peer::default)
    }
    pub fn clear_term(&mut self) {
        self.term = 0
    }
    pub fn set_term(&mut self, v: u64) {
        self.term = v;
    }
    pub fn get_term(&self) -> u64 {
        self.term
    }
    pub fn clear_priority(&mut self) {
        self.priority = 0
    }
    pub fn set_priority_(&mut self, v: CommandPri) {
        self.priority = unsafe { ::std::mem::transmute::<CommandPri, i32>(v) };
    }
    pub fn get_priority(&self) -> CommandPri {
        unsafe { ::std::mem::transmute::<i32, CommandPri>(self.priority) }
    }
    pub fn clear_isolation_level(&mut self) {
        self.isolation_level = 0
    }
    pub fn set_isolation_level_(&mut self, v: IsolationLevel) {
        self.isolation_level = unsafe { ::std::mem::transmute::<IsolationLevel, i32>(v) };
    }
    pub fn get_isolation_level(&self) -> IsolationLevel {
        unsafe { ::std::mem::transmute::<i32, IsolationLevel>(self.isolation_level) }
    }
    pub fn clear_not_fill_cache(&mut self) {
        self.not_fill_cache = false
    }
    pub fn set_not_fill_cache(&mut self, v: bool) {
        self.not_fill_cache = v;
    }
    pub fn get_not_fill_cache(&self) -> bool {
        self.not_fill_cache
    }
    pub fn clear_sync_log(&mut self) {
        self.sync_log = false
    }
    pub fn set_sync_log(&mut self, v: bool) {
        self.sync_log = v;
    }
    pub fn get_sync_log(&self) -> bool {
        self.sync_log
    }
    pub fn clear_handle_time(&mut self) {
        self.handle_time = false
    }
    pub fn set_handle_time(&mut self, v: bool) {
        self.handle_time = v;
    }
    pub fn get_handle_time(&self) -> bool {
        self.handle_time
    }
    pub fn clear_scan_detail(&mut self) {
        self.scan_detail = false
    }
    pub fn set_scan_detail(&mut self, v: bool) {
        self.scan_detail = v;
    }
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
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static Context {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl RegionEpoch {
    pub fn new_() -> RegionEpoch {
        ::std::default::Default::default()
    }
    pub fn clear_conf_ver(&mut self) {
        self.conf_ver = 0
    }
    pub fn set_conf_ver(&mut self, v: u64) {
        self.conf_ver = v;
    }
    pub fn get_conf_ver(&self) -> u64 {
        self.conf_ver
    }
    pub fn clear_version(&mut self) {
        self.version = 0
    }
    pub fn set_version(&mut self, v: u64) {
        self.version = v;
    }
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
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static RegionEpoch {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl Peer {
    pub fn new_() -> Peer {
        ::std::default::Default::default()
    }
    pub fn clear_id(&mut self) {
        self.id = 0
    }
    pub fn set_id(&mut self, v: u64) {
        self.id = v;
    }
    pub fn get_id(&self) -> u64 {
        self.id
    }
    pub fn clear_store_id(&mut self) {
        self.store_id = 0
    }
    pub fn set_store_id(&mut self, v: u64) {
        self.store_id = v;
    }
    pub fn get_store_id(&self) -> u64 {
        self.store_id
    }
    pub fn clear_is_learner(&mut self) {
        self.is_learner = false
    }
    pub fn set_is_learner(&mut self, v: bool) {
        self.is_learner = v;
    }
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
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static Peer {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl KeyError {
    pub fn new_() -> KeyError {
        ::std::default::Default::default()
    }
    pub fn has_locked(&self) -> bool {
        self.locked.is_some()
    }
    pub fn clear_locked(&mut self) {
        self.locked = ::std::option::Option::None
    }
    pub fn set_locked(&mut self, v: LockInfo) {
        self.locked = ::std::option::Option::Some(v);;    }
    pub fn get_locked(&self) -> &LockInfo {
        match self.locked.as_ref() {
            Some(v) => v,
            None => <LockInfo as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_locked(&mut self) -> &mut LockInfo {
        if self.locked.is_none() {
            self.locked = ::std::option::Option::Some(LockInfo::default());
        }
        self.locked.as_mut().unwrap()
    }
    pub fn take_locked(&mut self) -> LockInfo {
        self.locked.take().unwrap_or_else(LockInfo::default)
    }
    pub fn clear_retryable(&mut self) {
        self.retryable.clear();
    }
    pub fn set_retryable(&mut self, v: std::string::String) {
        self.retryable = v;
    }
    pub fn get_retryable(&self) -> &str {
        &self.retryable
    }
    pub fn mut_retryable(&mut self) -> &mut std::string::String {
        &mut self.retryable
    }
    pub fn take_retryable(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.retryable, ::std::string::String::new())
    }
    pub fn clear_abort(&mut self) {
        self.abort.clear();
    }
    pub fn set_abort(&mut self, v: std::string::String) {
        self.abort = v;
    }
    pub fn get_abort(&self) -> &str {
        &self.abort
    }
    pub fn mut_abort(&mut self) -> &mut std::string::String {
        &mut self.abort
    }
    pub fn take_abort(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.abort, ::std::string::String::new())
    }
    pub fn has_conflict(&self) -> bool {
        self.conflict.is_some()
    }
    pub fn clear_conflict(&mut self) {
        self.conflict = ::std::option::Option::None
    }
    pub fn set_conflict(&mut self, v: WriteConflict) {
        self.conflict = ::std::option::Option::Some(v);;    }
    pub fn get_conflict(&self) -> &WriteConflict {
        match self.conflict.as_ref() {
            Some(v) => v,
            None => <WriteConflict as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_conflict(&mut self) -> &mut WriteConflict {
        if self.conflict.is_none() {
            self.conflict = ::std::option::Option::Some(WriteConflict::default());
        }
        self.conflict.as_mut().unwrap()
    }
    pub fn take_conflict(&mut self) -> WriteConflict {
        self.conflict.take().unwrap_or_else(WriteConflict::default)
    }
    pub fn has_already_exist(&self) -> bool {
        self.already_exist.is_some()
    }
    pub fn clear_already_exist(&mut self) {
        self.already_exist = ::std::option::Option::None
    }
    pub fn set_already_exist(&mut self, v: AlreadyExist) {
        self.already_exist = ::std::option::Option::Some(v);;    }
    pub fn get_already_exist(&self) -> &AlreadyExist {
        match self.already_exist.as_ref() {
            Some(v) => v,
            None => <AlreadyExist as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_already_exist(&mut self) -> &mut AlreadyExist {
        if self.already_exist.is_none() {
            self.already_exist = ::std::option::Option::Some(AlreadyExist::default());
        }
        self.already_exist.as_mut().unwrap()
    }
    pub fn take_already_exist(&mut self) -> AlreadyExist {
        self.already_exist
            .take()
            .unwrap_or_else(AlreadyExist::default)
    }
}
impl ::protobuf::Clear for KeyError {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for KeyError {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static KeyError {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl LockInfo {
    pub fn new_() -> LockInfo {
        ::std::default::Default::default()
    }
    pub fn clear_primary_lock(&mut self) {
        self.primary_lock.clear();
    }
    pub fn set_primary_lock(&mut self, v: std::vec::Vec<u8>) {
        self.primary_lock = v;
    }
    pub fn get_primary_lock(&self) -> &[u8] {
        &self.primary_lock
    }
    pub fn mut_primary_lock(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.primary_lock
    }
    pub fn take_primary_lock(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.primary_lock, ::std::vec::Vec::new())
    }
    pub fn clear_lock_version(&mut self) {
        self.lock_version = 0
    }
    pub fn set_lock_version(&mut self, v: u64) {
        self.lock_version = v;
    }
    pub fn get_lock_version(&self) -> u64 {
        self.lock_version
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    pub fn set_key(&mut self, v: std::vec::Vec<u8>) {
        self.key = v;
    }
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn mut_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.key
    }
    pub fn take_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    pub fn clear_lock_ttl(&mut self) {
        self.lock_ttl = 0
    }
    pub fn set_lock_ttl(&mut self, v: u64) {
        self.lock_ttl = v;
    }
    pub fn get_lock_ttl(&self) -> u64 {
        self.lock_ttl
    }
}
impl ::protobuf::Clear for LockInfo {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for LockInfo {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static LockInfo {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl AlreadyExist {
    pub fn new_() -> AlreadyExist {
        ::std::default::Default::default()
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    pub fn set_key(&mut self, v: std::vec::Vec<u8>) {
        self.key = v;
    }
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn mut_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.key
    }
    pub fn take_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for AlreadyExist {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for AlreadyExist {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static AlreadyExist {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
impl WriteConflict {
    pub fn new_() -> WriteConflict {
        ::std::default::Default::default()
    }
    pub fn clear_start_ts(&mut self) {
        self.start_ts = 0
    }
    pub fn set_start_ts(&mut self, v: u64) {
        self.start_ts = v;
    }
    pub fn get_start_ts(&self) -> u64 {
        self.start_ts
    }
    pub fn clear_conflict_ts(&mut self) {
        self.conflict_ts = 0
    }
    pub fn set_conflict_ts(&mut self, v: u64) {
        self.conflict_ts = v;
    }
    pub fn get_conflict_ts(&self) -> u64 {
        self.conflict_ts
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    pub fn set_key(&mut self, v: std::vec::Vec<u8>) {
        self.key = v;
    }
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn mut_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.key
    }
    pub fn take_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    pub fn clear_primary(&mut self) {
        self.primary.clear();
    }
    pub fn set_primary(&mut self, v: std::vec::Vec<u8>) {
        self.primary = v;
    }
    pub fn get_primary(&self) -> &[u8] {
        &self.primary
    }
    pub fn mut_primary(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.primary
    }
    pub fn take_primary(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.primary, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for WriteConflict {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for WriteConflict {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn default_instance() -> &'static WriteConflict {
        unimplemented!("dsfsf");
    }
    fn is_initialized(&self) -> bool {
        unimplemented!("dsfsf");
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!("dsfsf");
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!("dsfsf");
    }
}
