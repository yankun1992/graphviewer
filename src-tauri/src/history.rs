use std::{fs::File, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct History {
    name: String,
    path: String,
}

#[tauri::command]
pub fn read_history(app: tauri::AppHandle) -> Vec<History> {
    let app_dir = app.path_resolver().app_dir().unwrap();
    let history_file = app_dir.join(&Path::new(".history"));

    let historys: Vec<History> = match serde_json::from_reader(File::open(history_file).unwrap()) {
        Ok(value) => value,
        Err(e) => {
            println!("{:?}", e);
            vec![]
        },
    };

    println!("{:?}", historys);

    historys
}

#[tauri::command]
pub fn write_history(app: tauri::AppHandle, history: History) {
    let app_dir = app.path_resolver().app_dir().unwrap();
    let history_file = app_dir.join(&Path::new(".history"));

    let mut histories = read_history(app);

    if !histories.is_empty() {
        if histories[0] != history {
            histories.insert(0, history);
        }
    } else {
        histories.push(history);
    }

    serde_json::to_writer_pretty(File::create(history_file.as_path()).unwrap(), &histories)
        .unwrap();
}
