use std::error::Error;
use std::fs;
use std::path::Path;

use iptc::IPTC;
use iptc::IPTCTag;

fn file_contains(path: &Path, text: &str) -> bool {
    let bytes = fs::read(path).unwrap();
    bytes.windows(text.len()).any(|window| window == text.as_bytes())
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

    // Modify some tags
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
