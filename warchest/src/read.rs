use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
};

// How far are we in the file? This is on the basis of the compressed stream.
#[derive(Debug)]
pub struct Progress {
    pub byte: u64, // which byte are we up to?
    pub bit: u8,   // which bit are we up to? blocks can take up non-integral numbers of bytes.
    // to clarify, output is byte orientated, the input stream is bit orientated.
    pub buf: u8, // a buffer storing a single byte.
}

impl Progress {
    pub fn new() -> Self {
        Progress {
            byte: 0,
            bit: 0,
            buf: 0,
        }
    }
}

pub fn read_one_byte(
    reader: &mut BufReader<File>,
    prog: &mut Progress,
) -> Result<u8, Box<dyn Error>> {
    // for now, this assumes we are on a byte boundary.
    if prog.bit != 0 {
        return Err("read_one_byte only works on a byte boundary.".into());
    }
    let mut os = [0; 1];
    reader.read_exact(&mut os)?;
    prog.byte += 1;
    Ok(os[0])
}

pub fn read_two_bytes(
    reader: &mut BufReader<File>,
    prog: &mut Progress,
) -> Result<[u8; 2], Box<dyn Error>> {
    // for now, this assumes we are on a byte boundary.
    if prog.bit != 0 {
        return Err("read_two_bytes only works on a byte boundary.".into());
    }
    let mut os = [0; 2];
    reader.read_exact(&mut os)?;
    prog.byte += 2;
    Ok(os)
}

pub fn read_four_bytes(
    reader: &mut BufReader<File>,
    prog: &mut Progress,
) -> Result<[u8; 4], Box<dyn Error>> {
    // for now, this assumes we are on a byte boundary.
    if prog.bit != 0 {
        return Err("read_two_bytes only works on a byte boundary.".into());
    }
    let mut os = [0; 4];
    reader.read_exact(&mut os)?;
    prog.byte += 4;
    Ok(os)
}

pub fn read_n(
    reader: &mut BufReader<File>,
    prog: &mut Progress,
    buf: &mut Vec<u8>,
    n: usize,
) -> Result<(), Box<dyn Error>> {
    for _ in 0..n {
        let byte = read_one_byte(reader, prog)?;
        buf.push(byte);
    }
    Ok(())
}

pub fn skip_n(
    reader: &mut BufReader<File>,
    prog: &mut Progress,
    n: usize,
) -> Result<(), Box<dyn Error>> {
    for _ in 0..n {
        let _ = read_one_byte(reader, prog)?;
    }
    Ok(())
}

pub fn read_null_terminated_string(
    reader: &mut BufReader<File>,
    prog: &mut Progress,
    buf: &mut Vec<u8>,
) -> Result<(), Box<dyn Error>> {
    // for now, this assumes we are on a byte boundary.
    if prog.bit != 0 {
        return Err("read_two_bytes only works on a byte boundary.".into());
    }

    Ok(loop {
        let value = read_one_byte(reader, prog)?;
        if value == 0 {
            break;
        }
        buf.push(value);
    })
}

pub fn read_one_bit(
    reader: &mut BufReader<File>,
    prog: &mut Progress,
) -> Result<bool, Box<dyn Error>> {
    // OK, if we're reading "bit 0", we need to load in a new byte into the buffer.
    // ...but without changing the byte progress.
    if prog.bit == 0 {
        let mut temp_buf = [0; 1];
        reader.read_exact(&mut temp_buf)?;
        prog.buf = temp_buf[0];
    }

    let value = prog.buf >> prog.bit & 1 == 1;
    prog.bit = (prog.bit) + 1 % 8;

    Ok(value)
}

pub fn move_to_next_byte(prog: &mut Progress) {
    if prog.bit != 0 {
        prog.bit = 0;
        prog.byte += 1;    
    }
}
