use walkdir::WalkDir;
use std::fs;

pub fn run(dir: &str, top: usize) {
    let mut sizes = Vec::new();
    for entry in WalkDir::new(dir).into_iter().flatten() {
        let path = entry.path();
        if path.is_file() {
            if let Ok(metadata) = fs::metadata(path) {
                sizes.push((metadata.len(), path.display().to_string()));
            }
        }
    }
    sizes.sort_by(|a, b| b.0.cmp(&a.0));
    println!("Top {} largest files:", top);
    for (size, path) in sizes.iter().take(top) {
        println!("{} bytes\t{}", size, path);
    }
}
