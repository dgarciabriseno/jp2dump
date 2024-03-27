use std::{fmt::Display, io::Error};
use std::io::Result;

#[derive(Debug)]
pub struct XmlBox {
    pub xml: String
}

impl Display for XmlBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.xml)
    }
}

impl XmlBox {
    pub fn from_buffer(buf: Vec<u8>) -> Result<XmlBox> {
        let xml = String::from_utf8(buf);
        match xml {
            Ok(data) => Ok(XmlBox { xml: data }),
            Err(err) => Err(Error::new(std::io::ErrorKind::InvalidData, err))
        }
    }
}

