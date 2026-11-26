use std::error::Error;
use std::fs;
use std::path::Path;

use img_parts::Bytes;
use img_parts::jpeg::{Jpeg, JpegSegment};
use iptc::APP13_MARKER;
use iptc::IPTC;
use iptc::IPTCTag;

fn file_contains(path: &Path, text: &str) -> bool {
    let bytes = fs::read(path).unwrap();
    bytes
        .windows(text.len())
        .any(|window| window == text.as_bytes())
}

#[test]
fn keywords_survive_round_trip() -> Result<(), Box<dyn Error>> {
    let original = fs::read("tests/marine-corps.jpg")?;
    let keywords = ["party", "art", "rust"];

    let mut iptc = IPTC::new();
    for keyword in keywords {
        iptc.set_tag(IPTCTag::Keywords, keyword);
    }

    let written = iptc.write_to_buffer(&original)?;
    let read_back = IPTC::read_from_buffer(&written)?;
    assert_eq!(read_back.data[&IPTCTag::Keywords], keywords);

    let rewritten = read_back.write_to_buffer(&original)?;
    let read_again = IPTC::read_from_buffer(&rewritten)?;
    assert_eq!(read_again.data[&IPTCTag::Keywords], keywords);

    Ok(())
}

#[test]
fn saving_without_changes_does_not_grow_file() -> Result<(), Box<dyn Error>> {
    let original = fs::read("tests/DSC00512.jpg")?;
    let iptc = IPTC::read_from_buffer(&original)?;

    let saved = iptc.write_to_buffer(&original)?;

    assert!(saved.len() <= original.len());
    Ok(())
}

/// Get all the JPEG segments, exif, etc
fn get_non_iptc_segments(bytes: Vec<u8>) -> Vec<JpegSegment> {
    let jpeg = Jpeg::from_bytes(Bytes::from(bytes)).unwrap();
    jpeg.segments()
        .iter()
        .filter(|segment| segment.marker() != APP13_MARKER)
        .cloned()
        .collect()
}

#[test]
fn writing_preserves_other_segments() -> Result<(), Box<dyn Error>> {
    // DSC00563 has EXIF, an XMP rating and IPTC keywords
    let original = fs::read("tests/DSC00563.jpg")?;
    let mut iptc = IPTC::read_from_buffer(&original)?;
    iptc.set_tag(IPTCTag::Keywords, "street photography");
    let written = iptc.write_to_buffer(&original)?;

    assert_eq!(
        get_non_iptc_segments(original),
        get_non_iptc_segments(written)
    );
    Ok(())
}

#[test]
fn test_utf8_characters() -> Result<(), Box<dyn Error>> {
    let original = fs::read("tests/starry-night.jpg")?;

    let mut iptc = IPTC::new();
    iptc.set_tag(IPTCTag::City, "Vézelay");
    let written = iptc.write_to_buffer(&original)?;

    let read_back = IPTC::read_from_buffer(&written)?;
    // "ESC % G" means characters are encoded as UTF-8
    let utf8_marker = format!("{}%G", char::from(27)); // 27 = ESC
    assert_eq!(read_back.get(IPTCTag::CodedCharacterSet), utf8_marker);

    Ok(())
}

#[test]
fn test_write_iptc() -> Result<(), Box<dyn Error>> {
    // Create a copy of the test file so we don't modify the original
    // https://www.flickr.com/photos/marine_corps/13535839043/ (United States government work)
    let original_path = Path::new("tests/marine-corps.jpg");
    let test_path = Path::new("tests/marine-corps_write_test.jpg");
    let debug_path = Path::new("tests/marine-corps_debug.jpg");
    fs::copy(original_path, test_path)?;

    // Read the original IPTC data
    let mut iptc = IPTC::read_from_path(&test_path)?;

    // Modify some tags, last operation counts
    iptc.set_tag(IPTCTag::City, "Bergen");
    iptc.set_tag(IPTCTag::City, "Oslo");

    // Set multiple keywords as separate entries
    let keywords = vec!["rust", "metadata", "testing", "iptc"];
    for keyword in keywords.iter() {
        iptc.set_tag(IPTCTag::Keywords, keyword);
    }

    // Write the changes
    iptc.write_to_file(&test_path)?;

    // Make a copy for debugging
    fs::copy(test_path, debug_path)?;

    // Read back and verify
    let new_iptc = IPTC::read_from_path(&test_path)?;
    assert_eq!(new_iptc.get(IPTCTag::City), "Oslo");

    // Flickr keeps the link in metadata
    assert!(file_contains(
        test_path,
        "https://flickr.com/e/Js4hZ%2BfZvO6Xk0y3YZvxWLOOlBaGCVwNTnN3w9Zh8jw%3D"
    ));

    // Verify all keywords are present
    let read_keywords = new_iptc.get(IPTCTag::Keywords);
    for keyword in keywords {
        assert!(
            read_keywords.contains(keyword),
            "Missing keyword: {}",
            keyword
        );
    }

    // Clean up
    fs::remove_file(test_path)?;
    fs::remove_file(debug_path)?;

    Ok(())
}
