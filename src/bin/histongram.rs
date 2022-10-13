use std::fs::read_to_string;
use std::path::PathBuf;

use clap::Parser;
use color_eyre::Result;
use compact_str::CompactString;

use histongram::{Histogram, Ngrams};

#[derive(Parser, Debug, Clone)]
struct Args {
    file: PathBuf,

    #[clap(long)]
    print: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    let data = read_to_string(args.file)?;

    let ngrams = Ngrams::new(1..=5).count(data.split_whitespace());

    if args.print {
        println!("{ngrams:?}");
    }

    Ok(())
}
