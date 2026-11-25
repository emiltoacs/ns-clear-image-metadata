use std::env;

fn main() {
    let files = env::var(
        "NAUTILUS_SCRIPT_SELECTED_FILE_PATHS"
    ).unwrap_or_default();

    for file in files.lines() {
        println!("Fichier : {}", file);
    }
}
