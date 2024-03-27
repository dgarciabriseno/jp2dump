use clap::Parser;
use std::io::Result;
use std::fs::File;
mod boxes;
use crate::boxes::print_box;

/// This program prints all jpeg2000 boxes to stdout
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// JPEG2000 file to read
    jp2_file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    // Open and process the jp2 file.
    let result = match File::open(&args.jp2_file) {
        Ok(jp2) => dump_boxes(jp2),
        Err(err) => Ok(println!("Couldn't open {0}: {err}", args.jp2_file))
    };
    // If an error occurred during processing, log it here.
    if result.is_err() {
        println!("Error occurred while reading jpeg2000 file: {0}", result.unwrap_err());
    }
    Ok(())
}

fn dump_boxes(mut jp2: File) -> Result<()> {
    loop {
        print_box(&mut jp2)?;
    }
}