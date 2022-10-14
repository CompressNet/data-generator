mod features;

use clap::Parser;
use color_eyre::{Help, Result};
use features::get_features;
use std::{path::PathBuf, fs::File};

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

    

    Ok(())
}
