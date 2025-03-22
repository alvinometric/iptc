mod tags;
use image::ImageReader;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
pub use tags::IPTCTag;
use tags::{NULL_BLOCK, TagsMap};

const FIELD_DELIMITER: u8 = 0x1c;

pub struct IPTC {
    pub data: HashMap<IPTCTag, String>,
}

impl IPTC {
    pub fn get(&self, tag: IPTCTag) -> String {
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
) -> Result<HashMap<IPTCTag, String>, Box<dyn Error>> {
    let mut data: HashMap<IPTCTag, String> = HashMap::new();
    let tags_map = TagsMap::new();

    if buffer.get(start..start + 13).ok_or("Invalid slice")? != b"Photoshop 3.0" {
        return Err("Not valid Photoshop data".into());
    }

    extract_blocks(buffer, start + 13, length)?
        .iter()
        .filter(|block| block.resource_id == 1028)
        .for_each(|block| {
            println!("Block: {:?}", block);
            let fields =
                extract_iptc_fields_from_block(buffer, block.start_of_block, block.size_of_block);
            for field in fields {
                let tag_key = ((field.record_number as u32) << 16) | (field.dataset_number as u32);
                println!("Field ID: {}, Field: {:?}", tag_key, field);
                let (name, repeatable, parse) = tags_map.get(tag_key).unwrap_or(NULL_BLOCK);

                if name != IPTCTag::Null {
                    if repeatable {
                        let value = field.value.trim();
                        if !value.is_empty() {
                            if let Some(existing_value) = data.get_mut(&name) {
                                if !existing_value
                                    .to_lowercase()
                                    .contains(&value.to_lowercase())
                                {
                                    if !existing_value.is_empty() {
                                        existing_value.push_str(", ");
                                    }
                                    existing_value.push_str(value);
                                }
                            } else {
                                let parsed_value = parse(field.value);
                                data.insert(name, parsed_value);
                            }
                        }
                    } else if field.value.trim().len() > 0 {
                        let parsed_value = parse(field.value);
                        data.insert(name, parsed_value);
                    }
                }
            }
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
            if i + 6 + name_length + 2 > end {
                return Err("Invalid offset for block size".into());
            }
            let block_size = buffer.read_u16be(i + 6 + name_length) as usize;

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
    record_number: u8,
    dataset_number: u8,
    value: String,
}

fn extract_iptc_fields_from_block(buffer: &Vec<u8>, start: usize, length: usize) -> Vec<Field> {
    let mut data: Vec<Field> = Vec::new();
    let end = std::cmp::min(buffer.len(), start + length);
    let mut i = start;

    while i < end {
        if buffer[i] == FIELD_DELIMITER {
            let value_length = buffer.read_u16be(i + 3) as usize;
            let record_number = buffer[i + 1];
            let dataset_number = buffer[i + 2];

            println!(
                "Field at i: {}, length: {}, record_number: {}, dataset_number: {}",
                i, value_length, record_number, dataset_number
            );
            if i + 5 + value_length <= end {
                if let Ok(value) = String::from_utf8(buffer[i + 5..i + 5 + value_length].to_vec()) {
                    let cleaned_value = value
                        .trim_start_matches(|c: char| c == '\0' || c.is_control())
                        .trim_end_matches(|c: char| c == '\0' || c.is_control())
                        .replace(|c: char| !c.is_ascii(), "");
                    data.push(Field {
                        record_number,
                        dataset_number,
                        value: cleaned_value.to_string(),
                    });
                }
            }
            i += 5 + value_length;
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

        let city = iptc.get(IPTCTag::City);
        assert_eq!(city, "London");

        Ok(())
    }
}
