#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalInt32 {
    #[prost(sint32, tag="1")]
    pub val: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalUInt32 {
    #[prost(uint32, tag="1")]
    pub val: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int32 {
    #[prost(sint32, tag="1")]
    pub val: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UInt32 {
    #[prost(uint32, tag="1")]
    pub val: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int64 {
    #[prost(int64, tag="1")]
    pub val: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct String {
    #[prost(string, tag="1")]
    pub val: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Json {
    #[prost(bytes="vec", tag="1")]
    pub json: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bool {
    #[prost(bool, tag="1")]
    pub val: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringList {
    #[prost(string, repeated, tag="1")]
    pub vals: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
