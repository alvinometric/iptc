use iptc::IPTC;
use iptc::IPTCTag;
use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let image_path = Path::new("DSC00512.jpg");

    let iptc = IPTC::read_from_path(&image_path)?;

    println!("-------------------");

    println!("IPTC: {:?}", iptc.data);

    let city = iptc.get(IPTCTag::City);
    let keywords = iptc.get(IPTCTag::Keywords);

    println!("city: {}", city);
    println!("keywords: {}", keywords);
    println!(
        "ApplicationRecordVersion: {}",
        iptc.get(IPTCTag::ApplicationRecordVersion)
    );
    Ok(())
}
