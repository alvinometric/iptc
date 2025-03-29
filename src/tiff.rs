use crate::{ReadUtils, tags};
use std::collections::HashMap;
use std::error::Error;
use std::io::Cursor;
use tags::IPTCTag;
use tiff::{
    decoder::{Decoder, DecodingResult, ifd::Value},
    tags::Tag,
};
use xml::reader::{EventReader, XmlEvent};

pub(crate) struct TIFFReader;

impl TIFFReader {
    pub fn read_iptc(buffer: &Vec<u8>) -> Result<HashMap<IPTCTag, String>, Box<dyn Error>> {
        let cursor = Cursor::new(buffer);
        let mut decoder = Decoder::new(cursor)?;

        let tag_value = decoder.get_tag(Tag::Unknown(700))?;

        // Convert the tag value to bytes
        let bytes: Vec<u8> = match tag_value {
            Value::Byte(val) => vec![val],
            Value::Ascii(vals) => vals.into(),
            Value::List(vals) => vals
                .iter()
                .map(|x| match x {
                    Value::UnsignedBig(val) => *val as u8,
                    _ => 0,
                })
                .collect(),
            _ => vec![],
        };

        println!("Bytes: {:?}", bytes);

        // Parse the XMP data
        let data = read_xmp_data(&bytes)?;
        println!("Data: {:?}", data);

        Ok(data) // Return the parsed data instead of empty HashMap
    }
}

fn read_xmp_data(data: &[u8]) -> Result<HashMap<IPTCTag, String>, Box<dyn Error>> {
    let parser = EventReader::new(Cursor::new(data));
    let mut iptc_data = HashMap::new();
    let mut current_tag: Option<IPTCTag> = None;
    let mut in_creator_seq = false;

    for event in parser {
        match event? {
            XmlEvent::StartElement { name, .. } => match name.local_name.as_str() {
                "creator" => in_creator_seq = true,
                "li" if in_creator_seq => current_tag = Some(IPTCTag::ByLine),
                "Caption" => current_tag = Some(IPTCTag::Caption),
                "Description" => current_tag = Some(IPTCTag::Caption),
                _ => {}
            },
            XmlEvent::Characters(data) => {
                if let Some(tag) = &current_tag {
                    iptc_data.insert(tag.clone(), data);
                }
            }
            XmlEvent::EndElement { name } => {
                if name.local_name == "creator" {
                    in_creator_seq = false;
                }
                current_tag = None;
            }
            _ => {}
        }
    }

    Ok(iptc_data)
}
