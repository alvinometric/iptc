# IPTC

[![Crates.io](https://img.shields.io/crates/v/iptc)](https://crates.io/crates/iptc)
[![CI Build](https://github.com/alvinometric/iptc/actions/workflows/rust.yml/badge.svg)](https://github.com/alvinometric/iptc/actions/workflows/rust.yml)

A fast, lightweight library to read **and write** IPTC metadata in JPEG files, written in pure Rust. Also includes partial support for TIFF files.

## Features

- 🚀 **Fast & Lightweight** - Written in pure Rust
- 📖 **Read IPTC Tags** - Extract IPTC metadata like keywords, captions, and copyright info
- ✍️ **Write IPTC Tags** - Add or modify IPTC metadata in your images
- 🔒 **Safe** - Memory-safe operations with Rust's guarantees
- 📸 **Format Support** - Full JPEG support

## Example

```rs
use iptc::IPTC;
use iptc::IPTCTag;
use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let image_path = Path::new("tests/smiley.jpg");

    // Reading IPTC metadata
    let mut iptc = IPTC::read_from_path(&image_path)?;

    // See all the tags in the image
    println!("IPTC: {:?}", iptc.get_all());

    // Get a specific tag
    let keywords = iptc.get(IPTCTag::Keywords);
    println!("keywords: {}", keywords);

    // Writing new metadata
    // For repeatable fields like Keywords, you can add multiple values
    let keywords = vec!["rust", "metadata", "iptc"];
    for keyword in keywords {
        iptc.set_tag(IPTCTag::Keywords, keyword);
    }

    // For single-value fields, just set them directly
    iptc.set_tag(IPTCTag::City, "Oslo");

    iptc.write_to_file(&image_path)?;

    Ok(())
}
```
