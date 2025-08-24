use std::{fs::File, io::{BufRead, BufReader}, path::PathBuf};
use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: PathBuf,
}

fn main() -> Result<()> {
    let args: Cli = Cli::parse();
    grrs(&args)
}

fn grrs(args: &Cli) -> Result<()> {
    let f: File = File::open(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;
    let reader: BufReader<File> = BufReader::new(f);

    for line in reader.lines() {
        let line: String = line
            .with_context(|| "could not read line")?;
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
