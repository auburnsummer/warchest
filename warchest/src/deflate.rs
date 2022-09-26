use std::{error::Error, fs::File, io::BufReader};

use crate::read::{move_to_next_byte, read_one_bit, read_two_bytes, skip_n, Progress};

#[derive(Debug, Clone, Copy)]
pub enum CompressionType {
    NoCompression,
    FixedHuffman,
    DynamicHuffman,
}

#[derive(Debug, Clone, Copy)]
pub struct BlockInfo {
    pub block_type: CompressionType,
    pub last: bool,
    pub byte: u64, // starting position of this block in the compressed stream
    pub bit: u8,   // ''
}

pub fn read_no_compression_block(
    reader: &mut BufReader<File>,
    prog: &mut Progress,
) -> Result<(), Box<dyn Error>> {
    move_to_next_byte(prog);
    let len = read_two_bytes(reader, prog)?;
    let len = u16::from_le_bytes(len);
    // i'm not reading this, it's the one's compliment of len and we just """assume""" it's correct.
    // TODO: check this.
    let _nlen = read_two_bytes(reader, prog)?;
    // also, we don't actually read the contents.
    skip_n(reader, prog, len.into())?;
    Ok(())
}

pub fn read_fixed_huffman_block(
    reader: &mut BufReader<File>,
    prog: &mut Progress,
) -> Result<(), Box<dyn Error>> {
    loop {
        
    }
    Ok(())
}

pub fn read_deflate_block(
    reader: &mut BufReader<File>,
    prog: &mut Progress,
) -> Result<BlockInfo, Box<dyn Error>> {
    // alright, so the header is always on byte boundaries. but now it goes in to bits.
    let starting_byte = prog.byte;
    let starting_bit = prog.bit;
    let bfinal = read_one_bit(reader, prog)?;
    let btype1 = read_one_bit(reader, prog)?;
    let btype2 = read_one_bit(reader, prog)?;

    let ctype = if !btype1 && !btype2 {
        CompressionType::NoCompression
    } else if !btype1 && btype2 {
        CompressionType::FixedHuffman
    } else if btype1 && !btype2 {
        CompressionType::DynamicHuffman
    } else {
        return Err("Huffman code 11 is an error".into());
    };

    match ctype {
        CompressionType::NoCompression => {
            read_no_compression_block(reader, prog)?;
        }
        CompressionType::FixedHuffman => todo!(),
        CompressionType::DynamicHuffman => todo!(),
    }

    Ok(BlockInfo {
        block_type: ctype,
        last: bfinal,
        byte: starting_byte,
        bit: starting_bit,
    })
}
