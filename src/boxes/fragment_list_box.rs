use std::fmt::Display;
use std::io::Result;
use std::mem::size_of;

#[derive(Debug)]
pub struct FragmentListBox {
    /** Number of fragments */
    nf: u16,
    /** Offset to the start of the fragment in the file specified via the data reference */
    offset: u64,
    /** Length of the fragment */
    len: u32,
    /** Index of the data reference box for this fragment */
    index: u16
}

impl Display for FragmentListBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FragmentListBox {
    pub fn from_buffer(data: Vec<u8>) -> Result<FragmentListBox> {
        assert!(data.len() == size_of::<FragmentListBox>());
        Ok(FragmentListBox {
            nf: u16::from_be_bytes(data[0..2].try_into().unwrap()),
            offset: u64::from_be_bytes(data[2..10].try_into().unwrap()),
            len: u32::from_be_bytes(data[10..14].try_into().unwrap()),
            index: u16::from_be_bytes(data[14..16].try_into().unwrap())
        })
    }
}