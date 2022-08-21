use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_json::json;

pub mod command;

#[derive(Debug, Deserialize, Serialize)]
pub enum TableType {
    VERTICES,
    EDGES,
    Unknown,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum FileType {
    JSON,
    CSV,
    PARQUET,
    Unknown,
}

impl Display for TableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableType::VERTICES => write!(f, "vertices"),
            TableType::EDGES => write!(f, "edges"),
            TableType::Unknown => write!(f, "Unknown"),
        }
    }
}

impl From<TableType> for String {
    fn from(table_type: TableType) -> Self {
        match table_type {
            TableType::VERTICES => "vertices".into(),
            TableType::EDGES => "edges".into(),
            TableType::Unknown => "Unknown".into(),
        }
    }
}

impl From<String> for TableType {
    fn from(string: String) -> Self {
        if string.trim().to_lowercase() == "vertices" {
            TableType::VERTICES
        } else if string.trim().to_lowercase() == "edges" {
            TableType::EDGES
        } else {
            TableType::Unknown
        }
    }
}

impl From<&str> for TableType {
    fn from(string: &str) -> Self {
        if string.trim().to_lowercase() == "vertices" {
            TableType::VERTICES
        } else if string.trim().to_lowercase() == "edges" {
            TableType::EDGES
        } else {
            TableType::Unknown
        }
    }
}

impl Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileType::JSON => write!(f, "json"),
            FileType::CSV => write!(f, "csv"),
            FileType::PARQUET => write!(f, "parquet"),
            FileType::Unknown => write!(f, "Unknown"),
        }
    }
}

impl From<FileType> for String {
    fn from(file_type: FileType) -> Self {
        match file_type {
            FileType::JSON => "json".into(),
            FileType::CSV => "csv".into(),
            FileType::PARQUET => "parquet".into(),
            FileType::Unknown => "Unknown".into(),
        }
    }
}

impl From<String> for FileType {
    fn from(string: String) -> Self {
        if string.trim().to_lowercase() == "json" {
            FileType::JSON
        } else if string.trim().to_lowercase() == "csv" {
            FileType::CSV
        } else if string.trim().to_lowercase() == "parquet" {
            FileType::PARQUET
        } else {
            FileType::Unknown
        }
    }
}

impl From<&str> for FileType {
    fn from(string: &str) -> Self {
        if string.trim().to_lowercase() == "json" {
            FileType::JSON
        } else if string.trim().to_lowercase() == "csv" {
            FileType::CSV
        } else if string.trim().to_lowercase() == "parquet" {
            FileType::PARQUET
        } else {
            FileType::Unknown
        }
    }
}

#[test]
fn file_type_test() {
    println!("{:?}", serde_json::to_string(&FileType::JSON).unwrap());
    println!("{}", json!(FileType::CSV));
    println!("{}", json!(FileType::PARQUET));

    println!("{}", json!(TableType::VERTICES));
    println!("{}", json!(TableType::EDGES));

    let file_type: Result<FileType, serde_json::Error> =
        serde_json::from_str(&"\"Json\"".to_uppercase());

    println!("{:?}", file_type);
}
