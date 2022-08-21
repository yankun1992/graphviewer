use polars::datatypes::DataType;
use serde::{Deserialize, Serialize};

pub mod command;
pub mod csv;
pub mod json;
pub mod parquet;

#[derive(Debug, Deserialize, Serialize)]
pub struct Rename {
    pub name: String,
    pub re_type: String,
    pub rename: Option<String>,
}

pub fn datatype_to_string(datatype: &DataType) -> String {
    match datatype {
        DataType::Boolean => "Boolean".to_string(),
        DataType::UInt8 => "UInt8".to_string(),
        DataType::UInt16 => "UInt16".to_string(),
        DataType::UInt32 => "UInt32".to_string(),
        DataType::UInt64 => "UInt64".to_string(),
        DataType::Int8 => "Int8".to_string(),
        DataType::Int16 => "Int16".to_string(),
        DataType::Int32 => "Int32".to_string(),
        DataType::Int64 => "Int64".to_string(),
        DataType::Float32 => "Float32".to_string(),
        DataType::Float64 => "Float64".to_string(),
        DataType::Utf8 => "Utf8".to_string(),
        DataType::Date => "Date".to_string(),

        DataType::Time => "Time".to_string(),

        DataType::Null => "Null".to_string(),
        DataType::Unknown => "Unknown".to_string(),
        _ => "Unknown".to_string(),
    }
}

pub fn str_to_datatype(string: &str) -> DataType {
    match string {
        "Boolean" => DataType::Boolean,
        "UInt8" => DataType::UInt8,
        "UInt16" => DataType::UInt16,
        "UInt32" => DataType::UInt32,
        "UInt64" => DataType::UInt64,
        "Int8" => DataType::Int8,
        "Int16" => DataType::Int16,
        "Int32" => DataType::Int32,
        "Int64" => DataType::Int64,
        "Float32" => DataType::Float32,
        "Float64" => DataType::Float64,
        "Utf8" => DataType::Utf8,
        "Date" => DataType::Date,

        "Time" => DataType::Time,

        "Null" => DataType::Null,
        "Unknown" => DataType::Unknown,
        _ => DataType::Unknown,
    }
}
