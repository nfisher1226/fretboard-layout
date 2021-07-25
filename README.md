# Fretboard Layout
## Introduction
This library takes a set of specifications for a musical instrument fretboard
and generates an SVG image suitable for use as a template in the design of an
instrument. Currently, measurements are expected to be in millimeters.
```rust
use fretboard_layout::Specs;
use svg::Document;

fn main() -> Document {
    Specs::default().create_document(None)
}
```
