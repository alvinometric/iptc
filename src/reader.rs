use crate::tags;
use std::collections::HashMap;
use std::error::Error;
use tags::IPTCTag;
use tags::{NULL_BLOCK, TagsMap};

const FIELD_DELIMITER: u8 = 0x1c;

pub trait ReadUtils {
    fn read_u16be(&self, offset: usize) -> u16;
    fn read_i16be(&self, offset: usize) -> i16;
}

impl ReadUtils for [u8] {
    fn read_u16be(&self, offset: usize) -> u16 {
        ((self[offset] as u16) << 8) | (self[offset + 1] as u16)
    }

    fn read_i16be(&self, offset: usize) -> i16 {
        ((self[offset] as i16) << 8) | (self[offset + 1] as i16)
    }
}

#[derive(Debug)]
struct Block {
    resource_id: i16,
    #[allow(unused)]
    name: String,
    start_of_block: usize,
    size_of_block: usize,
}

#[derive(Debug)]
struct Field {
    record_number: u8,
    dataset_number: u8,
    value: String,
}

fn extract_iptc_fields_from_block(buffer: &[u8], start: usize, length: usize) -> Vec<Field> {
    let mut data: Vec<Field> = Vec::new();
    let end = std::cmp::min(buffer.len(), start + length);
    let mut i = start;

    while i < end {
        if buffer[i] == FIELD_DELIMITER {
            let value_length = buffer.read_u16be(i + 3) as usize;
            let record_number = buffer[i + 1];
            let dataset_number = buffer[i + 2];

            // println!(
            //     "Field at i: {}, length: {}, record_number: {}, dataset_number: {}",
            //     i, value_length, record_number, dataset_number
            // );
            if i + 5 + value_length <= end {
                let raw_bytes = &buffer[i + 5..i + 5 + value_length];
                let value = raw_bytes
                    .iter()
                    .map(|b| b.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                data.push(Field {
                    record_number,
                    dataset_number,
                    value,
                });
            }
            i += 5 + value_length;
        } else {
            i += 1;
        }
    }
    data
}

pub(crate) fn read_iptc_data(
    buffer: &[u8],
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
            // println!("Block: {:?}", block);
            let fields =
                extract_iptc_fields_from_block(buffer, block.start_of_block, block.size_of_block);
            for field in fields {
                let record_number = field.record_number;
                let dataset_number = field.dataset_number;

                let tag_key = format!("{}:{}", record_number, dataset_number);

                // println!("Field ID: {}, Field: {:?}", tag_key, field);
                let (name, repeatable, parse) = tags_map.get(tag_key).unwrap_or(NULL_BLOCK);

                if name != IPTCTag::Null {
                    let parsed_value = parse(field.value);
                    if !parsed_value.trim().is_empty() {
                        if repeatable {
                            if let Some(existing_value) = data.get_mut(&name) {
                                if !existing_value
                                    .to_lowercase()
                                    .contains(&parsed_value.to_lowercase())
                                {
                                    if !existing_value.is_empty() {
                                        existing_value.push_str(", ");
                                    }
                                    existing_value.push_str(&parsed_value);
                                }
                            } else {
                                data.insert(name, parsed_value);
                            }
                        } else {
                            data.insert(name, parsed_value);
                        }
                    }
                }
            }
        });

    Ok(data)
}

fn extract_blocks(
    buffer: &[u8],
    start: usize,
    length: usize,
) -> Result<Vec<Block>, Box<dyn Error>> {
    let mut blocks = Vec::new();
    let end = std::cmp::min(buffer.len(), start + length);

    let mut i = start;
    while i < end {
        // Signature: '8BIM'
        if buffer.get(i..i + 4).ok_or("Invalid slice")? == b"8BIM" {
            // println!("Found 8BIM at {}", i);
            // Resource ID is 2 bytes, so use i16BE
            let resource_id = buffer.read_i16be(i + 4);

            // Name: Pascal string, padded to make the size even
            if i + 5 >= end {
                return Err("Invalid offset for name length".into());
            }
            let name_length = buffer[i + 5] as usize;

            let name = String::from_utf8(buffer[i + 6..i + 6 + name_length].to_vec())?;

            // println!("Reading block size at i: {}", i + 6 + name_length);

            if i + 6 + name_length + 2 > end {
                return Err("Invalid offset for block size".into());
            }
            let block_size = buffer.read_u16be(i + 6 + name_length) as usize;

            // println!(
            //     "i: {}, name: {}, name_length: {}, block_size: {}",
            //     i, name, name_length, block_size
            // );

            blocks.push(Block {
                resource_id,
                name,
                start_of_block: i + 6 + name_length + 4,
                size_of_block: block_size,
            });

            i += 6 + name_length + 4;
            i += block_size;
        } else {
            // println!("Not 8BIM at {}: {:?}", i, buffer.get(i..i + 4));
            i += 1;
        }
    }
    Ok(blocks)
}
