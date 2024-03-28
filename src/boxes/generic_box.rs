use std::fmt::Display;
use std::fs::File;
use std::io::{Error, Read, Result, Seek};
use std::io;

#[derive(Debug)]
pub struct GenericBox {
    pub length: u32,
    pub box_type: String,
    pub contents: Vec<u8>
}

impl Display for GenericBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl GenericBox {
    /** Read a jp2 box from a file object */
    pub fn from_fp(fp: &mut File) -> Result<GenericBox> {
        // Declare data buffer
        let mut buf: [u8; 4] = [0; 4];
        // Read box length
        fp.read_exact(&mut buf)?;
        let length: u32 = u32::from_be_bytes(buf);

        // Read box type
        fp.read_exact(&mut buf)?;
        let btype = String::from_utf8(buf.into());
        if btype.is_err() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("{0}", btype.unwrap_err())));
        }

        if length == 0 && btype.clone().unwrap() == "jp2c" {
            let position = fp.stream_position()?;
            let codestream_length = fp.seek(io::SeekFrom::End(0))? - position;
            println!("Found codestream at file offset {}", position);
            println!("Assumed codestream runs to end of file. Length: {}", codestream_length);
            std::process::exit(0);
        }

        let data_size = length - 8;
        let mut buf: Vec<u8> = vec![0; data_size as usize];
        if data_size > 0 {
            fp.read_exact(&mut buf)?;
        }

        Ok(GenericBox { length: length, box_type: btype.unwrap(), contents: buf })
    }

    /** Read a jp2 box from a buffer */
    pub fn from_buffer(buf: Vec<u8>) -> Result<GenericBox> {
        let length: Result<u32> = match buf[0..4].try_into() {
            Ok(bytes) => Ok(u32::from_be_bytes(bytes)),
            Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, format!("{0}", e)))
        };
        if length.is_err() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, length.unwrap_err()));
        }
        let btype = String::from_utf8(buf[4..8].to_vec());
        if btype.is_err() {
            return Err(io::Error::new(io::ErrorKind::InvalidData,btype.unwrap_err()));
        }
        let length = length.unwrap();
        Ok(GenericBox { length: length, box_type: btype.unwrap(), contents: buf[8..(length as usize)].to_vec() })
    }
}