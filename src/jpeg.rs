use crate::reader::ReadUtils;
use crate::reader::read_iptc_data;
use crate::tags;
use crate::tags::TagsMap;
use crate::tags::{NULL_BLOCK, parse_short};

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

    pub fn write_iptc(
        buffer: &Vec<u8>,
        data: &HashMap<IPTCTag, String>,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut new_buffer = Vec::new();

        // Copy the initial JPEG marker (SOI)
        if buffer.len() < 2 || buffer[0] != 0xFF || buffer[1] != 0xD8 {
            return Err("Not a valid JPEG file".into());
        }
        new_buffer.extend_from_slice(&buffer[0..2]);
        let mut offset = 2;

        // Convert IPTC data to binary format first
        let iptc_data = Self::convert_iptc_to_binary(data)?;
        let mut found_app13 = false;
        let mut inserted_app13 = false;

        // Copy segments until we find SOS
        while offset + 1 < buffer.len() {
            // Every JPEG segment must start with 0xFF
            if buffer[offset] != 0xFF {
                offset += 1;
                continue;
            }

            let marker = buffer[offset + 1];

            // Skip empty markers
            if marker == 0xFF {
                offset += 1;
                continue;
            }

            // For markers without length field
            if marker == 0x00 || marker == 0x01 || (marker >= 0xD0 && marker <= 0xD7) {
                new_buffer.extend_from_slice(&buffer[offset..offset + 2]);
                offset += 2;
                continue;
            }

            // End of image marker
            if marker == 0xD9 {
                new_buffer.extend_from_slice(&buffer[offset..offset + 2]);
                break;
            }

            // Start of scan marker - copy the rest of the file
            if marker == 0xDA {
                // If we haven't inserted APP13 yet, do it now
                if !found_app13 && !inserted_app13 {
                    // Write APP13 marker
                    new_buffer.extend_from_slice(&[0xFF, 0xED]);
                    // Write length (including length bytes)
                    let total_length = (iptc_data.len() + 2) as u16;
                    new_buffer.push((total_length >> 8) as u8);
                    new_buffer.push(total_length as u8);
                    // Write IPTC data
                    new_buffer.extend_from_slice(&iptc_data);
                }

                // Copy SOS marker and all remaining data
                new_buffer.extend_from_slice(&buffer[offset..]);
                break;
            }

            // Check if we can read the length
            if offset + 3 >= buffer.len() {
                new_buffer.extend_from_slice(&buffer[offset..]);
                break;
            }

            let length = buffer.read_u16be(offset + 2) as usize;

            // Validate segment length
            if length < 2 || offset + 2 + length > buffer.len() {
                new_buffer.extend_from_slice(&buffer[offset..]);
                break;
            }

            // If this is APP13, replace it with our new data
            if marker == 0xED {
                found_app13 = true;
                // Write APP13 marker
                new_buffer.extend_from_slice(&[0xFF, 0xED]);
                // Write length (including length bytes)
                let total_length = (iptc_data.len() + 2) as u16;
                new_buffer.push((total_length >> 8) as u8);
                new_buffer.push(total_length as u8);
                // Write IPTC data
                new_buffer.extend_from_slice(&iptc_data);
                offset += 2 + length;
            } else {
                // If we haven't found APP13 and this is after APP0/APP1 but before other segments,
                // insert our APP13 data here
                if !found_app13 && !inserted_app13 && marker > 0xE1 {
                    // Write APP13 marker
                    new_buffer.extend_from_slice(&[0xFF, 0xED]);
                    // Write length (including length bytes)
                    let total_length = (iptc_data.len() + 2) as u16;
                    new_buffer.push((total_length >> 8) as u8);
                    new_buffer.push(total_length as u8);
                    // Write IPTC data
                    new_buffer.extend_from_slice(&iptc_data);
                    inserted_app13 = true;
                }

                // Copy the marker and its data
                new_buffer.extend_from_slice(&buffer[offset..offset + 2 + length]);
                offset += 2 + length;
            }
        }

        Ok(new_buffer)
    }

    fn convert_iptc_to_binary(data: &HashMap<IPTCTag, String>) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut binary = Vec::new();
        let tags_map = TagsMap::new();

        // Add Photoshop header
        binary.extend_from_slice(b"Photoshop 3.0\0");
        binary.push(0x00); // Pad to even length

        // Add 8BIM marker and IPTC block
        let mut iptc_block = Vec::new();

        // Sort tags by record and dataset numbers
        let mut sorted_tags: Vec<_> = data.iter().collect();
        sorted_tags.sort_by_key(|(tag, _)| {
            if let Some((record, dataset)) = Self::get_record_dataset(tag) {
                (record, dataset)
            } else {
                (0, 0) // Put unknown tags at the start
            }
        });

        // Add IPTC data in sorted order
        for (tag, value) in sorted_tags {
            if let Some((record, dataset)) = Self::get_record_dataset(tag) {
                // Field delimiter
                iptc_block.push(0x1C);

                // Record number and dataset number
                iptc_block.push(record);
                iptc_block.push(dataset);

                // Get the tag format
                let tag_key = format!("{}:{}", record, dataset);
                let (_, _, parse_fn) = tags_map.get(tag_key).unwrap_or(NULL_BLOCK);

                // Convert value based on tag format
                let value_bytes = if parse_fn as usize == parse_short as usize {
                    // For short values, convert string to u16 and then to bytes
                    let num_val = value.parse::<u16>().unwrap_or(0);
                    vec![(num_val >> 8) as u8, num_val as u8]
                } else {
                    // For regular strings, just use UTF-8 bytes
                    value.as_bytes().to_vec()
                };

                // Value length (big endian)
                let value_len = value_bytes.len() as u16;
                iptc_block.push((value_len >> 8) as u8);
                iptc_block.push(value_len as u8);

                // Value
                iptc_block.extend_from_slice(&value_bytes);
            }
        }

        // Add 8BIM marker
        binary.extend_from_slice(b"8BIM");

        // Resource ID for IPTC (0x0404)
        binary.extend_from_slice(&[0x04, 0x04]);

        // Empty name (padded to even length)
        binary.push(0x00);
        binary.push(0x00);

        // Block size (big endian)
        let block_size = iptc_block.len() as u32;
        binary.extend_from_slice(&[
            (block_size >> 24) as u8,
            (block_size >> 16) as u8,
            (block_size >> 8) as u8,
            block_size as u8,
        ]);

        // Add the IPTC block
        binary.extend_from_slice(&iptc_block);

        // Pad to even length if needed
        if binary.len() % 2 != 0 {
            binary.push(0x00);
        }

        Ok(binary)
    }

    fn get_record_dataset(tag: &IPTCTag) -> Option<(u8, u8)> {
        let tags_map = TagsMap::new();
        tags_map.get_record_dataset(tag)
    }
}
