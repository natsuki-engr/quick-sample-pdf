// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lopdf::Document;
use serde_json;
use serde::{Serialize};

#[derive(Serialize)]
struct File {
    exists: bool,
    path: String,
    page_num: usize,
}

#[tauri::command]
fn load_pdf_files(path_list: Vec<String>) -> String {
    let mut file_results: Vec<File> = vec![];

    for file_path in path_list.iter() {
        let result = Document::load(file_path);
        let file: File = match result {
            Ok(doc) => File {
                exists: true,
                path: file_path.to_string(),
                page_num: doc.get_pages().len(),
            },
            Err(_e) => File {
                exists: false,
                path: file_path.to_string(),
                page_num: 0,
            },
        };
        file_results.push(file);
    }

    serde_json::to_string(&file_results).unwrap()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_pdf_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
