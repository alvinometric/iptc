use crate::reader::read_iptc_data;
use crate::{ReadUtils, tags};
use std::collections::HashMap;
use std::error::Error;
use tags::IPTCTag;

pub(crate) struct JPEGReader;

impl JPEGReader {
    pub fn read_iptc(buffer: &Vec<u8>) -> Result<HashMap<IPTCTag, String>, Box<dyn Error>> {
        let mut offset = 0;
        offset += 2;

        // Loop through the file looking for the Photoshop header bytes
        while offset < buffer.len() {
            if buffer[offset] != 0xFF {
                return Err(format!(
                    "Not a valid marker at offset {}, found: {}",
                    offset, buffer[offset]
                )
                .into());
            }

            let application_marker = buffer[offset + 1];

            if application_marker == 237 {
                // This is our marker. The content length is 2 byte number.
                let iptc_data = read_iptc_data(
                    &buffer,
                    offset + 4,
                    (&buffer).read_u16be(offset + 2) as usize,
                )?;
                return Ok(iptc_data);
            } else {
                // Add header length (2 bytes after header type) to offset
                offset += 2 + (&buffer).read_u16be(offset + 2) as usize;
            }
        }

        Ok(HashMap::new())
    }
}
