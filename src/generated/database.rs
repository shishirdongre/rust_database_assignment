// This file is @generated by prost-build.
/// Represents a column definition (name, type, optional length for strings, nullable flag)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColumnDefinition {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration = "ColumnType", tag = "2")]
    pub col_type: i32,
    /// Optional, used only for STRING types
    #[prost(uint32, tag = "3")]
    pub length: u32,
    /// Indicates if the column is NOT NULL
    #[prost(bool, tag = "4")]
    pub not_null: bool,
}
/// Represents a single cell value in a row
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CellValue {
    #[prost(oneof = "cell_value::Value", tags = "1, 2, 3")]
    pub value: ::core::option::Option<cell_value::Value>,
}
/// Nested message and enum types in `CellValue`.
pub mod cell_value {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(int32, tag = "1")]
        IntVal(i32),
        #[prost(string, tag = "2")]
        StrVal(::prost::alloc::string::String),
        /// Add more types as needed (e.g., float, double)
        #[prost(bool, tag = "3")]
        NullVal(bool),
    }
}
/// Represents a row, with cells in a defined order based on ColumnDefinition
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Row {
    #[prost(message, repeated, tag = "1")]
    pub cells: ::prost::alloc::vec::Vec<CellValue>,
}
/// Represents a table, containing metadata and data rows
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableDefinition {
    /// Table name
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Column definitions
    #[prost(message, repeated, tag = "2")]
    pub columns: ::prost::alloc::vec::Vec<ColumnDefinition>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Database {
    /// Table definitions
    #[prost(message, repeated, tag = "1")]
    pub tables: ::prost::alloc::vec::Vec<TableDefinition>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableData {
    /// Table name
    #[prost(string, tag = "1")]
    pub table_name: ::prost::alloc::string::String,
    /// Number of rows in the table
    #[prost(uint32, tag = "2")]
    pub num_rows: u32,
    /// Rows in the table
    #[prost(message, repeated, tag = "3")]
    pub rows: ::prost::alloc::vec::Vec<Row>,
}
/// Column data types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ColumnType {
    Int = 0,
    String = 1,
}
impl ColumnType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Int => "INT",
            Self::String => "STRING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INT" => Some(Self::Int),
            "STRING" => Some(Self::String),
            _ => None,
        }
    }
}
