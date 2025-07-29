use clap::Parser;
use std::fs;

/// Fast recursive directory search CLI
///
/// Usage:
///   rusearch --pattern foo
///   rusearch --pattern-file file.txt
///   rusearch "foo"   # shortcut: searches current directory recursively for 'foo'
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

    /// Print absolute file paths (default: relative)
    #[arg(short = 'f', long = "um", help = "Print absolute file paths instead of relative")]
    pub absolute: bool,

    /// Shortcut: pattern as a single positional argument
    #[arg(required = false, index = 1, help = "Shortcut: search for this pattern in the current directory recursively if no --pattern or --pattern-file is given")] 
    pub shortcut_pattern: Option<String>,
}

impl Args {
    pub fn get_pattern(&self) -> Result<String, String> {
        match (&self.pattern, &self.pattern_file, &self.shortcut_pattern) {
            (Some(p), None, _) => Ok(p.clone()),
            (None, Some(f), _) => fs::read_to_string(f).map_err(|e| format!("Failed to read pattern file: {}", e)),
            (Some(_), Some(_), _) => Err("Provide either --pattern or --pattern-file, not both.".to_string()),
            (None, None, Some(s)) => Ok(s.clone()),
            (None, None, None) => Err("You must provide --pattern, --pattern-file, or a positional pattern argument.".to_string()),
        }
    }
}
