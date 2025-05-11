use std::error::Error;
use std::fs;
use std::path::Path;

use iptc::IPTC;
use iptc::IPTCTag;

#[test]
fn test_write_iptc() -> Result<(), Box<dyn Error>> {
    // Create a copy of the test file so we don't modify the original
    let original_path = Path::new("tests/smiley.jpg");
    let test_path = Path::new("tests/smiley_write_test.jpg");
    let debug_path = Path::new("tests/smiley_debug.jpg");
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
