# IPTC

[![Crates.io](https://img.shields.io/crates/v/iptc)](https://crates.io/crates/iptc)
[![CI Build](https://github.com/alvinometric/iptc/actions/workflows/rust.yml/badge.svg)](https://github.com/alvinometric/iptc/actions/workflows/rust.yml)

Read IPTC tags from JPEG files in pure Rust, with partial support for Tiff files.

## Example

```rs
use iptc::IPTC;
use iptc::IPTCTag;
use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let image_path = Path::new("tests/smiley.jpg");

    let iptc = IPTC::read_from_path(&image_path)?;

    // See all the tags in the image
    println!("IPTC: {:?}", iptc.get_all());

    // Get a specific tag
    let keywords = iptc.get(IPTCTag::Keywords);
    println!("keywords: {}", keywords);

    Ok(())
}
```
