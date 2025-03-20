mod tags;
use image::{ImageFormat, io::Reader as ImageReader};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use tags::IptcTags;

use lazy_static::lazy_static;

const FIELD_DELIMITER: u8 = 28;
const TEXT_START_MARKER: u8 = 2;

lazy_static! {
    static ref TAGS: IptcTags = IptcTags {
        object_type_reference: String::from("OBJECT_TYPE_REFERENCE"),
        object_attribute_reference: String::from("OBJECT_ATTRIBUTE_REFERENCE"),
        object_name: String::from("OBJECT_NAME"),
        edit_status: String::from("EDIT_STATUS"),
        editorial_update: String::from("EDITORIAL_UPDATE"),
        urgency: String::from("URGENCY"),
        subject_reference: String::from("SUBJECT_REFERENCE"),
        category: String::from("CATEGORY"),
        supplemental_categories: String::from("SUPPLEMENTAL_CATEGORIES"),
        fixture_id: String::from("FIXTURE_ID"),
        keywords: String::from("KEYWORDS"),
        content_location_code: String::from("CONTENT_LOCATION_CODE"),
        content_location_name: String::from("CONTENT_LOCATION_NAME"),
        release_date: String::from("RELEASE_DATE"),
        release_time: String::from("RELEASE_TIME"),
        expiration_date: String::from("EXPIRATION_DATE"),
        expiration_time: String::from("EXPIRATION_TIME"),
        special_instructions: String::from("SPECIAL_INSTRUCTIONS"),
        action_advised: String::from("ACTION_ADVISED"),
        reference_service: String::from("REFERENCE_SERVICE"),
        reference_date: String::from("REFERENCE_DATE"),
        reference_number: String::from("REFERENCE_NUMBER"),
        date_created: String::from("DATE_CREATED"),
        time_created: String::from("TIME_CREATED"),
        digital_date_created: String::from("DIGITAL_DATE_CREATED"),
        digital_time_created: String::from("DIGITAL_TIME_CREATED"),
        originating_program: String::from("ORIGINATING_PROGRAM"),
        program_version: String::from("PROGRAM_VERSION"),
        object_cycle: String::from("OBJECT_CYCLE"),
        by_line: String::from("BY_LINE"),
        caption: String::from("CAPTION"),
        by_line_title: String::from("BY_LINE_TITLE"),
        city: String::from("CITY"),
        sub_location: String::from("SUB_LOCATION"),
        province_or_state: String::from("PROVINCE_OR_STATE"),
        country_or_primary_location_code: String::from("COUNTRY_OR_PRIMARY_LOCATION_CODE"),
        country_or_primary_location_name: String::from("COUNTRY_OR_PRIMARY_LOCATION_NAME"),
        original_transmission_reference: String::from("ORIGINAL_TRANSMISSION_REFERENCE"),
        headline: String::from("HEADLINE"),
        credit: String::from("CREDIT"),
        source: String::from("SOURCE"),
        copyright_notice: String::from("COPYRIGHT_NOTICE"),
        contact: String::from("CONTACT"),
        local_caption: String::from("LOCAL_CAPTION"),
        caption_writer: String::from("CAPTION_WRITER"),
        rasterized_caption: String::from("RASTERIZED_CAPTION"),
        image_type: String::from("IMAGE_TYPE"),
        image_orientation: String::from("IMAGE_ORIENTATION"),
        language_identifier: String::from("LANGUAGE_IDENTIFIER"),
        audio_type: String::from("AUDIO_TYPE"),
        audio_sampling_rate: String::from("AUDIO_SAMPLING_RATE"),
        audio_sampling_resolution: String::from("AUDIO_SAMPLING_RESOLUTION"),
        audio_duration: String::from("AUDIO_DURATION"),
        audio_outcue: String::from("AUDIO_OUTCUE"),
        job_id: String::from("JOB_ID"),
        master_document_id: String::from("MASTER_DOCUMENT_ID"),
        short_document_id: String::from("SHORT_DOCUMENT_ID"),
        unique_document_id: String::from("UNIQUE_DOCUMENT_ID"),
        owner_id: String::from("OWNER_ID"),
        object_preview_file_format: String::from("OBJECT_PREVIEW_FILE_FORMAT"),
        object_preview_file_format_version: String::from("OBJECT_PREVIEW_FILE_FORMAT_VERSION"),
        object_preview_data: String::from("OBJECT_PREVIEW_DATA"),
    };
}

