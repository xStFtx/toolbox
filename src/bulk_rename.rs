use std::fs;
use std::path::Path;
use regex::Regex;

pub fn run(dir: &str, pattern: &str, replace: &str) {
    let re = match Regex::new(pattern) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Invalid regex pattern: {}", e);
            return;
        }
    };
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
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if re.is_match(name) {
                    let new_name = re.replace_all(name, replace);
                    let new_path = Path::new(dir).join(new_name.as_ref());
                    if let Err(e) = fs::rename(&path, &new_path) {
                        eprintln!("Failed to rename {:?} to {:?}: {}", path, new_path, e);
                    } else {
                        println!("Renamed {:?} -> {:?}", path, new_path);
                    }
                }
            }
        }
    }
}
