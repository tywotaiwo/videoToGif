use std::fs;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;

fn main() {
    let input_folder = "videos"; // Change this to your folder
    let output_folder = "gifs";  // Folder to save GIFs

    // Ensure output folder exists
    fs::create_dir_all(output_folder).expect("Failed to create output folder");

    // Loop through all .mp4 files in the input folder
    for entry in WalkDir::new(input_folder).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "mp4") {
            convert_to_gif(path, output_folder);
        }
    }

    println!("✅ All videos converted!");
}

// Function to convert MP4 to GIF using FFmpeg
fn convert_to_gif(input_path: &Path, output_folder: &str) {
    let input_filename = input_path.file_stem().unwrap().to_str().unwrap();
    let output_path = format!("{}/{}.gif", output_folder, input_filename);

    let ffmpeg_command = Command::new("ffmpeg")
        .args([
            "-i",
            input_path.to_str().unwrap(),
            "-vf",
            "fps=10,scale=480:-1:flags=lanczos",
            "-c:v",
            "gif",
            &output_path,
        ])
        .output()
        .expect("Failed to execute ffmpeg");

    if ffmpeg_command.status.success() {
        println!("🎥 Converted: {}", input_path.display());
    } else {
        println!("❌ Error converting: {}", input_path.display());
    }
}
