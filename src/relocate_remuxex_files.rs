use std::fs;
use std::path::Path;
use tokio::fs as tokio_fs;

pub async fn move_remux_files_to_transcoded(directory_path: &str) {
    let dir = Path::new(directory_path);
    let transcoded_folder = dir.join("transcoded");

    if !dir.exists() || !dir.is_dir() {
        println!("Invalid directory path: {}", directory_path);
        return;
    }

    if !transcoded_folder.exists() {
        if let Err(e) = tokio_fs::create_dir(&transcoded_folder).await {
            println!("Failed to create 'transcoded' folder: {:?}", e);
            return;
        }
    }

    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries.filter_map(Result::ok) {
                let file_path = entry.path();
                if let Some(file_name) = file_path.file_name() {
                    if file_name.to_string_lossy().ends_with("REMUX.mp4") {
                        let destination_path = transcoded_folder.join(file_name);
                        println!("Moving file: {:?} to {:?}", file_path, destination_path);
                        if let Err(e) = tokio_fs::rename(&file_path, &destination_path).await {
                            println!("Failed to move file: {:?}", e);
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Error reading directory: {:?}", e);
        }
    }
}
