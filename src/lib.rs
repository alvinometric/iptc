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
use image::{ImageFormat, ImageReader};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
pub use tags::IPTCTag;

pub struct IPTC {
    pub data: HashMap<IPTCTag, String>,
}

impl IPTC {
    pub fn get_all(&self) -> HashMap<IPTCTag, String> {
        self.data.clone()
    }

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
        let format = img_reader.format().ok_or("Image format not supported")?;
        let _image = img_reader.decode()?;

        let file = File::open(image_path)?;
        let mut bufreader = BufReader::new(file);
        let mut buffer: Vec<u8> = Vec::new();
        bufreader.read_to_end(&mut buffer)?;

        let mut data = HashMap::new();

        // Check if the file is a JPEG
        if format == ImageFormat::Jpeg {
            data = JPEGReader::read_iptc(&buffer)?;
        } else if format == ImageFormat::Tiff {
            println!("TIFF file detected");
            data = TIFFReader::read_iptc(&buffer)?;
        } else {
            println!("Unsupported file, only JPEG & Tiff files are supported");
        }

        Ok(IPTC { data })
    }
}

trait ReadUtils {
    fn read_u16be(&self, offset: usize) -> u16;
    fn read_i16be(&self, offset: usize) -> i16;
}

impl ReadUtils for Vec<u8> {
    fn read_u16be(&self, offset: usize) -> u16 {
        ((self[offset] as u16) << 8) | (self[offset + 1] as u16)
    }

    fn read_i16be(&self, offset: usize) -> i16 {
        ((self[offset] as i16) << 8) | (self[offset + 1] as i16)
    }
}
