// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{path::Path};

use lopdf::{Document};
use serde_json;
use serde::{Serialize};

mod copy_pages;

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

#[derive(Serialize)]
struct GenerationResult {
    error: bool,
    error_code: &'static str,
}

#[tauri::command]
fn generate_sample_pdf(file_dir: &str, file_name: &str, out_dir: String, range_begin: u32, range_end: u32) -> String {
    let file_path = Path::new(file_dir).join(file_name);
    let result = Document::load(file_path);
    if result.is_err() {
        println!("load error: {}", result.unwrap_err());
        let result = GenerationResult {
            error: true,
            error_code: "file-not-found",
        };
        return serde_json::to_string(&result).unwrap();
    }

    let mut doc = result.unwrap();

    doc = copy_pages::main(doc).unwrap();

    let out_file_path = Path::new(&out_dir).join(file_name);

    let save_result = doc.save(out_file_path);

    if save_result.is_err() {
        println!("save error: {}", save_result.unwrap_err());
        let result = GenerationResult {
            error: true,
            error_code: "save-failed",
        };
        return serde_json::to_string(&result).unwrap();
    }

    let result = GenerationResult {
        error: false,
        error_code: "",
    };
    serde_json::to_string(&result).unwrap()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_sample_pdf, load_pdf_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
