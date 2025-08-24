use std::{fs::File, io::{BufReader, BufWriter}, path::PathBuf};
use anyhow::{bail, Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: PathBuf,
}

fn main() -> Result<()> {
    let args: Cli = Cli::parse();

    if args.pattern.is_empty() {
        bail!("Pattern cannot be empty");
    }
    
    let f: File = File::open(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;
    let content: BufReader<File> = BufReader::new(f);

    let writer = BufWriter::new(std::io::stdout());

    grrs::find_matches(content, &args.pattern, writer)
}
