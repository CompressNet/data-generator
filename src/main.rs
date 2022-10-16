mod features;

use clap::Parser;
use color_eyre::{Help, Result};
use features::{get_features, Features, HEADER_SIZE, RANDOM_BYTES_SIZE};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressIterator, ProgressStyle};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::{
    fs::{self, File},
    path::PathBuf,
    time::Duration,
};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Cli {
    #[clap(short, long)]
    /// The directory containing files to generate data from.
    input_directory: PathBuf,

    #[clap(short, long)]
    /// The output CSV file to write to.
    output_file: PathBuf,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();

    // Ensure input_directory exists
    if !args.input_directory.exists() {
        return Err(color_eyre::eyre::eyre!(
            "Input directory does not exist: {:?}",
            args.input_directory
        ))
        .with_suggestion(|| {
            format!(
                "Create the directory: {:?} or supply an existing directory",
                args.input_directory
            )
        });
    }

    let files: Vec<PathBuf> = fs::read_dir(&args.input_directory)?
        .map(|entry| entry.unwrap().path())
        .collect::<Vec<_>>();

    let pb = create_progress_bar(files.len() as u64)?;
    pb.set_message("Generating data from input files");

    let features = files
        .par_iter()
        .progress_with(pb)
        .map(|file_path| {
            let file = File::open(file_path).unwrap();
            get_features(file_path, &file)
        })
        .collect::<Result<Vec<Features>>>()?;

    let pb = create_progress_bar(features.len() as u64)?;
    pb.set_message("Writing to CSV");

    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_path(&args.output_file)?;

    write_header(&mut writer)?;

    for feature in features.iter().progress_with(pb) {
        writer.serialize(feature)?;
    }

    Ok(())
}

fn write_header(writer: &mut csv::Writer<File>) -> Result<()> {
    // HACK: csv doesn't have a way to write headers for fixed sized arrays in a struct,
    // so we'll manually write the header.
    let mut header = Vec::new();

    header.push("file_name".to_owned());
    header.push("entropy".to_owned());

    // header 1-8
    for i in 1..=HEADER_SIZE {
        header.push(format!("header_{}", i));
    }

    // random_bytes 1-32
    for i in 1..=RANDOM_BYTES_SIZE {
        header.push(format!("random_byte_{}", i));
    }

    header.push("compression_ratio".to_owned());

    writer.write_record(&header)?;
    Ok(())
}

fn create_progress_bar(len: u64) -> Result<ProgressBar> {
    let pb = ProgressBar::new(len);
    pb.set_style(ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} ETA: [{eta_precise}] {pos:>7}/{len:7} {msg}",
    )?);
    pb.enable_steady_tick(Duration::from_millis(100));
    Ok(pb)
}
