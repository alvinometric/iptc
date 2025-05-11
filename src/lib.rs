//! Library to read IPTC tags from JPEG files, in pure Rust.
//!
//! # Example
//! ```rust,no_run
//! use iptc::IPTC;
//! use iptc::IPTCTag;
//! use std::error::Error;
//! use std::path::Path;

//! fn main() -> Result<(), Box<dyn Error>> {
//!     let image_path = Path::new("tests/smiley.jpg");
//!
//!     let iptc = IPTC::read_from_path(&image_path)?;
//!
//!     // See all the tags in the image
//!     println!("IPTC: {:?}", iptc.get_all());
//!
//!     // Get a specific tag
//!     let keywords = iptc.get(IPTCTag::Keywords);
//!     println!("keywords: {}", keywords);
//!
//!     Ok(())
//! }
//! ```

mod jpeg;
use jpeg::JPEGReader;
mod tiff;
use tiff::TIFFReader;
mod reader;
mod tags;
use image::ImageFormat;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
pub use tags::IPTCTag;

pub struct IPTC {
    pub data: HashMap<IPTCTag, Vec<String>>,
}

impl IPTC {
    pub fn get_all(&self) -> HashMap<IPTCTag, Vec<String>> {
        self.data.clone()
    }

    pub fn get(&self, tag: IPTCTag) -> String {
        let returned_tag = self.data.get(&tag);
        if returned_tag == None {
            return String::new();
        }
        returned_tag.unwrap().join(", ")
    }

    pub fn set_tag(&mut self, tag: IPTCTag, value: &str) {
        if let Some(values) = self.data.get_mut(&tag) {
            // For repeatable fields, add to the vector if not already present
            if !values.contains(&value.to_string()) {
                values.push(value.to_string());
            }
        } else {
            // For new fields, start a new vector
            self.data.insert(tag, vec![value.to_string()]);
        }
    }

    pub fn write_to_file(&self, image_path: &Path) -> Result<(), Box<dyn Error>> {
        let buffer = std::fs::read(image_path)?;
        let format = image::guess_format(&buffer)?;

        let new_buffer = if format == ImageFormat::Jpeg {
            JPEGReader::write_iptc(&buffer, &self.data)?
        } else {
            return Err("Writing IPTC data is only supported for JPEG files".into());
        };

        std::fs::write(image_path, new_buffer)?;
        Ok(())
    }

    pub fn read_from_path(image_path: &Path) -> Result<Self, Box<dyn Error>> {
        let buffer = std::fs::read(image_path)?;
        let format = image::guess_format(&buffer)?;

        let mut string_data = HashMap::new();

        // Check if the file is a JPEG
        if format == ImageFormat::Jpeg {
            string_data = JPEGReader::read_iptc(&buffer)?;
        } else if format == ImageFormat::Tiff {
            println!("TIFF file detected, not all tags are supported");
            string_data = TIFFReader::read_iptc(&buffer)?;
        } else {
            println!("Unsupported file, only JPEG & Tiff files are supported");
        }

        // Convert String to Vec<String>
        let data = string_data.into_iter().map(|(k, v)| (k, vec![v])).collect();

        Ok(IPTC { data })
    }
}
