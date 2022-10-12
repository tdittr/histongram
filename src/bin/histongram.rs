use std::fs::read_to_string;
use std::mem::size_of;
use std::path::PathBuf;

use clap::Parser;
use color_eyre::Result;
use compact_str::CompactString;

use histongram::Histogram;

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

    let hist: Histogram<CompactString> = data.split_whitespace().collect();

    if args.print {
        for (word, cnt) in hist.sorted_occurrences().into_iter().take(100) {
            println!("{word:?}: {cnt}");
        }
    }

    Ok(())
}
