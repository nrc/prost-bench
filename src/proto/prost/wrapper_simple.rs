impl Data {
    pub fn new_() -> Data {
        ::std::default::Default::default()
    }
    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }
    pub fn set_payload(&mut self, v: std::vec::Vec<u8>) {
        self.payload = v;
    }
    pub fn get_payload(&self) -> &[u8] {
        &self.payload
    }
    pub fn mut_payload(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.payload
    }
    pub fn take_payload(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.payload, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for Data {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Data {
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
        unimplemented!();
    }
    fn default_instance() -> &'static Data {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
        unimplemented!();
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
}
