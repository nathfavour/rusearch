use walkdir::WalkDir;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn search_dir(dir: &str, pattern: &str, absolute: bool) {
    let entries: Vec<_> = WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .collect();

    entries.par_iter().for_each(|entry| {
        if let Ok(file) = File::open(entry.path()) {
            let reader = BufReader::new(file);
            for (num, line) in reader.lines().enumerate() {
                if let Ok(line) = line {
                    if line.contains(pattern) {
                        let path_str = if absolute {
                            entry.path().to_path_buf().canonicalize().unwrap_or_else(|_| entry.path().to_path_buf())
                        } else {
                            match entry.path().strip_prefix(dir) {
                                Ok(rel) => rel.to_path_buf(),
                                Err(_) => entry.path().to_path_buf(),
                            }
                        };
                        println!("{}:{}:{}", path_str.display(), num + 1, line);
                    }
                }
            }
        }
    });
}
