use std::error::Error;
use std::fs;
use std::path::Path;

use iptc::IPTC;
use iptc::IPTCTag;

#[test]
fn exiv2_iptc_example() -> Result<(), Box<dyn Error>> {
    // https://exiv2.org/examples.html
    // Example 3: iptcprint.cpp

    let image_path = Path::new("tests/smiley.jpg");
    let iptc = IPTC::read_from_path(&image_path)?;

    let tags = iptc.get_all();
    println!("IPTC: {:?}", tags);

    let keywords = iptc.get(IPTCTag::Keywords);
    assert_eq!(keywords, "Yet another keyword");

    let date_created = iptc.get(IPTCTag::DateCreated);
    assert_eq!(date_created, "20040803");

    let urgency = iptc.get(IPTCTag::Urgency);
    assert_eq!(urgency, "very!");

    let model_version = iptc.get(IPTCTag::ModelVersion);
    assert_eq!(model_version, "42");

    let time_sent = iptc.get(IPTCTag::TimeSent);
    assert_eq!(time_sent, "144100-0500");

    Ok(())
}

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
    iptc.set_tag(IPTCTag::Keywords, "New keyword");

    // Write the changes
    iptc.write_to_file(&test_path)?;

    // Make a copy for debugging
    fs::copy(test_path, debug_path)?;

    // Read back and verify
    let new_iptc = IPTC::read_from_path(&test_path)?;
    assert_eq!(new_iptc.get(IPTCTag::City), "Oslo");
    assert_eq!(new_iptc.get(IPTCTag::Keywords), "New keyword");

    // Clean up
    fs::remove_file(test_path)?;
    fs::remove_file(debug_path)?;

    Ok(())
}
