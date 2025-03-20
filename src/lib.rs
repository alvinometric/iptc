mod tags;
use image::ImageReader;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use tags::{IPTCTags, TagsMap};

const FIELD_DELIMITER: u8 = 28;
const TEXT_START_MARKER: u8 = 2;

pub struct IPTC {
    data: HashMap<IPTCTags, String>,
}

impl IPTC {
    pub fn get(&self, tag: IPTCTags) -> String {
        let returned_tag = self.data.get(&tag);
        if returned_tag == None {
            return String::new();
        }
        returned_tag.unwrap().clone()
    }

    pub fn read_from_path(image_path: &Path) -> Result<Self, Box<dyn Error>> {
        let file = File::open(image_path)?;
        let bufreader = BufReader::new(file);
        let img_reader = ImageReader::new(bufreader).with_guessed_format()?;
        let _image = img_reader.decode()?;

        let file = File::open(image_path)?;
        let mut bufreader = BufReader::new(file);
        let mut buffer: Vec<u8> = Vec::new();
        bufreader.read_to_end(&mut buffer)?;

        let mut offset = 0;

        // Check for JPEG magic bytes header
        if buffer[0] != 0xFF || buffer[1] != 0xD8 {
            return Err("Not a valid JPEG".into());
        }
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
                return Ok(IPTC { data: iptc_data });
            } else {
                // Add header length (2 bytes after header type) to offset
                offset += 2 + (&buffer).read_u16be(offset + 2) as usize;
            }
        }

        Ok(IPTC {
            data: HashMap::new(),
        })
    }
}

trait ReadUtils {
    fn read_u16be(&self, offset: usize) -> u16;
    fn read_i16be(&self, offset: usize) -> i16;
    fn read_i32be(&self, offset: usize) -> i32;
}

impl ReadUtils for Vec<u8> {
    fn read_u16be(&self, offset: usize) -> u16 {
        ((self[offset] as u16) << 8) | (self[offset + 1] as u16)
    }

    fn read_i16be(&self, offset: usize) -> i16 {
        ((self[offset] as i16) << 8) | (self[offset + 1] as i16)
    }

    fn read_i32be(&self, offset: usize) -> i32 {
        let b1 = self[offset] as i32;
        let b2 = self[offset + 1] as i32;
        let b3 = self[offset + 2] as i32;
        let b4 = self[offset + 3] as i32;
        println!("b1: {}, b2: {}, b3: {}, b4: {}", b1, b2, b3, b4);
        ((b1) << 24) | ((b2) << 16) | ((b3) << 8) | (b4)
    }
}

fn read_iptc_data(
    buffer: &Vec<u8>,
    start: usize,
    length: usize,
) -> Result<HashMap<IPTCTags, String>, Box<dyn Error>> {
    let mut data = HashMap::new();
    let tags_map = TagsMap::new();

    if buffer.get(start..start + 13).ok_or("Invalid slice")? != b"Photoshop 3.0" {
        return Err("Not valid Photoshop data".into());
    }

    extract_blocks(buffer, start + 13, length)?
        .iter()
        .filter(|block| block.resource_id == 1028)
        .for_each(|block| {
            println!("Block: {:?}", block);
            extract_iptc_fields_from_block(buffer, block.start_of_block, block.size_of_block)
                .iter()
                .for_each(|field| {
                    println!("Field ID: {}, Field: {:?}", field.id, field);
                    let name: IPTCTags = tags_map.get(field.id.into()).unwrap_or(IPTCTags::Null);
                    if name != IPTCTags::Null {
                        data.insert(name, field.value.clone());
                    }
                });
        });

    Ok(data)
}

#[derive(Debug)]
struct Block {
    resource_id: i16,
    name: String,
    start_of_block: usize,
    size_of_block: usize,
}

fn extract_blocks(
    buffer: &Vec<u8>,
    start: usize,
    length: usize,
) -> Result<Vec<Block>, Box<dyn Error>> {
    let mut blocks = Vec::new();
    let end = std::cmp::min(buffer.len(), start + length);

    let mut i = start;
    while i < end {
        // Signature: '8BIM'
        if buffer.get(i..i + 4).ok_or("Invalid slice")? == b"8BIM" {
            println!("Found 8BIM at {}", i);
            // Resource ID is 2 bytes, so use i16BE
            let resource_id = buffer.read_i16be(i + 4);

            // Name: Pascal string, padded to make the size even
            if i + 5 >= end {
                return Err("Invalid offset for name length".into());
            }
            let name_length = buffer[i + 5] as usize;

            let name = String::from_utf8(buffer[i + 6..i + 6 + name_length].to_vec())?;

            println!("Reading block size at i: {}", i + 6 + name_length);
            if i + 6 + name_length + 4 > end {
                return Err("Invalid offset for block size".into());
            }
            let block_size = buffer.read_i32be(i + 6 + name_length);
            if block_size < 0 {
                return Err("Negative block size".into());
            }
            let block_size = block_size as usize;

            println!(
                "i: {}, name_length: {}, block_size: {}",
                i, name_length, block_size
            );

            blocks.push(Block {
                resource_id,
                name,
                start_of_block: i + 6 + name_length + 4,
                size_of_block: block_size,
            });

            i += 6 + name_length + 4;
            i += block_size;
        } else {
            println!("Not 8BIM at {}: {:?}", i, buffer.get(i..i + 4));
            i += 1;
        }
    }
    Ok(blocks)
}

#[derive(Debug)]
struct Field {
    id: u8,
    value: String,
}

fn extract_iptc_fields_from_block(buffer: &Vec<u8>, start: usize, length: usize) -> Vec<Field> {
    let mut data = Vec::new();
    let end = std::cmp::min(buffer.len(), start + length);
    let mut i = start;

    while i < end {
        if buffer[i] == TEXT_START_MARKER {
            // Get the length by finding the next field separator
            let mut field_length = 0;
            while i + field_length < end
                && i + field_length < buffer.len()
                && buffer[i + field_length] != FIELD_DELIMITER
            {
                field_length += 1;
            }

            if field_length > 0 {
                if i + 2 < i + field_length {
                    if let Ok(value) = String::from_utf8(buffer[i + 2..i + field_length].to_vec()) {
                        let cleaned_value = value
                            .trim_start_matches(|c: char| c == '\0' || c.is_control())
                            .to_string();
                        data.push(Field {
                            id: buffer[i + 1],
                            value: cleaned_value,
                        });
                    }
                }
                i += field_length;
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    data
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_read_write_iptc() -> Result<(), Box<dyn Error>> {
        let image_path = Path::new("DSC00512.jpg");

        let iptc = IPTC::read_from_path(&image_path)?;

        let city = iptc.get(IPTCTags::City);
        assert_eq!(city, "London");

        Ok(())
    }
}
