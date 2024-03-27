use std::fmt::Display;

#[derive(Debug)]
pub struct FTypeBox {
    brand: String,
    minor_version: u32,
    cl: Vec<String>
}

impl Display for FTypeBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FTypeBox {
    pub fn from_buffer(buf: Vec<u8>) -> FTypeBox {
        assert!(buf.len() >= 8, "ftyp field is not long enough");
        assert!((buf.len() % 4) == 0, "ftyp field has an invalid Compatibility List");
        let numcl = (buf.len() - 8) / 4;
        let brand = String::from_utf8(buf[0..4].to_vec()).unwrap();
        let mv = u32::from_be_bytes(buf[4..8].try_into().unwrap());
        let mut cl_list = vec![];
        for i in 0..numcl {
            let offset = 8 + 4*i;
            cl_list.push(String::from_utf8(buf[offset..(offset+4)].to_vec()).unwrap());
        }
        FTypeBox {
            brand: brand,
            minor_version: mv,
            cl: cl_list
        }
    }
}