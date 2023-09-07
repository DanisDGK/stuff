use rayon::prelude::*;
use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

fn main() -> Result<(), Box<dyn Error>> {
    // Get the source directory and target directory
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <source_directory> <target_directory>", args[0]);
        std::process::exit(1);
    }
    let source_directory = &args[1];
    let target_directory = &args[2];

    // Ensure the target directory exists; create it if necessary
    if !Path::new(target_directory).exists() {
        fs::create_dir_all(target_directory)?;
    }

    // Traverse the source directory recursively and collect audio files
    let audio_files: Vec<DirEntry> = WalkDir::new(source_directory)
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().is_file() && is_audio_file(&entry.path()) {
                Some(entry)
            } else {
                None
            }
        })
        .collect();

    // Copy audio files in parallel using Rayon
    audio_files.par_iter().for_each(|entry| {
        copy_audio_file(&entry.path(), target_directory).unwrap();
    });

    println!("Audio files copied successfully.");
    Ok(())
}

fn is_audio_file(path: &Path) -> bool {
    // Check if the file extension is one commonly used for audio files
    if let Some(extension) = path.extension() {
        let audio_extensions = ["mp3", "wav", "flac", "ogg", "aac", "opus"];
        audio_extensions.iter().any(|&ext| ext == extension)
    } else {
        false
    }
}

fn copy_audio_file(source_path: &Path, target_directory: &str) -> Result<(), Box<dyn Error>> {
    let source_filename = source_path.file_name().ok_or("Invalid file name")?;
    let target_path = PathBuf::from(target_directory).join(source_filename);

    fs::copy(source_path, &target_path)?;

    println!("Copied: {:?}", target_path);
    Ok(())
}
