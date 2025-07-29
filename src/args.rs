use clap::Parser;
use std::fs;

/// Fast recursive directory search CLI
#[derive(Parser, Debug)]
#[command(author, version, about = "Fast recursive directory search CLI", long_about = None)]
pub struct Args {
    /// Directory to search
    #[arg(short, long, default_value = ".")]
    pub dir: String,

    /// Pattern to search for (substring)
    #[arg(short, long, required = false)]
    pub pattern: Option<String>,

    /// File containing pattern to search for
    #[arg(long, required = false)]
    pub pattern_file: Option<String>,
}

impl Args {
    pub fn get_pattern(&self) -> Result<String, String> {
        match (&self.pattern, &self.pattern_file) {
            (Some(p), None) => Ok(p.clone()),
            (None, Some(f)) => fs::read_to_string(f).map_err(|e| format!("Failed to read pattern file: {}", e)),
            (Some(_), Some(_)) => Err("Provide either --pattern or --pattern-file, not both.".to_string()),
            (None, None) => Err("You must provide --pattern or --pattern-file.".to_string()),
        }
    }
}
