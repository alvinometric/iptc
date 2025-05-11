use std::error::Error;
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
fn street_photo_example() -> Result<(), Box<dyn Error>> {
    let image_path = Path::new("tests/DSC00512.jpg");
    let iptc = IPTC::read_from_path(&image_path)?;

    let tags = iptc.get_all();
    println!("IPTC: {:?}", tags);

    let date_created = iptc.get(IPTCTag::DateCreated);
    assert_eq!(date_created, "20190519");

    let time_created = iptc.get(IPTCTag::TimeCreated);
    assert_eq!(time_created, "162932+0000");

    let keywords = iptc.get(IPTCTag::Keywords);
    assert_eq!(keywords, "London, England, Street, Night");

    let city = iptc.get(IPTCTag::City);
    assert_eq!(city, "London");

    let country = iptc.get(IPTCTag::CountryOrPrimaryLocationName);
    assert_eq!(country, "UK");

    Ok(())
}
