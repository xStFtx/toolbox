use walkdir::WalkDir;
use std::fs;
use std::path::{Path, PathBuf};
use colored::*;
use crate::config::{ToolboxConfig, OrganizeRule};
use glob::Pattern;
use chrono::{Datelike, Local};

pub fn run(dir: &str, mode: &str) {
    let config = ToolboxConfig::load();
    let custom_rules = config.organize.and_then(|o| o.custom_rules).unwrap_or_default();
    let dir_path = Path::new(dir);
    if !dir_path.is_dir() {
        println!("{}", "Target is not a directory".red());
        return;
    }
    let mut applied = false;
    if !custom_rules.is_empty() {
        for entry in WalkDir::new(dir_path).min_depth(1).max_depth(1) {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    let fname = path.file_name().unwrap().to_string_lossy();
                    for rule in &custom_rules {
                        if Pattern::new(&rule.pattern).map(|pat| pat.matches(&fname)).unwrap_or(false) {
                            let target_dir = dir_path.join(&rule.target_dir);
                            if !target_dir.exists() { let _ = fs::create_dir_all(&target_dir); }
                            let target = target_dir.join(&*fname);
                            let _ = fs::rename(path, &target);
                            println!("{} {} → {}", "[RULE]".blue(), fname, target.display());
                            applied = true;
                            break;
                        }
                    }
                }
            }
        }
    }
    if !applied {
        match mode {
            "type" => organize_by_type(dir_path),
            "date" => organize_by_date(dir_path),
            _ => organize_by_type(dir_path),
        }
    }
}

fn organize_by_type(dir: &Path) {
    for entry in WalkDir::new(dir).min_depth(1).max_depth(1) {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() {
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("other");
                let target_dir = dir.join(ext);
                if !target_dir.exists() { let _ = fs::create_dir_all(&target_dir); }
                let fname = path.file_name().unwrap();
                let target = target_dir.join(&*fname);
                let _ = fs::rename(path, &target);
                println!("{} {} → {}", "[TYPE]".green(), fname.to_string_lossy(), target.display());
            }
        }
    }
}

fn organize_by_date(dir: &Path) {
    for entry in WalkDir::new(dir).min_depth(1).max_depth(1) {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_file() {
                if let Ok(meta) = fs::metadata(path) {
                    if let Ok(modified) = meta.modified() {
                        let datetime: chrono::DateTime<Local> = modified.into();
                        let y = datetime.year();
                        let m = datetime.month();
                        let date = format!("{}_{}", y, m);
                        let target_dir = dir.join(date);
                        if !target_dir.exists() { let _ = fs::create_dir_all(&target_dir); }
                        let fname = path.file_name().unwrap();
                        let target = target_dir.join(&*fname);
                        let _ = fs::rename(path, &target);
                        println!("{} {} → {}", "[DATE]".yellow(), fname.to_string_lossy(), target.display());
                    }
                }
            }
        }
    }
}
