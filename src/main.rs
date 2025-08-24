use std::{fs::File, io::{BufRead, BufReader}, path::PathBuf};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: PathBuf,
}

fn main() {
    // Before clap: manually reading std::env::args and mapping into Cli struct
    /*
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no pattern given");

    let args = Cli {
        pattern,
        path: PathBuf::from(path),
    };
    */

    // After clap: automatically maps cli args into Cli struct and provides a --help
    let args = Cli::parse();
    grrs(&args);
}

fn grrs(args: &Cli) {
    // Current best implementation
    //let _ = grrs_std(&args);
    let _ = grrs_buffered(&args);
}

#[allow(dead_code)]
fn grrs_std(args: &Cli) -> Result<(), std::io::Error> {
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}

#[allow(dead_code)]
fn grrs_buffered(args: &Cli) -> Result<(), std::io::Error> {
    let f = File::open(&args.path)?;
    let reader = BufReader::new(f);

    // println!("{}", reader.lines().nth(1).unwrap().unwrap());
    for line in reader.lines() {
        let line = line?;
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
