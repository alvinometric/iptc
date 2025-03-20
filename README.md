# IPTC

WIP, obviously

## Example

```rs
use iptc::IPTC;
use std::path::Path;

let image_path = Path::new("image.png");
let mut tags = IPTC::read_from_path(&image_path);

let city = tags.get(IPTC::City)
        assert_eq!(city, "London");

tags.set_city("Oslo")
    .set_country("Norway")
    .set_keywords(vec!["keyword1", "keyword2"]);

tags.write_to_file(&image_path)?;
```
