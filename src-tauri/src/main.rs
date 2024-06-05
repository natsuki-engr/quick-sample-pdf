// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{path::Path};

use lopdf::{Document, Object};
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

#[derive(Serialize)]
struct GenerationResult {
    error: bool,
}

#[tauri::command]
fn generate_sample_pdf(file_dir: &str, file_name: &str, out_dir: String) -> String {
    let file_path = Path::new(file_dir).join(file_name);
    let file_path_str = &file_path.to_str().unwrap();
    let result = Document::load(&file_path);
    if result.is_err() {
        println!("load error: {}", result.unwrap_err());
        let result = GenerationResult {
            error: true,
        };
        return serde_json::to_string(&result).unwrap();
    }

    let mut doc = result.unwrap();
    let pages = doc.get_pages();
    let mut new_page_refs = vec![];

    let mut new_doc = Document::with_version("1.5");

    let page_ids = pages.values().take(3).cloned().collect::<Vec<_>>();
    for page_id in &page_ids {
        let page = doc.get_object(*page_id).unwrap();
        let new_page_id = new_doc.new_object_id();
        new_doc.objects.insert(new_page_id, page.clone());
        new_page_refs.push(Object::Reference(new_page_id));
    }

    let mut pages_dict = lopdf::Dictionary::new();
    pages_dict.set("Type", "Pages");
    pages_dict.set("Kids", new_page_refs);
    pages_dict.set("Count", page_ids.len() as i64);

    let pages_id = doc.new_object_id();
    new_doc.objects.insert(pages_id, Object::Dictionary(pages_dict));

    let mut catalog_dict = lopdf::Dictionary::new();
    catalog_dict.set("Type", "Catalog");
    catalog_dict.set("Pages", Object::Reference(pages_id));

    let catalog_id = doc.new_object_id();
    new_doc.trailer.set("Root", catalog_id);
    new_doc.objects.insert(catalog_id, Object::Dictionary(catalog_dict));

    let save_result = new_doc.save(out_dir);
    if save_result.is_err() {
        println!("save error: {}", save_result.unwrap_err());
        let result = GenerationResult {
            error: true,
        };
        return serde_json::to_string(&result).unwrap();
    }

    let result = GenerationResult {
        error: false,
    };
    serde_json::to_string(&result).unwrap()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_sample_pdf, load_pdf_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
