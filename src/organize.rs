use std::fs;
use std::path::Path;
use chrono::{Datelike, Local};

pub fn run(dir: &str, mode: &str) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Error reading directory {}: {}", dir, e);
            return;
        }
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            match mode {
                "type" => organize_by_type(&path, dir),
                "date" => organize_by_date(&path, dir),
                _ => println!("Unknown mode: {}", mode),
            }
        }
    }
}

fn organize_by_type(path: &Path, base: &str) {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        let target_dir = Path::new(base).join(ext);
        let _ = fs::create_dir_all(&target_dir);
        let file_name = path.file_name().unwrap();
        let target = target_dir.join(file_name);
        if let Err(e) = fs::rename(&path, &target) {
            eprintln!("Failed to move {:?} to {:?}: {}", path, target, e);
        }
    }
}

fn organize_by_date(path: &Path, base: &str) {
    if let Ok(metadata) = fs::metadata(path) {
        if let Ok(modified) = metadata.modified() {
            let datetime: chrono::DateTime<Local> = modified.into();
            let y = datetime.year();
            let m = datetime.month();
            let target_dir = Path::new(base).join(format!("{}_{}", y, m));
            let _ = fs::create_dir_all(&target_dir);
            let file_name = path.file_name().unwrap();
            let target = target_dir.join(file_name);
            if let Err(e) = fs::rename(&path, &target) {
                eprintln!("Failed to move {:?} to {:?}: {}", path, target, e);
            }
        }
    }
}
