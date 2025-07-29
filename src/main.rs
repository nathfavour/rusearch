use clap::Parser;

/// Fast recursive directory search CLI
#[derive(Parser, Debug)]
#[command(author, version, about = "Fast recursive directory search CLI", long_about = None)]
struct Args {
    /// Directory to search
    #[arg(short, long, default_value = ".")]
    dir: String,

    /// Pattern to search for (substring)
    #[arg(short, long)]
    pattern: String,
}

fn main() {
    let args = Args::parse();
    println!("Searching for '{}' in directory '{}'...", args.pattern, args.dir);

    use walkdir::WalkDir;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use rayon::prelude::*;
    let entries: Vec<_> = WalkDir::new(&args.dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .collect();

    entries.par_iter().for_each(|entry| {
        if let Ok(file) = File::open(entry.path()) {
            let reader = BufReader::new(file);
            for (num, line) in reader.lines().enumerate() {
                if let Ok(line) = line {
                    if line.contains(&args.pattern) {
                        println!("{}:{}:{}", entry.path().display(), num + 1, line);
                    }
                }
            }
        }
    });
}
