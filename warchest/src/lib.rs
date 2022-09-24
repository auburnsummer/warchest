use clap::{Parser, ValueEnum};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::str;

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

#[derive(Debug)]
pub struct GZHeader {
    name: String,
    mtime: u32,
    os: u8,
}

// given a file, read the gzip header. by the end of this, we are at the beginning of the
// first DEFLATE block.
fn check_gzip_header(reader: &mut BufReader<File>) -> Result<GZHeader, Box<dyn Error>> {
    // ID1 and ID2
    let mut sig = [0; 2];
    reader.read_exact(&mut sig)?;
    // check it. it should be the gzip magic bytes.
    if sig != [31, 139] {
        return Err("Signature of the file was not a gzip.".into());
    }

    // CM
    let mut cm = [0; 1];
    reader.read_exact(&mut cm)?;
    if cm != [8] {
        return Err("Compression type must be DEFLATE.".into());
    }

    // FLG
    let mut flg = [0; 1];
    reader.read_exact(&mut flg)?;

    let flg = flg[0];
    // let ftext = flg >> 0 & 1;  // ignore ftext.
    let fhcrc = flg >> 1 & 1 == 1;
    let fextra = flg >> 2 & 1 == 1;
    let fname_flag = flg >> 3 & 1 == 1;
    let fcomment = flg >> 4 & 1 == 1;

    // for now, assume fextra doesn't exist, and throw an error if it does.
    if fextra {
        return Err("FEXTRA field is not supported at the moment.".into());
    }
    // ditto with fcomment.
    if fcomment {
        return Err("FCOMMENT field is not supported at the moment.".into());
    }
    // ditto with fhcrc.
    if fhcrc {
        return Err("FHCRC field is not supported at the moment.".into());
    }

    // MTIME
    let mut mtime = [0; 4];
    reader.read_exact(&mut mtime)?;
    let mtime = u32::from_le_bytes(mtime);

    // XFL
    let mut xfl = [0; 1];
    reader.read_exact(&mut xfl)?;
    //  let xfl = xfl[0]; // not using this atm, but we do need to consume it.

    // OS
    let mut os = [0; 1];
    reader.read_exact(&mut os)?;
    let os = os[0];

    // for fname......
    let mut fname: Vec<u8> = vec![];
    let mut char = vec![0; 1];
    if fname_flag {
        loop {
            reader.read_exact(&mut char)?;
            if char[0] == 0 {
                break;
            }
            fname.push(char[0]);
        }
    }
    let fname = str::from_utf8(&fname)?;

    Ok(GZHeader {
        name: fname.to_string(),
        mtime: mtime,
        os: os,
    })
}

pub fn read_deflate_block(reader: &mut BufReader<File>) {}

pub fn generate(cli: &Cli) -> Result<(), Box<dyn Error>> {
    let name = match &cli.name {
        Some(n) => n,
        None => return Err("A file was not given.".into()),
    };
    dbg!(name);
    let f = File::open(name)?;
    let mut reader = BufReader::new(f);

    let header = check_gzip_header(&mut reader)?;

    dbg!(&header);

    Ok(())
}
