use std::{fs::File, io::{BufRead, BufReader}, path::PathBuf};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Cli = Cli::parse();
    grrs(&args)
}

fn grrs(args: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    let f: File = File::open(&args.path)?;
    let reader: BufReader<File> = BufReader::new(f);

    for line in reader.lines() {
        let line: String = line?;
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
