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
//!     // Reading IPTC metadata from file
//!     let mut iptc = IPTC::read_from_path(&image_path)?;
//!
//!     // Alternatively, you can read from a buffer
//!     // let buffer = std::fs::read(&image_path)?;
//!     // let mut iptc = IPTC::read_from_buffer(&buffer)?;
//!
//!     // See all the tags in the image
//!     println!("IPTC: {:?}", iptc.get_all());
//!
//!     // Get a specific tag
//!     let keywords = iptc.get(IPTCTag::Keywords);
//!     println!("keywords: {}", keywords);
//!
//!     // Writing new metadata
//!     // For repeatable fields like Keywords, you can add multiple values
//!     let keywords = vec!["rust", "metadata", "iptc"];
//!     for keyword in keywords {
//!         iptc.set_tag(IPTCTag::Keywords, keyword);
//!     }
//!
//!     // For single-value fields, just set them directly
//!     iptc.set_tag(IPTCTag::City, "Oslo");
//!
//!     iptc.write_to_file(&image_path)?;
//!
//!     // Alternatively, you can write to a buffer
//!     // let buffer = std::fs::read(&image_path)?;
//!     // let updated_buffer = iptc.write_to_buffer(&buffer)?;
//!     // std::fs::write("output.jpg", updated_buffer)?;
//!
//!     Ok(())
//! }
//! ```

mod jpeg;
use jpeg::JPEGReader;
mod tiff;
use tiff::TIFFReader;
mod reader;
use reader::Rest;
mod tags;
use tags::TagsMap;
use image::ImageFormat;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
pub use tags::IPTCTag;

#[derive(Default)]
pub struct IPTC {
    pub data: HashMap<IPTCTag, Vec<String>>,
    rest: Rest,
}

impl IPTC {
    /// Creates an empty IPTC metadata collection.
    pub fn new() -> Self {
        Self::default()
    }

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

    /// Adds a value to a repeatable tag like Keywords, or replaces the value of a single-value tag like City.
    pub fn set_tag(&mut self, tag: IPTCTag, value: &str) {
        let repeatable = TagsMap::new().find(&tag).is_some_and(|entry| entry.1);
        if !repeatable {
            self.data.insert(tag, vec![value.to_string()]);
            return;
        }
        let values = self.data.entry(tag).or_default();
        if !values.contains(&value.to_string()) {
            values.push(value.to_string());
        }
    }

    /// Produces a new JPEG image buffer augmented with IPTC metadata.
    pub fn write_to_buffer(&self, image_buffer: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let format = image::guess_format(image_buffer)?;

        if format != ImageFormat::Jpeg {
            return Err("Writing IPTC data is only supported for JPEG files".into());
        }

        JPEGReader::write_iptc(image_buffer, &self.data, &self.rest)
    }

    /// Writes IPTC metadata to a JPEG file.
    pub fn write_to_file(&self, image_path: &Path) -> Result<(), Box<dyn Error>> {
        let buffer = std::fs::read(image_path)?;
        let new_buffer = self.write_to_buffer(&buffer)?;

        std::fs::write(image_path, new_buffer)?;
        Ok(())
    }

    /// Reads IPTC metadata from a buffer containing a JPEG or TIFF image.
    pub fn read_from_buffer(image_buffer: &[u8]) -> Result<Self, Box<dyn Error>> {
        let format = image::guess_format(image_buffer)?;

        let mut array_data = HashMap::new();
        let mut rest = Rest::default();

        // Check if the file is a JPEG
        if format == ImageFormat::Jpeg {
            (array_data, rest) = JPEGReader::read_iptc(image_buffer)?;
        } else if format == ImageFormat::Tiff {
            println!("TIFF file detected, not all tags are supported");
            array_data = TIFFReader::read_iptc(image_buffer)?
                .into_iter()
                .map(|(k, v)| (k, vec![v]))
                .collect();
        } else {
            println!("Unsupported file, only JPEG & Tiff files are supported");
        }

        Ok(IPTC {
            data: array_data,
            rest,
        })
    }

    /// Reads IPTC metadata from a JPEG or TIFF file.
    pub fn read_from_path(image_path: &Path) -> Result<Self, Box<dyn Error>> {
        let buffer = std::fs::read(image_path)?;
        Self::read_from_buffer(&buffer)
    }
}
