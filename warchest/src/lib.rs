use clap::{Parser, ValueEnum};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

mod deflate;
mod header;
mod read;
use crate::deflate::read_deflate_block;
use crate::header::check_gzip_header;
use crate::read::{read_four_bytes, read_one_bit, Progress};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// What mode to run the program in
    #[clap(arg_enum, value_parser)]
    pub mode: Mode,

    /// If generating, a path to the .warc.gz file. If fetching, a download URL for the file.
    #[clap(value_parser)]
    pub name: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    Generate,
    Fetch,
}

pub fn generate(cli: &Cli) -> Result<(), Box<dyn Error>> {
    let name = match &cli.name {
        Some(n) => n,
        None => return Err("A file was not given.".into()),
    };

    let mut prog = Progress::new();
    let f = File::open(name)?;
    let mut reader = BufReader::new(f);

    loop {
        let header = match check_gzip_header(&mut reader, &mut prog) {
            Ok(h) => h,
            Err(_) => break, // todo: this should fail if the gzip header is actually bad,
                             // but apparently I have to redo all my error handling for it to work correctly rip me
        };
        dbg!(header);

        loop {
            let block = read_deflate_block(&mut reader, &mut prog)?;
            dbg!(block);
            if block.last {
                break;
            }
        }

        // we don't actually decompress, so we can't verify these.
        // just assume they are correct.
        let _crc = read_four_bytes(&mut reader, &mut prog)?;
        let _isize = read_four_bytes(&mut reader, &mut prog)?;
    }

    Ok(())
}
