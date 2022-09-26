use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
    vec,
};

use std::str;

use crate::read::{
    read_four_bytes, read_n, read_null_terminated_string, read_one_byte, read_two_bytes, Progress,
};

#[derive(Debug)]
pub struct GZHeader {
    name: String,
    mtime: u32,
    os: u8,
}

// given a file, read the gzip header. by the end of this, we are at the beginning of the
// first DEFLATE block.
pub fn check_gzip_header(
    reader: &mut BufReader<File>,
    prog: &mut Progress,
) -> Result<GZHeader, Box<dyn Error>> {
    // ID1 and ID2
    let sig = read_two_bytes(reader, prog)?;
    if sig != [31, 139] {
        return Err("Signature of the file was not a gzip.".into());
    }

    let cm = read_one_byte(reader, prog)?;
    if cm != 8 {
        return Err("Compression type must be DEFLATE.".into());
    }

    // FLG
    let flg = read_one_byte(reader, prog)?;

    // let ftext = flg >> 0 & 1;  // ignore ftext.
    let fhcrc = flg >> 1 & 1 == 1;
    let fextra_flag = flg >> 2 & 1 == 1;
    let fname_flag = flg >> 3 & 1 == 1;
    let fcomment = flg >> 4 & 1 == 1;

    // MTIME
    let mtime = read_four_bytes(reader, prog)?;
    let mtime = u32::from_le_bytes(mtime);

    // XFL not using this atm, but we do need to consume it.
    let _xfl = read_one_byte(reader, prog);

    // OS
    let os = read_one_byte(reader, prog)?;

    // FEXTRA
    let mut fextra: Vec<u8> = vec![];
    let fextra = if fextra_flag {
        let len = read_two_bytes(reader, prog)?;
        let len = u16::from_le_bytes(len);
        read_n(reader, prog, &mut fextra, len.into())?;
        fextra
    } else {
        fextra
    };

    // for now, assume FCOMMENT field doesn't exist, and throw an error if it does.
    if fcomment {
        return Err("FCOMMENT field is not supported at the moment.".into());
    };
    // ditto with fhcrc.
    if fhcrc {
        return Err("FHCRC field is not supported at the moment.".into());
    };

    // FNAME
    let mut fname = vec![];
    let fname = if fname_flag {
        read_null_terminated_string(reader, prog, &mut fname)?;
        str::from_utf8(&fname)?
    } else {
        ""
    };

    Ok(GZHeader {
        name: fname.to_string(),
        mtime: mtime,
        os: os,
    })
}
