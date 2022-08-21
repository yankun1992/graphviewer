use std::fs::{create_dir, File};
use std::path::Path;

use polars::datatypes::DataType;
use polars::df;
use polars::io::parquet::ParquetCompression;
use polars::prelude::*;
use polars::prelude::{DataFrame, LazyFrame, ParquetWriter, Schema};
use serde::{Deserialize, Serialize};

use crate::common::ChaosResult;
use crate::table::csv::{read_csv, read_csv_schema, read_csv_with_schema, CsvOptions};
use crate::table::parquet::{
    read_parquet, read_parquet_schema, read_parquet_with_rename, write_lazy_parquet, write_parquet,
};
use crate::table::{datatype_to_string, str_to_datatype, Rename};

use super::{FileType, TableType};

#[derive(Debug, Deserialize, Serialize)]
pub struct AdvanceSetting {
    csv: Option<CsvOptions>,
    renames: Vec<Rename>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TableSetting {
    file_path: String,
    file_type: FileType,
    advance_setting: AdvanceSetting,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectParam {
    app_name: String,
    project_home: String,
    edge_setting: TableSetting,
    vertex_setting: Option<TableSetting>,
}

#[tauri::command]
pub fn new_project(app: tauri::AppHandle, project_param: ProjectParam) -> ChaosResult<()> {
    let ProjectParam {
        app_name,
        project_home,
        edge_setting,
        vertex_setting,
    } = project_param;
    let project_path = Path::new(&project_home).join(app_name);

    // create project dir
    create_dir(project_path.clone()).unwrap();

    // covert edges table
    covert_table(
        project_path.as_path(),
        edge_setting,
        TableType::EDGES.into(),
    );

    // cover vertices table
    match vertex_setting {
        Some(setting) => {
            covert_table(project_path.as_path(), setting, TableType::VERTICES.into());
            Ok(())
        }
        None => {
            let edges_df = read_table(project_path.as_path(), TableType::EDGES);
            let mut src = edges_df.column("src").unwrap().clone();
            src.rename("id");

            let dst = edges_df.column("dst").unwrap();

            src.append(dst).unwrap();

            let mut id = DataFrame::new(vec![src])
                .unwrap()
                .unique_stable(None, UniqueKeepStrategy::First)
                .unwrap();

            write_table(&project_path, TableType::VERTICES.into(), &mut id);
            Ok(())
        }
    }
}

fn covert_table(project_path: &Path, table_setting: TableSetting, table_type: String) {
    let table_path = project_path.join(&table_type);
    let TableSetting {
        file_path,
        file_type,
        advance_setting,
    } = table_setting;
    let AdvanceSetting { csv, renames } = advance_setting;

    let mut schema: Schema = match file_type {
        FileType::JSON => todo!(),
        FileType::CSV => {
            let schema = match csv {
                Some(ref options) => read_csv_schema(&file_path, &options),
                None => {
                    let options = CsvOptions::default();
                    read_csv_schema(&file_path, &options)
                }
            };
            schema
        }
        FileType::PARQUET => read_parquet_schema(&file_path),
        FileType::Unknown => panic!(""),
    };

    for rename in &renames {
        if rename.re_type != datatype_to_string(schema.get(&rename.name).unwrap()) {
            schema.with_column(rename.name.clone(), str_to_datatype(&rename.re_type));
        };
        if let Some(ref name) = rename.rename {
            if name != "" {
                schema.rename(&rename.name, name.clone());
            };
        };
    }

    let mut df = match file_type {
        FileType::JSON => {
            todo!()
        }
        FileType::CSV => {
            let df = read_csv_with_schema(&file_path, &csv.unwrap(), &schema);
            df
        }
        FileType::PARQUET => {
            let df = read_parquet_with_rename(&file_path, None, &renames);
            df
        }
        FileType::Unknown => panic!(""),
    };

    write_parquet(table_path.to_str().unwrap(), &mut df);
}

fn write_table_lazy(project_path: &Path, table_type: &str, table: LazyFrame) {
    let table_path = project_path.join(table_type);
    write_lazy_parquet(table_path.to_str().unwrap(), table)
}

fn write_table(project_path: &Path, table_type: String, table: &mut DataFrame) {
    let table_path = project_path.join(table_type);
    write_parquet(table_path.to_str().unwrap(), table);
}

fn read_table(project_path: &Path, table_type: TableType) -> DataFrame {
    let table_name: String = table_type.into();
    let table_path = project_path.join(table_name);

    read_parquet(table_path.to_str().unwrap(), None)
}
