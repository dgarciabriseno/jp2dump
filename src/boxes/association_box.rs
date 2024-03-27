use std::io::Result;
use crate::boxes::xml_box::XmlBox;

use super::GenericBox;
use super::number_list::NumberListBox;

pub fn print_associations(len: u32, data: Vec<u8>) -> Result<()> {
    println!("Associations:");

    let mut ptr: usize = 0;
    while ptr < data.len() {
        let jp2box = GenericBox::from_buffer(data[ptr..data.len()].to_vec())?;
        match jp2box.box_type.as_str() {
            "free" => println!("Unused Space"),
            "asoc" => print_associations(jp2box.length, jp2box.contents)?,
            "nlst" => println!("{}", NumberListBox::from_buffer(jp2box.contents)?),
            "xml " => println!("{}", XmlBox::from_buffer(jp2box.contents)?),
            _ => println!("{jp2box}")
        }
        ptr += jp2box.length as usize;
    }

    Ok(())
}