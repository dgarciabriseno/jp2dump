use std::io::{Cursor, Read, Result};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

// I got lazy, there's 86 values in the standard.
// #[repr(u16)]
// enum StandardFlag {
//     NotUnderstood = 0,
//     NoExtensions = 1,
//     MultipleCompositionLayers = 2,
//     Deprecated3 = 3,
//     Profile1Codestream = 4,
//     UnrestrictedCodestream = 5,
//     UnrestrictedExtensions = 6,
//     JPEGCodestream = 7,
//     Deprecated8 = 8,
//     NonPremultipliedOpacity = 9,
//     PremultipliedOpacity = 10,
//     ChromaKeyOpacity = 11,
//     Deprecated12 = 12,
//     FragmentsInOrderInFile = 13,
//     FragmentsOutOfOrderInFile = 14,
//     // Fragments are not in file, but are on the local machine
//     FragmentsLocalNotInFile = 15,
//     // Fragments are not in file, but are remotely accessible via url
//     FragmentsRemoteNotInFile = 16,

// }
#[derive(Debug)]
pub struct VendorFeature {
    pub feature: u128,
    pub mask: Vec<u8>
}

#[derive(Debug)]
pub struct StandardFlag {
    pub flag: u16,
    pub mask: Vec<u8>
}

#[derive(Debug)]
pub struct ReaderRequirements {
    /** Number of bytes used for compatibility masks */
    pub mask_length: u8,
    pub fully_understand_aspects: Vec<u8>,
    pub display_contents_mask: Vec<u8>,
    pub flags: Vec<StandardFlag>,
    pub vendor_features: Vec<VendorFeature>
}

impl ReaderRequirements {
    pub fn from_buffer(buf: Vec<u8>) -> Result<ReaderRequirements> {
        // // println!("{:?}", buf);
        // Just learned about this byteorder crate
        let mut reader = Cursor::new(buf);

        let mask_length = reader.read_u8()?;
        // println!("Mask length is {}", mask_length);
        // Read FUA, which is mask_length bytes
        let mut fua: Vec<u8> = vec![0; mask_length as usize];
        reader.read_exact(&mut fua)?;
        // println!("FUA: {:x?}", fua);
        // Read display contents mask, which is mask_length bytes
        let mut dcm: Vec<u8> = vec![0; mask_length as usize];
        reader.read_exact(&mut dcm)?;
        // println!("DCM: {:x?}", dcm);
        // Read number of standard flags, big endian u16
        let nflags = reader.read_u16::<BigEndian>()?;
        // println!("There are {} standard flags", nflags);
        // Read all the flags
        let mut standard_flags: Vec<StandardFlag> = vec![];
        for _ in 0..nflags {
            let mut mask: Vec<u8> = vec![0; mask_length as usize];
            let flag = reader.read_u16::<BigEndian>()?;
            // println!("Flag: {}", flag);
            reader.read_exact(&mut mask)?;
            // println!("Mask: {:?}", mask);
            standard_flags.push(StandardFlag {
                flag: flag,
                mask: mask
            });
        }
        // Read number of vendor features
        let n_features = reader.read_u16::<BigEndian>()?;
        // println!("There are {} vendor features", n_features);
        // Read all the vendor features
        let mut features: Vec<VendorFeature> = vec![];
        for _ in 0..n_features {
            let mut mask: Vec<u8> = vec![0; mask_length as usize];
            let feature = reader.read_u128::<BigEndian>()?;
            reader.read_exact(&mut mask)?;
            features.push(VendorFeature {
                feature: feature,
                mask: mask
            });
        }
        Ok(ReaderRequirements {
            mask_length: mask_length,
            fully_understand_aspects: fua,
            display_contents_mask: dcm,
            flags: standard_flags,
            vendor_features: features
        })
    }
}