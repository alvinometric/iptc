use iptc::IPTC;
use iptc::IPTCTag;
use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let image_path = Path::new("tests/smiley.jpg");

    let iptc = IPTC::read_from_path(&image_path)?;

    println!("IPTC: {:?}", iptc.get_all());

    let city = iptc.get(IPTCTag::City);
    let keywords = iptc.get(IPTCTag::Keywords);

    println!("city: {}", city);
    println!("keywords: {}", keywords);
    Ok(())
}
