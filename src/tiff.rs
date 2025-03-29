use crate::{ReadUtils, tags};
use std::collections::HashMap;
use std::error::Error;
use std::io::Cursor;
use tags::IPTCTag;
use tiff::decoder::{Decoder, DecodingResult};
use xml::reader::{EventReader, XmlEvent};

pub(crate) struct TIFFReader;

impl TIFFReader {
    pub fn read_iptc(buffer: &Vec<u8>) -> Result<HashMap<IPTCTag, String>, Box<dyn Error>> {
        let cursor = Cursor::new(buffer);
        let mut decoder = Decoder::new(cursor)?;

        while let Ok(Some(field)) = decoder.next_field() {
            if field.tag == 700 {
                if let DecodingResult::U8(data) = field.data {
                    return read_xmp_data(&data);
                }
            }
        }

        Ok(HashMap::new())
    }
}

fn read_xmp_data(data: &[u8]) -> Result<HashMap<IPTCTag, String>, Box<dyn Error>> {
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
