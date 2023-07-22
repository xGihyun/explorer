use rayon::prelude::*;
use std::fs;
use std::io;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileData {
    pub name: String,
    pub path: String,
    pub size: f64,
}

pub fn get_file_data(dir_path: &str) -> io::Result<Vec<FileData>> {
    let mut files = Vec::new();

    traverse_directory(Path::new(dir_path), &mut files)?;

    Ok(files)
}

pub fn traverse_directory(path: &Path, files: &mut Vec<FileData>) -> io::Result<()> {
    if path.is_file() {
        if let Some(name) = path.file_name() {
            let size_in_kb = get_file_size(path)?;
            let file_name = name.to_string_lossy().to_string();
            let file_path = path.to_string_lossy().to_string();

            let file_data = FileData {
                name: file_name,
                path: file_path,
                size: size_in_kb,
            };

            files.push(file_data);
        }
    } else if path.is_dir() {
        let entries = fs::read_dir(path)?;

        let file_data_entries: Vec<io::Result<Vec<FileData>>> = entries
            .into_iter()
            .par_bridge()
            .map(|entry_result| {
                entry_result.and_then(|entry| {
                    let entry_path = entry.path();
                    let mut sub_files = Vec::new();
                    traverse_directory(&entry_path, &mut sub_files)?;
                    Ok(sub_files)
                })
            })
            .collect();

        for file_data_result in file_data_entries {
            let file_data = file_data_result?;
            files.par_extend(file_data);
        }
    } else {
        eprintln!("Unkown file type");
    }

    Ok(())
}

pub fn get_file_size(path: &Path) -> io::Result<f64> {
    let metadata = fs::metadata(path)?;
    let size_in_bytes = metadata.len();

    Ok(size_in_bytes as f64 / 1024.0)
}
