use std::fs;
use std::path::Path;
use tokio::fs as tokio_fs;

pub async fn delete_files_with_extension(directory_path: &str, extension: &str) {
    let dir = Path::new(directory_path);

    if !dir.exists() || !dir.is_dir() {
        println!("Invalid directory path: {}", directory_path);
        return;
    }

    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries.filter_map(Result::ok) {
                let file_path = entry.path();
                if let Some(file_extension) = file_path.extension() {
                    if file_extension == extension {
                        println!("Deleting file: {:?}", file_path);
                        if let Err(e) = tokio_fs::remove_file(&file_path).await {
                            println!("Failed to delete file: {:?}", e);
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
