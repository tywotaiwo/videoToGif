use std::fs;
use std::path::Path;
use regex::Regex;
use walkdir::WalkDir;

/// Sanitizes video filenames in the specified folder by removing spaces and special characters.
pub fn sanitize_video_filenames(folder: &str) {
    let re = Regex::new(r"[^a-zA-Z]").unwrap();

    for entry in WalkDir::new(folder).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "mp4") {
            if let Some(filename) = path.file_stem().and_then(|f| f.to_str()) {
                let sanitized_filename = re.replace_all(filename, "");
                let new_path = path.with_file_name(format!("{}.mp4", sanitized_filename));

                if let Err(e) = fs::rename(path, &new_path) {
                    eprintln!("Failed to rename {}: {}", path.display(), e);
                } else {
                    println!("Renamed: {} -> {}", path.display(), new_path.display());
                }
            }
        }
    }
} 