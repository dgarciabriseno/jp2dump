use std::io::{Error, Result};
use std::fmt::Display;
use super::GenericBox;

#[derive(Debug)]
pub struct UrlBox {
    pub version: u8,
    pub flags: [u8; 3],
    pub location: String
}

#[derive(Debug)]
pub struct DataReferenceBox {
    /** Number of data references */
    count: u16,
    refs: Vec<UrlBox>
}

impl Display for DataReferenceBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Data Reference Box ({} refs)", self.count)?;
        let mut counter = 0;
        for reference in &self.refs {
            writeln!(f, "Reference {}, {:?}", counter, reference)?;
            counter += 1;
        }
        Ok(())
    }
}

impl DataReferenceBox {
    pub fn from_buffer(buf: Vec<u8>) -> Result<DataReferenceBox> {
        assert!(buf.len() >= 2);
        let count = u16::from_be_bytes(buf[0..2].try_into().unwrap());
        let mut ref_box = DataReferenceBox { count: count, refs: vec![] };
        let mut offset = 2;
        for i in 0..count {
            let url_box = GenericBox::from_buffer(buf[offset..buf.len()].to_vec())?;
            offset += url_box.length as usize;
            ref_box.refs.push(UrlBox::from_buffer(url_box.contents)?);
        }
        Ok(ref_box)
    }
}

impl UrlBox {
    pub fn from_buffer(buf: Vec<u8>) -> Result<UrlBox> {
        let version = buf[0];
        let flags = [buf[1], buf[2], buf[3]];
        let location = String::from_utf8(buf[4..].to_vec());
        if location.is_err() {
            return Err(Error::new(std::io::ErrorKind::InvalidData, location.unwrap_err()));
        }
        Ok(UrlBox {version: version, flags: flags, location: location.unwrap()})
    }
}