use std::io::{Cursor, Result};

use byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug)]
pub struct ColorBox {
    pub method: u8,
    pub precedence: u8,
    pub approximation: u8,
    pub enumerated_colorspace: u32,
    pub profile: Vec<u8>
}

impl ColorBox {
    pub fn from_buffer(buf: Vec<u8>) -> Result<ColorBox> {
        let mut reader = Cursor::new(&buf);
        let method = reader.read_u8()?;
        let prec = reader.read_u8()?;
        let approx = reader.read_u8()?;
        let enumcs: u32 = if method == 1 { reader.read_u32::<BigEndian>()? } else { 0 };
        let profile = if method == 2 { buf[reader.position() as usize..buf.len()].to_vec() } else { vec![] };
        Ok(ColorBox {
            method: method,
            precedence: prec,
            approximation: approx,
            enumerated_colorspace: enumcs,
            profile: profile
        })
    }
}