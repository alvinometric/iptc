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

        // Uncomment this once the bytes are correct
        // let data = read_xmp_data(&bytes)?;
        // println!("Data: {:?}", data);

        Ok(HashMap::new())
    }
}

fn read_xmp_data(data: &[u8]) -> Result<HashMap<IPTCTag, String>, Box<dyn Error>> {
    // Debug print the raw bytes and as string
    println!("Raw bytes: {:?}", data);
    println!("As string: {}", String::from_utf8_lossy(data));

    let parser = EventReader::new(Cursor::new(data));
    let mut iptc_data = HashMap::new();
    let mut current_tag: Option<IPTCTag> = None;

    for event in parser {
        match event? {
            XmlEvent::StartElement { name, .. } => {
                current_tag = match name.local_name.as_str() {
                    "creator" => Some(IPTCTag::ByLine),
                    "title" => Some(IPTCTag::ByLineTitle),
                    _ => None,
                };
            }
            XmlEvent::Characters(data) => {
                if let Some(tag) = &current_tag {
                    iptc_data.insert(tag.clone(), data);
                }
            }
            XmlEvent::EndElement { .. } => {
                current_tag = None;
            }
            _ => {}
        }
    }

    Ok(iptc_data)
}
