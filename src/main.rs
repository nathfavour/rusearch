mod args;
mod search;

use args::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
    let pattern = match args.get_pattern() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
    println!("Searching for '{}' in directory '{}'...", pattern, args.dir);
    search::search_dir(&args.dir, &pattern, args.absolute);
}
