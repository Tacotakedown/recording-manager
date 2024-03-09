use crate::delete_duplicates::scan_and_delete_duplicate_files;
use crate::delete_with_extension::delete_files_with_extension;
use crate::transcode::probe_and_transcode_flac_audio;

use std::io;

mod delete_duplicates;
mod delete_with_extension;
mod transcode;

const RECORDINGS_PATH: &str = "E:\\recordings";

#[tokio::main]
async fn main() {
    println!("Select function to run:");
    println!("1. Scan and delete duplicate files");
    println!("2. Probe and transcode FLAC audio");
    println!("3. Delete all shitty .llc files");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let choice: u32 = input.trim().parse().expect("Invalid input");

    match choice {
        1 => {
            scan_and_delete_duplicate_files(RECORDINGS_PATH).await;
        }
        2 => {
            probe_and_transcode_flac_audio(RECORDINGS_PATH).await;
        }
        3 => {
            delete_files_with_extension(RECORDINGS_PATH, "llc").await;
        }
        _ => {
            println!("Invalid choice. Exiting.");
        }
    }
}
