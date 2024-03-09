use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use tokio::time::Duration;

pub async fn scan_and_delete_duplicate_files(path: &str) {
    let recordings_dir = Path::new(path);

    if !recordings_dir.exists() || !recordings_dir.is_dir() {
        println!("Invalid directory path: {}", path);
        return;
    }

    match fs::read_dir(recordings_dir) {
        Ok(entries) => {
            let mp4_files = get_mp4_files(entries).await;

            let mut tasks = Vec::new();

            for entry in mp4_files {
                let recordings_dir = recordings_dir.to_path_buf();
                let task = tokio::spawn(async move {
                    let base_name = entry.file_stem().unwrap();
                    let corresponding_mkvs =
                        get_corresponding_mkvs(&recordings_dir, base_name).await;

                    for mkv_path in corresponding_mkvs {
                        println!("Found duplicate files: {:?} and {:?}", entry, mkv_path);
                        delete_file(mkv_path);
                    }
                });

                tasks.push(task);
            }

            tokio::time::sleep(Duration::from_secs(1)).await;

            for task in tasks {
                task.await.expect("Error in task");
            }
        }
        Err(e) => {
            println!("Error reading directory: {:?}", e);
        }
    }
}

async fn get_mp4_files(entries: fs::ReadDir) -> Vec<PathBuf> {
    let mut mp4_files = Vec::new();

    for entry in entries.filter_map(Result::ok) {
        let file_path = entry.path();
        if let Some(extension) = file_path.extension() {
            if extension == "mp4" {
                mp4_files.push(file_path.clone());
            }
        }
    }

    mp4_files
}

async fn get_corresponding_mkvs(base_dir: &Path, base_name: &OsStr) -> Vec<PathBuf> {
    let mut mkv_files = Vec::new();

    if let Ok(entries) = fs::read_dir(base_dir) {
        for entry in entries.filter_map(Result::ok) {
            let file_path = entry.path();
            if let Some(extension) = file_path.extension() {
                if extension == "mkv" && file_path.file_stem() == Some(base_name) {
                    mkv_files.push(file_path.clone());
                }
            }
        }
    }

    mkv_files
}

fn delete_file(file_path: PathBuf) {
    println!("Deleting file: {:?}", file_path);
    if let Err(e) = fs::remove_file(&file_path) {
        println!("Failed to delete file: {:?}", e);
    }
}
