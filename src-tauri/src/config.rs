use std::{
    fs::{create_dir, File, OpenOptions},
    path::Path,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    /// projects home dir, can be set by user!
    projects_home: String,
    /// recent opened projects history file.
    history_file: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            projects_home: String::from(""),
            history_file: String::from(""),
        }
    }
}

#[tauri::command]
pub fn is_first_running(app: tauri::AppHandle) -> bool {
    let app_dir = app.path_resolver().app_dir().unwrap();
    let config_file = app_dir.join(&Path::new(".config.json"));
    let projects_dir = app_dir.join(&Path::new("projects"));
    let history_file = app_dir.join(&Path::new(".history"));

    if !app_dir.clone().exists() {
        // create app dir
        create_dir(&app_dir).unwrap();
        // craete [projects]
        create_dir(projects_dir.clone()).unwrap();

        // create [.history]
        File::create(history_file.clone()).unwrap();
        // create [.config.json]
        let mut app_config = AppConfig::default();
        app_config.history_file = history_file.to_str().unwrap().to_string();
        app_config.projects_home = projects_dir.to_str().unwrap().to_string();
        write_app_config(app, app_config);
        // serde_json::to_writer_pretty(File::create(config_file.as_path()).unwrap(), &app_config)
        // .unwrap();
        return true;
    } else {
        return false;
    }
}

#[tauri::command]
pub fn get_default_projects_home(app: tauri::AppHandle) -> String {
    let app_dir = app.path_resolver().app_dir();
    let path = app_dir.unwrap().join(&Path::new("projects"));
    path.to_str().unwrap().to_string()
}

#[tauri::command]
pub fn read_app_config(app: tauri::AppHandle) -> AppConfig {
    let app_dir = app.path_resolver().app_dir().unwrap();
    let config_file = app_dir.join(&Path::new(".config.json"));

    let app_config: AppConfig = serde_json::from_reader(File::open(config_file).unwrap()).unwrap();

    app_config
}

#[tauri::command]
pub fn write_app_config(app: tauri::AppHandle, config: AppConfig) {
    let app_dir = app.path_resolver().app_dir().unwrap();
    let config_file = app_dir.join(&Path::new(".config.json"));

    serde_json::to_writer_pretty(File::create(config_file.as_path()).unwrap(), &config).unwrap();
}

#[tauri::command]
pub fn set_projects_home(app: tauri::AppHandle, config_path: Option<String>) {
    let app_dir = app.path_resolver().app_dir();
    let path = app_dir.unwrap().join(&Path::new(".config.json"));
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .unwrap();

    let config_path = match config_path {
        Some(config_path) => config_path,
        None => app
            .path_resolver()
            .app_dir()
            .unwrap()
            .join(&Path::new("projects"))
            .as_path()
            .to_str()
            .unwrap()
            .to_string(),
    };

    let mut config = AppConfig::default();
    config.projects_home = config_path;

    serde_json::to_writer(file, &config).unwrap();
}

#[tauri::command]
pub fn path_exists(app: tauri::AppHandle, path: String) -> bool {
    let path = Path::new(&path);
    path.exists()
}
