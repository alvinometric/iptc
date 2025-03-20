use iptc::IPTC;
use iptc::IPTCTags;
use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello from the example!");
    let image_path = Path::new("DSC00512.jpg");

    let iptc = IPTC::read_from_path(&image_path)?;

    println!("IPTC: {:?}", iptc.data);

    let city = iptc.get(IPTCTags::City);
    let keywords = iptc.get(IPTCTags::Keywords);
    println!("city: {}", city);
    println!("keywords: {}", keywords);
    Ok(())
}
