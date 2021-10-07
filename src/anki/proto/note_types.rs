#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotetypeId {
    #[prost(int64, tag = "1")]
    pub ntid: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Notetype {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub mtime_secs: i64,
    #[prost(sint32, tag = "4")]
    pub usn: i32,
    #[prost(message, optional, tag = "7")]
    pub config: ::core::option::Option<notetype::Config>,
    #[prost(message, repeated, tag = "8")]
    pub fields: ::prost::alloc::vec::Vec<notetype::Field>,
    #[prost(message, repeated, tag = "9")]
    pub templates: ::prost::alloc::vec::Vec<notetype::Template>,
}
/// Nested message and enum types in `Notetype`.
pub mod notetype {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Config {
        #[prost(enumeration = "config::Kind", tag = "1")]
        pub kind: i32,
        #[prost(uint32, tag = "2")]
        pub sort_field_idx: u32,
        #[prost(string, tag = "3")]
        pub css: ::prost::alloc::string::String,
        //// This is now stored separately; retrieve with DefaultsForAdding()
        #[prost(int64, tag = "4")]
        pub target_deck_id_unused: i64,
        #[prost(string, tag = "5")]
        pub latex_pre: ::prost::alloc::string::String,
        #[prost(string, tag = "6")]
        pub latex_post: ::prost::alloc::string::String,
        #[prost(bool, tag = "7")]
        pub latex_svg: bool,
        #[prost(message, repeated, tag = "8")]
        pub reqs: ::prost::alloc::vec::Vec<config::CardRequirement>,
        #[prost(bytes = "vec", tag = "255")]
        pub other: ::prost::alloc::vec::Vec<u8>,
    }
    /// Nested message and enum types in `Config`.
    pub mod config {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CardRequirement {
            #[prost(uint32, tag = "1")]
            pub card_ord: u32,
            #[prost(enumeration = "card_requirement::Kind", tag = "2")]
            pub kind: i32,
            #[prost(uint32, repeated, tag = "3")]
            pub field_ords: ::prost::alloc::vec::Vec<u32>,
        }
        /// Nested message and enum types in `CardRequirement`.
        pub mod card_requirement {
            #[derive(
                Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
            )]
            #[repr(i32)]
            pub enum Kind {
                None = 0,
                Any = 1,
                All = 2,
            }
        }
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Kind {
            Normal = 0,
            Cloze = 1,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Field {
        #[prost(message, optional, tag = "1")]
        pub ord: ::core::option::Option<super::super::generic::OptionalUInt32>,
        #[prost(string, tag = "2")]
        pub name: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "5")]
        pub config: ::core::option::Option<field::Config>,
    }
    /// Nested message and enum types in `Field`.
    pub mod field {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Config {
            #[prost(bool, tag = "1")]
            pub sticky: bool,
            #[prost(bool, tag = "2")]
            pub rtl: bool,
            #[prost(string, tag = "3")]
            pub font_name: ::prost::alloc::string::String,
            #[prost(uint32, tag = "4")]
            pub font_size: u32,
            #[prost(bytes = "vec", tag = "255")]
            pub other: ::prost::alloc::vec::Vec<u8>,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Template {
        #[prost(message, optional, tag = "1")]
        pub ord: ::core::option::Option<super::super::generic::OptionalUInt32>,
        #[prost(string, tag = "2")]
        pub name: ::prost::alloc::string::String,
        #[prost(int64, tag = "3")]
        pub mtime_secs: i64,
        #[prost(sint32, tag = "4")]
        pub usn: i32,
        #[prost(message, optional, tag = "5")]
        pub config: ::core::option::Option<template::Config>,
    }
    /// Nested message and enum types in `Template`.
    pub mod template {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Config {
            #[prost(string, tag = "1")]
            pub q_format: ::prost::alloc::string::String,
            #[prost(string, tag = "2")]
            pub a_format: ::prost::alloc::string::String,
            #[prost(string, tag = "3")]
            pub q_format_browser: ::prost::alloc::string::String,
            #[prost(string, tag = "4")]
            pub a_format_browser: ::prost::alloc::string::String,
            #[prost(int64, tag = "5")]
            pub target_deck_id: i64,
            #[prost(string, tag = "6")]
            pub browser_font_name: ::prost::alloc::string::String,
            #[prost(uint32, tag = "7")]
            pub browser_font_size: u32,
            #[prost(bytes = "vec", tag = "255")]
            pub other: ::prost::alloc::vec::Vec<u8>,
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOrUpdateNotetypeRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub json: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "2")]
    pub preserve_usn_and_mtime: bool,
    #[prost(bool, tag = "3")]
    pub skip_checks: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StockNotetype {
    #[prost(enumeration = "stock_notetype::Kind", tag = "1")]
    pub kind: i32,
}
/// Nested message and enum types in `StockNotetype`.
pub mod stock_notetype {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Kind {
        Basic = 0,
        BasicAndReversed = 1,
        BasicOptionalReversed = 2,
        BasicTyping = 3,
        Cloze = 4,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotetypeNames {
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<NotetypeNameId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotetypeUseCounts {
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<NotetypeNameIdUseCount>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotetypeNameId {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotetypeNameIdUseCount {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub use_count: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAuxConfigKeyRequest {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAuxTemplateConfigKeyRequest {
    #[prost(int64, tag = "1")]
    pub notetype_id: i64,
    #[prost(uint32, tag = "2")]
    pub card_ordinal: u32,
    #[prost(string, tag = "3")]
    pub key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChangeNotetypeInfoRequest {
    #[prost(int64, tag = "1")]
    pub old_notetype_id: i64,
    #[prost(int64, tag = "2")]
    pub new_notetype_id: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeNotetypeRequest {
    #[prost(int64, repeated, tag = "1")]
    pub note_ids: ::prost::alloc::vec::Vec<i64>,
    /// -1 is used to represent null, as nullable repeated fields
    /// are unwieldy in protobuf
    #[prost(int32, repeated, tag = "2")]
    pub new_fields: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, tag = "3")]
    pub new_templates: ::prost::alloc::vec::Vec<i32>,
    #[prost(int64, tag = "4")]
    pub old_notetype_id: i64,
    #[prost(int64, tag = "5")]
    pub new_notetype_id: i64,
    #[prost(int64, tag = "6")]
    pub current_schema: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeNotetypeInfo {
    #[prost(string, repeated, tag = "1")]
    pub old_field_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub old_template_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub new_field_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "4")]
    pub new_template_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "5")]
    pub input: ::core::option::Option<ChangeNotetypeRequest>,
}
