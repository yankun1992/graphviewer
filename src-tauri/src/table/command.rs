use polars::prelude::{DataFrame, Schema};

use super::csv::{read_csv, read_csv_schema, CsvOptions};
use super::parquet::{read_parquet, read_parquet_schema};

#[tauri::command]
pub fn get_csv_schema(app: tauri::AppHandle, file: &str, options: CsvOptions) -> Schema {
    read_csv_schema(file, &options)
}

#[tauri::command]
pub fn read_csv_command(app: tauri::AppHandle, file: &str, options: CsvOptions) -> DataFrame {
    read_csv(file, &options)
}

#[tauri::command]
pub fn get_parquet_schema(app: tauri::AppHandle, file: &str) -> Schema {
    read_parquet_schema(file)
}

#[tauri::command]
pub fn read_parquet_command(app: tauri::AppHandle, file: &str, n_row: Option<usize>) -> DataFrame {
    read_parquet(file, n_row)
}
