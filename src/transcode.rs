// ffmpeg -i input.mp4 -c:v copy -c:a aac -strict experimental -b:a 192k output.mp4  // transode command we will call if the codex is flac, also add (Trans) to the file name
// ffprobe -v error -show_entries stream=codec_name,width,height,bit_rate,duration,avg_frame_rate -select_streams v:0 -show_entries stream=codec_name,channels,channel_layout,bit_rate,duration -select_streams a:0 -of default=noprint_wrappers=1 input_file.mp4 // probe command we will call to get mp4 data
use std::fs;
use std::path::{Path, PathBuf};
use tokio::process::Command;
use tokio::time::Duration;

pub async fn probe_and_transcode_flac_audio(path: &str) {
    let recordings_dir = Path::new(path);

    if !recordings_dir.exists() || !recordings_dir.is_dir() {
        println!("Invalid directory path: {}", path);
        return;
    }

    let transcoded_folder = recordings_dir.join("transcoded");

    if !transcoded_folder.exists() || !transcoded_folder.is_dir() {
        println!("'transcoded' folder does not exist. Massive Config issue.");
        return;
    }

    match fs::read_dir(recordings_dir) {
        Ok(entries) => {
            let mp4_files = get_mp4_files(entries).await;

            let mut tasks = Vec::new();

            for file_path in mp4_files {
                let transcoded_file_name = format!(
                    "{}-REMUX.mp4",
                    file_path.file_stem().unwrap().to_string_lossy()
                );

                if !transcoded_folder.join(&transcoded_file_name).exists() {
                    let task = tokio::spawn(async move {
                        let codec_info = probe_mp4_file(&file_path).await;

                        if let Some(codec_name) = codec_info {
                            if codec_name == "flac" {
                                transcode_to_opus(&file_path).await;
                            }
                        }
                    });

                    tasks.push(task);
                } else {
                    println!("video already transcoded, skipping: {:?}", file_path)
                }
            }

            tokio::time::sleep(Duration::from_secs(1)).await; // Ensure all tasks are spawned before exiting

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

async fn probe_mp4_file(file_path: &Path) -> Option<String> {
    let output = Command::new("ffprobe")
        .arg("-v")
        .arg("error")
        .arg("-select_streams")
        .arg("a:0")
        .arg("-show_entries")
        .arg("stream=codec_name")
        .arg("-of")
        .arg("default=noprint_wrappers=1:nokey=1")
        .arg(file_path)
        .output()
        .await
        .expect("Failed to execute ffprobe command.");

    let codec_name = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_lowercase();

    if codec_name.is_empty() {
        None
    } else {
        Some(codec_name.to_string())
    }
}

async fn transcode_to_opus(file_path: &Path) {
    let file_stem = file_path.file_stem().unwrap().to_string_lossy();
    let output_path = file_path.with_file_name(format!("{}-REMUX.mp4", file_stem));

    println!("Transcoding file: {:?}", file_path);

    Command::new("ffmpeg")
        .arg("-i")
        .arg(file_path)
        .arg("-c:v")
        .arg("copy")
        .arg("-c:a")
        .arg("libopus")
        .arg("-b:a")
        .arg("192k")
        .arg("-strict")
        .arg("experimental")
        .arg("-map")
        .arg("0")
        .arg(&output_path)
        .output()
        .await
        .expect("Failed to execute ffmpeg command.");

    println!("Transcoding complete. Output file: {:?}", output_path);
}
