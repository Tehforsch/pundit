#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenCollectionRequest {
    #[prost(string, tag="1")]
    pub collection_path: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub media_folder_path: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub media_db_path: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub log_path: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseCollectionRequest {
    #[prost(bool, tag="1")]
    pub downgrade_to_schema11: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckDatabaseResponse {
    #[prost(string, repeated, tag="1")]
    pub problems: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpChanges {
    #[prost(bool, tag="1")]
    pub card: bool,
    #[prost(bool, tag="2")]
    pub note: bool,
    #[prost(bool, tag="3")]
    pub deck: bool,
    #[prost(bool, tag="4")]
    pub tag: bool,
    #[prost(bool, tag="5")]
    pub notetype: bool,
    #[prost(bool, tag="6")]
    pub config: bool,
    #[prost(bool, tag="11")]
    pub deck_config: bool,
    #[prost(bool, tag="12")]
    pub mtime: bool,
    #[prost(bool, tag="7")]
    pub browser_table: bool,
    #[prost(bool, tag="8")]
    pub browser_sidebar: bool,
    /// editor and displayed card in review screen
    #[prost(bool, tag="9")]
    pub note_text: bool,
    /// whether to call .reset() and getCard()
    #[prost(bool, tag="10")]
    pub study_queues: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpChangesWithCount {
    #[prost(uint32, tag="1")]
    pub count: u32,
    #[prost(message, optional, tag="2")]
    pub changes: ::core::option::Option<OpChanges>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpChangesWithId {
    #[prost(int64, tag="1")]
    pub id: i64,
    #[prost(message, optional, tag="2")]
    pub changes: ::core::option::Option<OpChanges>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndoStatus {
    #[prost(string, tag="1")]
    pub undo: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub redo: ::prost::alloc::string::String,
    #[prost(uint32, tag="3")]
    pub last_step: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpChangesAfterUndo {
    #[prost(message, optional, tag="1")]
    pub changes: ::core::option::Option<OpChanges>,
    #[prost(string, tag="2")]
    pub operation: ::prost::alloc::string::String,
    #[prost(int64, tag="3")]
    pub reverted_to_timestamp: i64,
    #[prost(message, optional, tag="4")]
    pub new_status: ::core::option::Option<UndoStatus>,
    #[prost(uint32, tag="5")]
    pub counter: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Progress {
    #[prost(oneof="progress::Value", tags="1, 2, 3, 4, 5, 6")]
    pub value: ::core::option::Option<progress::Value>,
}
/// Nested message and enum types in `Progress`.
pub mod progress {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MediaSync {
        #[prost(string, tag="1")]
        pub checked: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub added: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub removed: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FullSync {
        #[prost(uint32, tag="1")]
        pub transferred: u32,
        #[prost(uint32, tag="2")]
        pub total: u32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NormalSync {
        #[prost(string, tag="1")]
        pub stage: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub added: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub removed: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DatabaseCheck {
        #[prost(string, tag="1")]
        pub stage: ::prost::alloc::string::String,
        #[prost(uint32, tag="2")]
        pub stage_total: u32,
        #[prost(uint32, tag="3")]
        pub stage_current: u32,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(message, tag="1")]
        None(super::super::generic::Empty),
        #[prost(message, tag="2")]
        MediaSync(MediaSync),
        #[prost(string, tag="3")]
        MediaCheck(::prost::alloc::string::String),
        #[prost(message, tag="4")]
        FullSync(FullSync),
        #[prost(message, tag="5")]
        NormalSync(NormalSync),
        #[prost(message, tag="6")]
        DatabaseCheck(DatabaseCheck),
    }
}
