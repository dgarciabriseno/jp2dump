use std::io::Result;
use std::fmt::Display;

#[derive(Debug)]
pub struct NumberListBox {
    pub an: Vec<u32>
}

impl Display for NumberListBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08x?}", self)
    }
}

impl NumberListBox {
    pub fn from_buffer(buf: Vec<u8>) -> Result<NumberListBox> {
        assert!((buf.len() % 4) == 0);
        let count = buf.len() / 4;
        let mut nlb = NumberListBox { an: vec![] };
        for i in 0..count {
            let offset = i * 4;
            nlb.an.push(u32::from_be_bytes(buf[offset..(offset+4)].try_into().unwrap()))
        }
        Ok(nlb)
    }
}