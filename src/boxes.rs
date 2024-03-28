mod generic_box;
mod ftype_box;
mod association_box;
mod fragment_list_box;
mod number_list;
mod xml_box;
mod data_reference_box;
mod reader_requirements;
mod img_header_box;
mod color_box;
use ftype_box::FTypeBox;

use std::{fmt::Display, fs::File};
use std::io::Result;

use crate::boxes::color_box::ColorBox;

use self::association_box::print_associations;
use self::data_reference_box::DataReferenceBox;
use self::fragment_list_box::FragmentListBox;
use self::generic_box::GenericBox;
use self::img_header_box::ImageHeader;
use self::reader_requirements::ReaderRequirements;

pub fn print_superbox(superbox: GenericBox) -> Result<()> {
    let length = superbox.contents.len();
    let mut ptr = 0;
    println!("----- Superbox {} -----", superbox.box_type);
    while ptr < length {
        let jp2box = GenericBox::from_buffer(superbox.contents[ptr..length].to_vec())?;
        match jp2box.box_type.as_str() {
            "ihdr" => println!("{:?}", ImageHeader::from_buffer(jp2box.contents)?),
            "colr" => println!("{:?}", ColorBox::from_buffer(jp2box.contents)?),
            _ => println!("{jp2box}")
        }
        ptr += jp2box.length as usize;
    }
    println!("-----------------------");
    Ok(())
}

pub fn print_box(fp: &mut File) -> Result<()> {
    let jp2box = GenericBox::from_fp(fp)?;
    match jp2box.box_type.as_str() {
        "ftyp" => println!("{0}", FTypeBox::from_buffer(jp2box.contents)),
        "asoc" => print_associations(jp2box.length, jp2box.contents)?,
        "ftbl" => println!("Fragment Table:\n{0}", FragmentListBox::from_buffer(jp2box.contents[8..jp2box.contents.len()].to_vec())?),
        "dtbl" => println!("{}", DataReferenceBox::from_buffer(jp2box.contents)?),
        "rreq" => println!("{:?}", ReaderRequirements::from_buffer(jp2box.contents)?),
        "jp2h" => print_superbox(jp2box)?,
        _ => println!("{jp2box}")
    }
    Ok(())
}