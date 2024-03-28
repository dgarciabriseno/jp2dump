use std::io::{Cursor, Result};

use byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug)]
pub struct ImageHeader {
    pub height: u32,
    pub width: u32,
    pub num_components: u16,
    pub bits_per_component: u8,
    pub compression_type: u8,
    pub colorspace_unknown: u8,
    pub intellectual_property: u8
}

impl ImageHeader {
    pub fn from_buffer(buf: Vec<u8>) -> Result<ImageHeader> {
        let mut reader = Cursor::new(buf);
        Ok(ImageHeader {
            height: reader.read_u32::<BigEndian>()?,
            width: reader.read_u32::<BigEndian>()?,
            num_components: reader.read_u16::<BigEndian>()?,
            bits_per_component: reader.read_u8()?,
            compression_type: reader.read_u8()?,
            colorspace_unknown: reader.read_u8()?,
            intellectual_property: reader.read_u8()?
        })
    }
}