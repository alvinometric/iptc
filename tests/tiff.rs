use std::error::Error;
use std::path::Path;

use iptc::IPTC;
use iptc::IPTCTag;

#[test]
fn tiff_test() -> Result<(), Box<dyn Error>> {
    // Tiff files should work too
    let image_path = Path::new("tests/DSC3003.tif");
    let iptc = IPTC::read_from_path(&image_path)?;

    let province_or_state = iptc.get(IPTCTag::ProvinceOrState);
    assert_eq!(province_or_state, "Ontario");

    Ok(())
}