pub struct IPTC {
    data: HashMap<String, String>,
}

impl IPTC {
    pub const CITY: &'static str = "CITY";

    pub fn get_data(&self) -> HashMap<String, String> {
        self.data.clone()
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
) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut data = HashMap::new();

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
                    println!("Field: {:?}", field);
                    let name = match field.id {
                        3 => &TAGS.object_type_reference,
                        4 => &TAGS.object_attribute_reference,
                        5 => &TAGS.object_name,
                        7 => &TAGS.edit_status,
                        8 => &TAGS.editorial_update,
                        10 => &TAGS.urgency,
                        12 => &TAGS.subject_reference,
                        15 => &TAGS.category,
                        20 => &TAGS.supplemental_categories,
                        22 => &TAGS.fixture_id,
                        25 => &TAGS.keywords,
                        26 => &TAGS.content_location_code,
                        27 => &TAGS.content_location_name,
                        30 => &TAGS.release_date,
                        35 => &TAGS.release_time,
                        37 => &TAGS.expiration_date,
                        38 => &TAGS.expiration_time,
                        40 => &TAGS.special_instructions,
                        42 => &TAGS.action_advised,
                        45 => &TAGS.reference_service,
                        47 => &TAGS.reference_date,
                        50 => &TAGS.reference_number,
                        55 => &TAGS.date_created,
                        60 => &TAGS.time_created,
                        62 => &TAGS.digital_date_created,
                        63 => &TAGS.digital_time_created,
                        65 => &TAGS.originating_program,
                        70 => &TAGS.program_version,
                        75 => &TAGS.object_cycle,
                        80 => &TAGS.by_line,
                        84 => &TAGS.caption,
                        85 => &TAGS.by_line_title,
                        90 => &TAGS.city,
                        92 => &TAGS.sub_location,
                        95 => &TAGS.province_or_state,
                        100 => &TAGS.country_or_primary_location_code,
                        101 => &TAGS.country_or_primary_location_name,
                        103 => &TAGS.original_transmission_reference,
                        105 => &TAGS.headline,
                        110 => &TAGS.credit,
                        115 => &TAGS.source,
                        116 => &TAGS.copyright_notice,
                        118 => &TAGS.contact,
                        120 => &TAGS.caption,
                        121 => &TAGS.local_caption,
                        122 => &TAGS.caption_writer,
                        125 => &TAGS.rasterized_caption,
                        130 => &TAGS.image_type,
                        131 => &TAGS.image_orientation,
                        135 => &TAGS.language_identifier,
                        150 => &TAGS.audio_type,
                        151 => &TAGS.audio_sampling_rate,
                        152 => &TAGS.audio_sampling_resolution,
                        153 => &TAGS.audio_duration,
                        154 => &TAGS.audio_outcue,
                        184 => &TAGS.job_id,
                        185 => &TAGS.master_document_id,
                        186 => &TAGS.short_document_id,
                        187 => &TAGS.unique_document_id,
                        188 => &TAGS.owner_id,
                        200 => &TAGS.object_preview_file_format,
                        201 => &TAGS.object_preview_file_format_version,
                        202 => &TAGS.object_preview_data,
                        _ => "",
                    };
                    if name != "" {
                        data.insert(name.to_string(), field.value.clone());
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
        let tags = iptc.get_data();

        let city = tags.get(IPTC::CITY).unwrap();
        assert_eq!(city, "London");

        Ok(())
    }
}
