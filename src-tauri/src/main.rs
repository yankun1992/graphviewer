#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[allow(dead_code)]
#[allow(unused_variables)]
mod config;
#[allow(dead_code)]
#[allow(unused_variables)]
mod history;
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]
mod project;

#[allow(dead_code)]
#[allow(unused_variables)]
mod table;

#[allow(dead_code)]
#[allow(unused_variables)]
mod common;

use config::{
    get_default_projects_home, is_first_running, path_exists, read_app_config, set_projects_home,
    write_app_config,
};

use history::{read_history, write_history};

use table::command::{get_csv_schema, get_parquet_schema};
use table::command::{read_csv_command, read_parquet_command};

use project::command::new_project;

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            is_first_running,
            set_projects_home,
            get_default_projects_home,
            read_app_config,
            write_app_config,
            read_history,
            write_history,
            path_exists,
            get_csv_schema,
            read_csv_command,
            get_parquet_schema,
            read_parquet_command,
            new_project
        ])
        .run(context)
        .expect("error while running tauri application");
}
