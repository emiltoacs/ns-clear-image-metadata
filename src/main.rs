use std::env;
use std::path::Path;
use std::process::Command;

use filetime::{set_file_mtime, FileTime};

/// Check whether exiftool can be executed.
fn exiftool_program_exists() -> bool {
    let available = Command::new("exiftool")
        .arg("-ver")
        .output()
        .is_ok_and(|output| output.status.success());

    if !available {
        let _ = Command::new("zenity")
            .args([
                "--error",
                "--title=Missing dependency",
                "--text=ExifTool is not installed or cannot be executed.\n\nPlease install it before running this script:\n\nsudo apt install libimage-exiftool-perl",
                "--width=400",
            ])
            .status();
    }

    available
}

/// Remove all the metadada of the file if its an image
fn clean_image(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Check that the file exists and retrieve its metadata.
    let metadata = std::fs::metadata(path)?;

    if !metadata.is_file() {
        return Ok(());
    }

    // Detect image format from its actual file signature.
    let kind = infer::get_from_path(path)?;

    let Some(kind) = kind else {
        return Ok(());
    };

    if kind.matcher_type() != infer::MatcherType::Image {
        return Ok(());
    }

    // Save the original modification time.
    let original_mtime = FileTime::from_last_modification_time(&metadata);

    println!("Cleaning: {}", path.display());

    // Use ExifTool to remove all metadata.
    let result = Command::new("exiftool")
        .arg("-all=")
        .arg("-overwrite_original")
        .arg(path)
        .output();

    // Restore the original modification time regardless
    // of whether ExifTool succeeded or failed.
    let restore_result = set_file_mtime(path, original_mtime);

    match result {
        Ok(output) => {
            if output.status.success() {
                println!("  Metadata removed.");
            } else {
                eprintln!(
                    "  ExifTool failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
        Err(error) => {
            eprintln!("  Could not execute ExifTool: {}", error);
        }
    }

    if let Err(error) = restore_result {
        eprintln!("  Could not restore mtime: {}", error);
    } else {
        println!("  Original mtime restored.");
    }

    Ok(())
}

fn main() {

    if !exiftool_program_exists() {
        return;
    }

    let selected_files =
        match env::var("NAUTILUS_SCRIPT_SELECTED_FILE_PATHS") {
            Ok(files) => files,
            Err(_) => {
                eprintln!("No file selected in nautilus -> script does nothing");
                return;
            }
        };

    let mut processed = 0;
    let mut skipped = 0;

    for file in selected_files.lines() {
        if file.is_empty() {
            continue;
        }

        let path = Path::new(file);

        match clean_image(path) {
            Ok(()) => processed += 1,
            Err(error) => {
                eprintln!("Error cannot process {}: {}", file, error);
                skipped += 1;
            }
        }
    }

    println!();
    println!("Finished.");
    println!("Processed: {}", processed);
    println!("Errors:    {}", skipped);
}
