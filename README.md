# IPTC

[![Crates.io](https://img.shields.io/crates/v/iptc)](https://crates.io/crates/iptc)

Read IPTC tags from JPEG files, in pure Rust.

## Example

```rs
use iptc::IPTC;
use std::path::Path;

let image_path = Path::new("image.png");
let mut tags = IPTC::read_from_path(&image_path);

let city = tags.get(IPTCTag::City)

assert_eq!(city, "London");
```
