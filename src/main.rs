use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use ahash::AHashSet;
use anyhow::Result;
use env_logger::Env;
use log::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "mash")]
struct MashOpts {
    /// The target files on which the selector is applied
    targets: Vec<PathBuf>,

    /// A file with a string per line. Any row in targets with a column that matches
    /// a selector will be printed.
    #[structopt(short, long)]
    selector: PathBuf,

    #[structopt(short, long, default_value = "\t")]
    delim: String,

    /// The column that is the target of the selector
    #[structopt(short, long, default_value = "1")]
    column_target: usize,

    /// Invert the lookup, so any row in targets not in selector will be printed.
    #[structopt(short, long)]
    invert: bool
}

fn create_selector<P: AsRef<Path>>(path: P) -> Result<AHashSet<String>> {
    let reader = BufReader::new(File::open(path)?);
    let mut set = AHashSet::new();
    for line in reader.lines() {
        set.insert(line?);
    }
    Ok(set)
}

fn main() -> Result<()> {
    let mash_opts = MashOpts::from_args();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let selector = create_selector(&mash_opts.selector)?;
    let stdout = std::io::stdout();
    let writer = stdout.lock();

    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .delimiter(*mash_opts.delim.as_bytes().first().unwrap())
        .from_writer(writer);
    let mut found = 0;

    for target in mash_opts.targets.iter() {
        info!("Processing {:?}", target);
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(*mash_opts.delim.as_bytes().first().unwrap())
            .from_path(&target)?;

        for record in reader.records() {
            let record = record?;
            if selector.contains(&record[mash_opts.column_target - 1]) ^ mash_opts.invert {
                found += 1;
                writer.write_record(&record)?;
            }
        }
    }
    info!("Selected {} records from input targets.", found);
    Ok(())
}
