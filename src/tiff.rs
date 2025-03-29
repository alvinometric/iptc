use crate::ReadUtils;
use crate::reader::read_iptc_data;
use crate::tags::IPTCTag;
use std::collections::HashMap;
use std::error::Error;
use std::io::Cursor;
use tiff::decoder::Decoder;

pub(crate) struct TIFFReader;

impl TIFFReader {
    pub fn read_iptc(buffer: &Vec<u8>) -> Result<HashMap<IPTCTag, String>, Box<dyn Error>> {
        let cursor = Cursor::new(buffer);
        let decoder = Decoder::new(cursor);

        Ok(HashMap::new())
    }
}
