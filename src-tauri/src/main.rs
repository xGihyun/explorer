// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use rayon::prelude::*;
// use std::io;
// use std::time::Instant;
use sysinfo::{DiskExt, System, SystemExt};

mod search;
use search::{get_file_data, FileData};

#[tauri::command]
fn explore(path: &str, query: &str) -> Vec<FileData> {
    // let start_time = Instant::now();

    // Unwrap for now
    let files = get_file_data(path).unwrap();

    let matcher = SkimMatcherV2::default();

    let filtered_files: Vec<FileData> = files
        .into_par_iter()
        .filter(|file| matcher.fuzzy_match(&file.name, query).is_some())
        .collect();

    // let elapsed_time = start_time.elapsed().as_secs_f64();

    filtered_files
}

#[tauri::command]
fn get_disks() -> Vec<String> {
    let mut disk_names: Vec<String> = Vec::new();

    let system = System::new_all();
    let disks = system.disks();

    for disk in disks {
        let disk_name = disk.mount_point().to_str().unwrap().to_string();

        disk_names.push(disk_name)
    }

    println!("{disk_names:?}");

    disk_names
}

#[tauri::command]
fn get_files_in_dir(path: &str) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();

    let dir = Path::new(path);
    let read_dir = dir.read_dir().unwrap();

    read_dir.for_each(|res| {
        let entry = res.unwrap();

        files.push(entry.file_name().to_str().unwrap().to_string())
    });

    files
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            explore,
            get_disks,
            get_files_in_dir
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
