use std::{fs::File, io::{self, BufReader, BufWriter}};
use anyhow::{bail, Context, Result};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: Option<String>,
}

fn main() -> Result<()> {
    let args: Cli = Cli::parse();

    if args.pattern.is_empty() {
        bail!("Pattern cannot be empty");
    }

    if args.path.is_none() {
        return grrs_stdin(args);
    }

    return grrs_file(args);
}

fn grrs_stdin(args: Cli) -> Result<()> {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());
    let writer = BufWriter::new(std::io::stdout());
    grrs::find_matches(reader, &args.pattern, writer)
}

fn grrs_file(args: Cli) -> Result<()> {
    let path = args.path.unwrap();
    let f: File = File::open(&path)
        .with_context(|| format!("could not read file `{}`", &path))?;
    let content: BufReader<File> = BufReader::new(f);
    let writer = BufWriter::new(std::io::stdout());
    grrs::find_matches(content, &args.pattern, writer)
}
